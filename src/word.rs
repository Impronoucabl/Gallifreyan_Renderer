use std::f64::consts::PI;
use std::rc::{Rc, Weak};

use svg::node::element::path::Data;
use svg::Document;
use svg::node::element::{Circle, Path};

use crate::ctx::Context;
use crate::pord::{Cartesian, POrd, PordOrCord};
use crate::utils;
use crate::StemType;

#[derive(Debug, Clone)]
pub struct LetterArc {
    pord: Rc<PordOrCord>,
    radius:f64,
    stem_type:StemType,
    ctx:Option<Context>,
}
#[derive(Debug, Clone)]
pub struct Word {
    name:String,
    pord:Rc<PordOrCord>,
    radius:f64,
    arcs: Vec<LetterArc>,
    default_ctx:Context,
    path_circle: bool,
}

impl Word {
    pub fn new(name:&str, pord:Rc<PordOrCord>, radius:f64,ctx:Context) -> Word {
        Word { 
            name: name.to_string(), 
            pord, 
            radius, 
            arcs: Vec::new(), 
            default_ctx: ctx,
            path_circle:false,
        }
    }
    pub fn ctx(&self) -> Context {
        self.default_ctx.clone()
    }
    fn new_letter(&mut self, pord:Rc<PordOrCord>,radius:f64,stem_type:StemType,ctx:Option<Context>) -> Weak<PordOrCord> {
        let letter = LetterArc::new(pord.clone(),radius,stem_type,ctx);
        self.arcs.push(letter);
        if stem_type == StemType::S || stem_type == StemType::B {
            self.path_circle = true;
        }
        Rc::downgrade(&pord)
    }
    pub fn new_letter_from_data(&mut self, r:f64,theta:f64,radius:f64,stem_type:StemType,ctx:Option<Context>) -> Rc<PordOrCord> {
        let dist = if stem_type == StemType::S {
            r + self.ctx().stroke().strokewidth()/2.0
        } else {r};
        let location = Rc::new(PordOrCord::Pord(POrd::new(dist,theta,self.pord.clone())));
        self.new_letter(location.clone(), radius, stem_type, ctx);
        location
    }
    pub fn new_letter_with_attach(&mut self, r:f64,theta:f64,radius:f64,stem_type:StemType,ctx:Option<Context>, num_of_attach:usize) -> (Rc<PordOrCord>,Vec<POrd>) {
        let letter_pord = self.new_letter_from_data(r, theta, radius, stem_type, ctx);
        let result = utils::generate_pord_vector(num_of_attach,letter_pord.clone(),radius);
        (letter_pord,result)
    }
    pub fn new_letter_from_pordorcord(&mut self,pord:Rc<PordOrCord>, radius:f64, stem_type:StemType,ctx:Option<Context>, num_of_attach:usize) -> Vec<POrd> {
        self.new_letter(pord.clone(), radius, stem_type, ctx);
        utils::generate_pord_vector(num_of_attach,pord.clone(),radius)
    }
    pub fn new_letter_from_pord(&mut self, pord:POrd,radius:f64,stem_type:StemType, ctx:Option<Context>, num_of_attach:usize) -> Vec<POrd> {
        let poc = Rc::new(PordOrCord::Pord(pord));
        self.new_letter_from_pordorcord(poc, radius, stem_type, ctx, num_of_attach)
    }
    fn sort_letters(&mut self) {
        let location= self.pord.as_ref();
        self.arcs.sort_by_key(|a|location.angle_to(a.pord.as_ref()) as i32);
    }
    pub fn draw(mut self,doc:Document) -> Document {
        println!("drawing {}...",self.name);
        let xy = self.pord.abs_svg_xy(self.default_ctx.origin());
        if !self.path_circle {
            self.draw_circle_only(doc, xy.0, xy.1)
        } else {
            self.sort_letters();
            self.loop_word_arc(doc)
        }
    }
    fn draw_circle_only(self, mut doc: Document, word_x:f64, word_y:f64) ->Document {
        let w_circle = Circle::new()
            .set("fill", self.default_ctx.colour().fill())
            .set("stroke", self.default_ctx.colour().stroke())
            .set("stroke-width", self.default_ctx.stroke().strokewidth())
            .set("cx", word_x)
            .set("cy", word_y)
            .set("r", self.radius);
        doc = doc.add(w_circle);
        for letter in &self.arcs {
            doc = doc.add(self.letter_circle_node(letter));
        }
        doc
    }
    fn loop_word_arc(self, mut doc:Document) -> Document {
        let mut l_iter = self.arcs.iter();
        let letter = l_iter.next().expect("no letters in word arc");
        let mut prev_pord = letter.pord();
        let mut circle_letters = Vec::new();
        let mut i_letter_start_angle = self.calc_letter_ang(letter.pord.as_ref());
        let mut o_letter_start_angle = i_letter_start_angle;
        let (i_word_start_angle, o_word_start_angle) = match letter.stem_type {
            StemType::J | StemType::Z => (0.0, 0.0), 
            StemType::B | StemType::S => {
                if let (Some(thi1),Some(thi2),_,_) = self.calc_letter_thi(letter) {
                    i_letter_start_angle -= thi2;
                    o_letter_start_angle -= thi1;
                    let val1 = if i_letter_start_angle < 0.0 {i_letter_start_angle} else {0.0};
                    let val2 = if o_letter_start_angle < 0.0 {o_letter_start_angle} else {0.0};
                    (val1,val2)
                } else {
                    (0.0,0.0)
                }
            },
        };
        let (mut i_word_end_angle, mut o_word_end_angle) = (i_word_start_angle,o_word_start_angle);
        let mut data = self.start_path_data((i_word_start_angle, o_word_start_angle));
        if i_word_start_angle < i_letter_start_angle || o_word_start_angle < o_letter_start_angle {
            data = self.draw_word_arc(data,(i_word_start_angle,o_word_start_angle),(i_letter_start_angle,o_letter_start_angle));
        }
        if i_word_end_angle <= 0.0 {
            i_word_end_angle += PI*2.0;
        }
        if o_word_end_angle <= 0.0 {
            o_word_end_angle += PI*2.0;
        }
        let mut cir: Option<Circle>;
        let mut end_angle: (f64, f64);
        (cir, data, end_angle) = self.draw_letter_arc(letter, data);
        if let Some(letter_circle) = cir {
            circle_letters.push(letter_circle);
        }; 
        while let Some(letter) = l_iter.next() {
            if Rc::ptr_eq(&prev_pord, &letter.pord()) {
                prev_pord = letter.pord();
                (cir, data,_) = self.draw_letter_arc( letter, data);
                if let Some(letter_circle) =  cir {
                    circle_letters.push(letter_circle);
                };
                continue;
            }
            prev_pord = letter.pord();
            i_letter_start_angle = self.calc_letter_ang(letter.pord.as_ref());
            o_letter_start_angle = i_letter_start_angle;
            let (i_thi, o_thi) = match letter.stem_type {
                StemType::J | StemType::Z => (0.0, 0.0), 
                StemType::B | StemType::S => {
                    if let (Some(thi1),Some(thi2),_,_) = self.calc_letter_thi(letter) {
                        (thi2, thi1)
                    } else {(0.0,0.0)}
                }
            };
            i_letter_start_angle -= i_thi;
            o_letter_start_angle -= o_thi;
            if i_letter_start_angle > end_angle.0 {
                //this will break if we start doing overlapping s_divots
                data = self.draw_word_arc(data,end_angle,(i_letter_start_angle,o_letter_start_angle));
            }
            (cir, data,end_angle) = self.draw_letter_arc( letter, data);
            if let Some(letter_circle) =  cir {
                circle_letters.push(letter_circle);
            };
        } 
        data = self.draw_word_arc(data,end_angle,(i_word_end_angle,o_word_end_angle));
        let i_word_arc = Path::new()
            .set("d", data.0.close())
            .set("fill", self.default_ctx.colour().bg())
            .set("stroke", "none")
            .set("stroke-width", 0.0);
        let o_word_arc = Path::new()
            .set("d", data.1.close())
            .set("fill", self.default_ctx.colour().stroke())
            .set("stroke", "none")
            .set("stroke-width", 0.0);
        doc = doc.add(o_word_arc).add(i_word_arc);
        for cir in circle_letters {
            doc = doc.add(cir);
        }
        doc
    }
    fn draw_letter_arc(&self, letter:&LetterArc, data:(Data,Data)) -> (Option<Circle>,(Data,Data), (f64,f64)) {
        let mut i_end_angle = self.calc_letter_ang(letter.pord.as_ref());
        let s_divot = match letter.stem_type {
            StemType::J | StemType::Z => {
                return (Some(self.letter_circle_node(letter)),data,(i_end_angle,i_end_angle)); 
            }, 
            StemType::S => true,
            StemType::B => false
        };
        let (i_radius,o_radius)= self.get_letter_radii(letter);
        let mut o_end_angle = i_end_angle;
        if let (Some(thi1),Some(thi2),_,_) = self.calc_letter_thi(letter) {
            i_end_angle += thi2;
            o_end_angle += thi1;
        }
        let i_xy = self.calc_word_arc_svg_point(i_end_angle,true);
        let o_xy = self.calc_word_arc_svg_point(o_end_angle,false);
        let i_data = data.0
            .elliptical_arc_to((
                o_radius,o_radius,
                0.0, //angle offset
                if s_divot {0.0} else {1.0}, //large arc
                1.0, //sweep dir - 0 anti-clockwise
                i_xy.0,i_xy.1,
            ));
        let o_data = data.1
            .elliptical_arc_to((
                i_radius,i_radius,
                0.0, //angle offset
                if s_divot {0.0} else {1.0}, //large arc
                1.0, //sweep dir - 0 anti-clockwise
                o_xy.0,o_xy.1,
            ));
        (None,(i_data,o_data), (i_end_angle,o_end_angle))
    }
    fn draw_word_arc(&self, data:(Data,Data), start_angle:(f64,f64),end_angle:(f64,f64)) -> (Data, Data) {
        let (i_radius,o_radius) = self.get_radii();
        let i_end = self.calc_word_arc_svg_point(end_angle.0, true);
        let o_end = self.calc_word_arc_svg_point(end_angle.1, false);
        let i_large_arc = end_angle.0 - start_angle.0 > PI;
        let o_large_arc = end_angle.1 - start_angle.1 > PI;
        let outer_arc = data.1        
            .elliptical_arc_to((
                o_radius,o_radius,
                0.0, //angle offset
                if o_large_arc {1.0} else {0.0}, //large arc
                0.0, //sweep dir - 0 anti-clockwise
                o_end.0,o_end.1,
            ));
        let inner_arc = data.0        
            .elliptical_arc_to((
                i_radius,i_radius,
                0.0, //angle offset
                if i_large_arc {1.0} else {0.0}, //large arc
                0.0, //sweep dir - 0 anti-clockwise
                i_end.0,i_end.1,
            ));
        (inner_arc,outer_arc)
    }
    fn letter_circle_node(&self, letter:&LetterArc) -> Circle {
        let ctx = match &letter.ctx {
            None => &self.default_ctx,
            Some(con) => &con.clone()
        };
        let (x,y) = letter.pord.abs_svg_xy(ctx.origin());
        Circle::new()
            .set("fill", ctx.colour().fill())
            .set("stroke", ctx.colour().stroke())
            .set("stroke-width", ctx.stroke().strokewidth())
            .set("cx", x)
            .set("cy", y)
            .set("r", letter.radius)
    }
    fn start_path_data(&self, angle:(f64,f64)) -> (Data, Data) {
        let inner_start_xy = self.calc_word_arc_svg_point(angle.0, true);
        let outer_start_xy = self.calc_word_arc_svg_point(angle.1, false);
        let o_data = Data::new()
            .move_to(outer_start_xy);
        let i_data = Data::new()
            .move_to(inner_start_xy);
        (i_data,o_data)
    }
    fn calc_letter_thi(&self, letter:&LetterArc) -> (Option<f64>, Option<f64>, Option<f64>, Option<f64>) {
        let l_stroke = match &letter.ctx {
            Some(con) => &con.stroke().clone(),
            None => self.default_ctx.stroke()
        };
        let (word_r_i, word_r_o) = self.get_radii();
        let lett_r_i = letter.radius - l_stroke.i_stroke();
        let lett_r_o = letter.radius + l_stroke.o_stroke();
        let dist_sq = self.calc_dist_sq(letter.pord.as_ref());
        //"outer thi"
        let thi1_top = -lett_r_i.powi(2) + dist_sq + word_r_o.powi(2);
        let thi1_bot = dist_sq.sqrt()*2.0*word_r_o;
        let thi1 = thi_check(thi1_top, thi1_bot);
        //"inner thi"
        let thi2_top = -lett_r_o.powi(2) + dist_sq + word_r_i.powi(2);
        let thi2_bot = 2.0*dist_sq.sqrt()*word_r_i;
        let thi2 = thi_check(thi2_top, thi2_bot);
        //inner word boundary thi
        let thi3_top = -lett_r_i.powi(2) + dist_sq + word_r_i.powi(2);
        let thi3_bot = 2.0*dist_sq.sqrt()*word_r_i;
        let thi3 = thi_check(thi3_top, thi3_bot);
        //outer word boundary thi
        let thi4_top = -lett_r_o.powi(2) + dist_sq + word_r_o.powi(2);
        let thi4_bot = 2.0*dist_sq.sqrt()*word_r_o;
        let thi4 = thi_check(thi4_top, thi4_bot);
        (thi1,thi2,thi3,thi4)
    }
    fn calc_word_arc_svg_point(&self, angle:f64, inner:bool) -> (f64,f64) {
        let stroke = self.default_ctx.stroke();
        let (a,b) = angle.sin_cos();
        let (x,y) = self.abs_svg_xy(self.default_ctx.origin());
        //negatives cancel
        if inner {
            let i_radius = self.radius - stroke.i_stroke();
            (x + i_radius * a,  y + i_radius * b)
        } else {
            let o_radius = self.radius + stroke.o_stroke();
            (x + o_radius * a,  y + o_radius * b)
        }
    }
    fn calc_letter_ang(&self, pord:&PordOrCord) -> f64 {
        self.angle_to(pord)
    }
    fn calc_dist_sq(&self, pord:&PordOrCord) -> f64 {
        self.pord.dist_to_sq(pord)
    }
    fn get_letter_radii(&self, letter:&LetterArc) -> (f64,f64) {
        let stroke = match &letter.ctx {
            None => self.default_ctx.stroke(),
            Some(con) => con.stroke()
        };
        (letter.radius - stroke.i_stroke(), letter.radius + stroke.o_stroke())
    }
    pub fn get_last_letter_pord(&self) -> Option<Rc<PordOrCord>> {
        let lett = self.arcs.last()?;
        Some(lett.pord())
    }
    fn get_radii(&self) -> (f64,f64) {
        let stroke = self.default_ctx.stroke();
        (self.radius - stroke.i_stroke(),self.radius + stroke.o_stroke())
    }
}

impl Cartesian for Word {
    fn rel_xy(&self) -> (f64,f64) {
        self.pord.rel_xy()
    }

    fn abs_svg_xy(&self, svg_origin:(f64,f64)) -> (f64,f64) {
        self.pord.abs_svg_xy(svg_origin)
    }
}

impl LetterArc {
    pub fn new(pord: Rc<PordOrCord>, radius: f64, stem_type: StemType, ctx:Option<Context>) -> LetterArc {
        LetterArc {
            pord,
            radius, 
            stem_type, 
            ctx
        }
    }
    fn pord(&self) -> Rc<PordOrCord> {
        self.pord.clone()
    }
}

fn thi_check(top:f64,bot:f64) -> Option<f64> {
    if top.abs() <= bot {
        Some((top/bot).acos())
    } else {None}
}