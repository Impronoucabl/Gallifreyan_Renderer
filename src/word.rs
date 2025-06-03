use std::rc::Rc;

use svg::node::element::path::Data;
use svg::Document;
use svg::node::element::{Circle, Path};

use crate::ctx::Context;
use crate::pord::{Cartesian, POrd, PordOrCord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StemType {B,J,S,Z}
pub struct LetterArc {
    pord: Rc<PordOrCord>,
    radius:f64,
    stem_type:StemType,
    ctx:Option<Context>,
}
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
    pub fn new_letter(&mut self, r:f64,theta:f64,radius:f64,stem_type:StemType,ctx:Option<Context>) {
        let location = POrd::new(r,theta,&self.pord.clone());
        let letter = LetterArc::new(Rc::new(PordOrCord::Pord(location)),radius,stem_type,ctx);
        self.arcs.push(letter);
        if stem_type == StemType::S || stem_type == StemType::B {
            self.path_circle = true;
        }
    }
    pub fn draw(self,doc:Document) -> Document {
        let xy = self.pord.abs_svg_xy(self.default_ctx.origin());
        if !self.path_circle {
            self.draw_circle_only(doc, xy.0, xy.1)
        } else {
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
        let letter = match l_iter.next() {
            None => {panic!("no letters in word arc")},
            Some(lett) => lett,
        };
        let mut circle_letters = Vec::new();
        let mut i_letter_start_angle = self.calc_letter_ang(letter.pord.clone());
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
        let (i_word_end_angle,o_word_end_angle) = (i_word_start_angle,o_word_start_angle);
        let mut data = self.start_path_data((i_word_start_angle, o_word_start_angle));
        if i_word_start_angle < i_letter_start_angle {
            data = self.draw_word_arc(data,(i_letter_start_angle,o_letter_start_angle));
        }
        match self.draw_letter_arc(letter, data) {
            (Some(letter_circle), new_data)=> {
                circle_letters.push(letter_circle);
                data = new_data;
            },
            (_,new_data) => {data = new_data;} 
        }
        while let Some(letter) = l_iter.next() {
            i_letter_start_angle = self.calc_letter_ang(letter.pord.clone());
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
            data = self.draw_word_arc(data,(i_letter_start_angle,o_letter_start_angle));
            match self.draw_letter_arc( letter, data) {
            (Some(letter_circle), new_data)=> {
                circle_letters.push(letter_circle);
                data = new_data;
            },
            (_,new_data) => {data = new_data;} 
        }
        } 
        data = self.draw_word_arc(data,(i_word_end_angle,o_word_end_angle));
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
    fn draw_letter_arc(&self, letter:&LetterArc, data:(Data,Data)) -> (Option<Circle>,(Data,Data)) {
        let s_divot = match letter.stem_type {
            StemType::J | StemType::Z => {
                return (Some(self.letter_circle_node(letter)),data); 
            }, 
            StemType::S => true,
            StemType::B => false
        };
        let (i_radius,o_radius)= self.get_radii();
        let mut i_end_angle = self.calc_letter_ang(letter.pord.clone());
        let mut o_end_angle = i_end_angle;
        if let (Some(thi1),Some(thi2),_,_) = self.calc_letter_thi(letter) {
            i_end_angle += thi2;
            o_end_angle += thi1;
        }
        let i_xy = self.calc_word_arc_svg_point(i_end_angle,true);
        let o_xy = self.calc_word_arc_svg_point(o_end_angle,false);
        let i_data = data.0
            .elliptical_arc_to((
                i_radius,i_radius,
                0.0, //angle offset
                if s_divot {0.0} else {1.0}, //large arc
                0.0, //sweep dir - 0 anti-clockwise
                i_xy.0,i_xy.1,
            ));
        let o_data = data.1
            .elliptical_arc_to((
                o_radius,o_radius,
                0.0, //angle offset
                if s_divot {0.0} else {1.0}, //large arc
                0.0, //sweep dir - 0 anti-clockwise
                o_xy.0,o_xy.1,
            ));
        (None,(i_data,o_data))
    }
    fn draw_word_arc(&self, data:(Data,Data), end_angle:(f64,f64)) -> (Data, Data) {
        let (i_radius,o_radius) = self.get_radii();
        let i_end = self.calc_word_arc_svg_point(end_angle.0, true);
        let o_end = self.calc_word_arc_svg_point(end_angle.1, false);
        let outer_arc = data.1        
            .elliptical_arc_to((
                o_radius,o_radius,
                0.0, //angle offset
                0.0, //large arc
                0.0, //sweep dir - 0 anti-clockwise
                o_end.0,o_end.1,
            ));
        let inner_arc = data.0        
            .elliptical_arc_to((
                i_radius,i_radius,
                0.0, //angle offset
                0.0, //large arc
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
        let dist_sq = self.calc_dist_sq(letter.pord.clone());
        //"outer thi"
        let thi1_top = lett_r_i.powi(2) - dist_sq - word_r_o.powi(2);
        let thi1_bot = 2.0*dist_sq.sqrt()*word_r_o;
        let thi1 = thi_check(thi1_top, thi1_bot);
        //"inner thi"
        let thi2_top = lett_r_o.powi(2) - dist_sq - word_r_i.powi(2);
        let thi2_bot = 2.0*dist_sq.sqrt()*word_r_i;
        let thi2 = thi_check(thi2_top, thi2_bot);
        //inner word boundary thi
        let thi3_top = lett_r_i.powi(2) - dist_sq - word_r_i.powi(2);
        let thi3_bot = 2.0*dist_sq.sqrt()*word_r_i;
        let thi3 = thi_check(thi3_top, thi3_bot);
        //outer word boundary thi
        let thi4_top = lett_r_o.powi(2) - dist_sq - word_r_o.powi(2);
        let thi4_bot = 2.0*dist_sq.sqrt()*word_r_o;
        let thi4 = thi_check(thi4_top, thi4_bot);
        (thi1,thi2,thi3,thi4)
    }
    fn calc_word_arc_svg_point(&self, angle:f64, inner:bool) -> (f64,f64) {
        let stroke = self.default_ctx.stroke();
        let (a,b) = angle.sin_cos();
        let (x,y) = self.abs_svg_xy(self.default_ctx.origin());
        if inner {
            let i_radius = self.radius - stroke.i_stroke();
            (x + i_radius * a,  y - i_radius * b)
        } else {
            let o_radius = self.radius + stroke.o_stroke();
            (x + o_radius * a,  y - o_radius * b)
        }
    }
    fn calc_letter_ang(&self, pord:Rc<PordOrCord>) -> f64 {
        let ((lett_x, lett_y), (word_x,word_y)) = (pord.rel_xy(),self.pord.rel_xy());
        (lett_x-word_x).atan2(lett_y-word_y)
    }
    fn calc_dist_sq(&self, pord:Rc<PordOrCord>) -> f64 {
        let ((lett_x, lett_y), (word_x,word_y)) = (pord.rel_xy(),self.pord.rel_xy());
        (word_y - lett_y).powi(2) + (word_x - lett_x).powi(2)
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
}

fn thi_check(top:f64,bot:f64) -> Option<f64> {
    if top.abs() <= bot {
        Some((top/bot).acos())
    } else {None}
}