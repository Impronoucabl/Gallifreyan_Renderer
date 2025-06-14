use std::f32::consts::PI;
use std::rc::{Rc, Weak};

use svg::node::element::path::Data;
use svg::Document;
use svg::node::element::{Circle, Path};

use crate::ctx::Context;
use crate::pord::{Cartesian, POrd, PordOrCord};
use crate::utils;
use crate::StemType;

enum RadiusType{Inner,Average,Outer}
#[derive(Debug,Clone, Copy,PartialEq, PartialOrd)]
pub struct InnerAngle(f32);
#[derive(Debug,Clone, Copy,PartialEq, PartialOrd)]
pub struct OuterAngle(f32);
const ZERO_ANGLE : (InnerAngle, OuterAngle) = (InnerAngle(0.0),OuterAngle(0.0));
struct SvgPosition(f32,f32);
#[derive(Debug, Clone)]
pub struct LetterArc {
    pord: Rc<PordOrCord>,
    radius:f32,
    stem_type:StemType,
    ctx:Option<Context>,
}
#[derive(Debug, Clone)]
pub struct WordCircle {
    name:String,
    pord:Rc<PordOrCord>,
    radius:f32,
    arcs: Vec<LetterArc>,
    default_ctx:Context,
    path_circle: bool,
    sorted: bool,
}

pub struct WordArc {
    name:String,
    pord:Rc<PordOrCord>,
    radius:f32,
    arcs: Vec<LetterArc>,
    default_ctx:Context,
    start_angle:f32,
    end_angle:f32,
    arc_tip_length:f32,
    sorted:bool,
}

pub trait Word:Cartesian {
    fn pord(&self) -> Rc<PordOrCord>;
    fn radius(&self) -> f32;
    fn arcs(&mut self) -> &mut Vec<LetterArc>;
    fn ctx(&self) -> Context;
    fn get_last_letter(&self) -> Option<&LetterArc>;
    fn get_first_letter(&self) -> Option<&LetterArc>;
    fn new_letter(&mut self, pord:Rc<PordOrCord>,radius:f32,stem_type:StemType,ctx:Option<Context>) -> Weak<PordOrCord>;
    fn new_letter_from_data(&mut self, r:f32,theta:f32,radius:f32,stem_type:StemType,ctx:Option<Context>) -> Rc<PordOrCord> {
        let dist = if stem_type == StemType::S {
            r + self.ctx().stroke().strokewidth()/2.0
        } else {r};
        let location = Rc::new(PordOrCord::Pord(POrd::new(dist,theta,self.pord())));
        self.new_letter(location.clone(), radius, stem_type, ctx);
        location
    }
    fn new_letter_from_pordorcord(&mut self,pord:Rc<PordOrCord>, radius:f32, stem_type:StemType,ctx:Option<Context>, num_of_attach:usize) -> Vec<POrd> {
        self.new_letter(pord.clone(), radius, stem_type, ctx);
        utils::generate_pord_vector(num_of_attach,pord.clone(),radius)
    }
    fn new_letter_from_pord(&mut self, pord:POrd,radius:f32,stem_type:StemType, ctx:Option<Context>, num_of_attach:usize) -> (Weak<PordOrCord>,Vec<POrd>) {
        let poc: Rc<PordOrCord> = Rc::new(pord.into());
        let loc = Rc::downgrade(&poc.clone());
        (loc, self.new_letter_from_pordorcord(poc, radius, stem_type, ctx, num_of_attach))
    }
    fn new_letter_with_attach(&mut self, r:f32,theta:f32,radius:f32,stem_type:StemType,ctx:Option<Context>, num_of_attach:usize) -> (Rc<PordOrCord>,Vec<POrd>) {
        let letter_pord = self.new_letter_from_data(r, theta, radius, stem_type, ctx);
        let result = utils::generate_pord_vector(num_of_attach,letter_pord.clone(),radius);
        (letter_pord,result)
    }
    fn sorted(&mut self) -> &mut bool;
    fn sort_letters(&mut self) {
        if *self.sorted() {
            return
        } 
        let location= self.pord();
        self.arcs().sort_by_key(|a|location.angle_to(a.pord.as_ref()) as i32);
        *self.sorted() = true;
    }
    fn start_path_data(&self, angle:(InnerAngle,OuterAngle)) -> (Data, Data);
    fn end_path_data(&self, doc:Document, data:(Data,Data)) -> Document;
    fn draw(self,doc:Document) -> Document;
    //This assumes the arc is already sorted.
    fn word_arc_loop(&mut self, mut doc:Document) -> Document {
        let arc_vec = self.arcs().clone();
        let mut l_iter = arc_vec.iter();
        let letter = l_iter.next().expect("no letters in word arc");
        let mut prev_pord = letter.pord();
        let mut circle_letters = Vec::new();
        let (mut i_letter_start_angle, mut o_letter_start_angle) = self.calc_starting_letter_angle();
        let i_word_start_angle = if i_letter_start_angle.0 < 0.0 {
            i_letter_start_angle
        } else {0.0.into()};
        let o_word_start_angle = if o_letter_start_angle.0 < 0.0 {
            o_letter_start_angle
        } else {0.0.into()};
        let mut data = self.start_path_data((i_word_start_angle, o_word_start_angle));
        if i_word_start_angle < i_letter_start_angle || o_word_start_angle < o_letter_start_angle {
            data = self.draw_word_arc(data,(i_word_start_angle,o_word_start_angle),(i_letter_start_angle,o_letter_start_angle));
        }
        let mut cir: Option<Circle>;
        let mut end_angle: (InnerAngle,OuterAngle);
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
            i_letter_start_angle.0 = self.angle_to(letter.pord.as_ref());
            o_letter_start_angle.0 = i_letter_start_angle.0;
            let (i_thi, o_thi) = match letter.stem_type {
                StemType::J | StemType::Z => (0.0, 0.0), 
                StemType::B | StemType::S => {
                    if let (Some(thi1),Some(thi2),_,_) = self.calc_letter_thi(letter) {
                        (thi2, thi1)
                    } else {(0.0,0.0)}
                }
            };
            i_letter_start_angle.0 -= i_thi;
            o_letter_start_angle.0 -= o_thi;
            if i_letter_start_angle > end_angle.0 {
                //this will break if we start doing overlapping s_divots
                data = self.draw_word_arc(data,end_angle,(i_letter_start_angle,o_letter_start_angle));
            }
            (cir, data,end_angle) = self.draw_letter_arc( letter, data);
            if let Some(letter_circle) =  cir {
                circle_letters.push(letter_circle);
            };
        }
        let end_angles = (
            if i_word_start_angle.0 <= 0.0 {
                i_word_start_angle.0 + PI*2.0
            } else {i_word_start_angle.0}.into(),
            if o_word_start_angle.0 <= 0.0 {
                o_word_start_angle.0 + PI*2.0
            } else {o_word_start_angle.0}.into()
        );
        data = self.draw_word_arc(data,end_angle,end_angles);
        doc = self.end_path_data(doc, data);
        for cir in circle_letters {
            doc = doc.add(cir);
        }
        doc
    }
    fn draw_word_arc(&self, data:(Data,Data), start_angle:(InnerAngle,OuterAngle), end_angle:(InnerAngle,OuterAngle)) -> (Data, Data) {
        let (i_radius,o_radius) = self.get_radii();
        let i_end = self.calc_word_arc_svg_point(end_angle.0.0, RadiusType::Inner);
        let o_end = self.calc_word_arc_svg_point(end_angle.1.0, RadiusType::Outer);
        let i_large_arc = end_angle.0.0 - start_angle.0.0 > PI;
        let o_large_arc = end_angle.1.0 - start_angle.1.0 > PI;
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
    fn draw_letter_arc(&self, letter:&LetterArc, data:(Data,Data)) -> (Option<Circle>,(Data,Data), (InnerAngle,OuterAngle)) {
        let mut i_end_angle = self.angle_to(letter.pord.as_ref());
        let s_divot = match letter.stem_type {
            StemType::J | StemType::Z => {
                return (Some(self.letter_circle_node(letter)),data,(i_end_angle.into(),i_end_angle.into())); 
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
        let i_xy = self.calc_word_arc_svg_point(i_end_angle,RadiusType::Inner);
        let o_xy = self.calc_word_arc_svg_point(o_end_angle,RadiusType::Outer);
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
        (None,(i_data,o_data), (i_end_angle.into(),o_end_angle.into()))
    }
    fn letter_circle_node(&self, letter:&LetterArc) -> Circle {
        let ctx = match &letter.ctx {
            None => &self.ctx(),
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
    fn calc_starting_letter_angle(&self) -> (InnerAngle,OuterAngle) {
        let letter = self.get_first_letter().expect("no letters in word arc");
        let mut i_letter_start_angle = self.angle_to(letter.pord.as_ref());
        let mut o_letter_start_angle = i_letter_start_angle;
        match letter.stem_type { 
            StemType::B | StemType::S => {
                match self.calc_letter_thi(letter) {
                    (Some(thi1),Some(thi2),_,_) => {
                        i_letter_start_angle -= thi2;
                        o_letter_start_angle -= thi1;
                    }
                    (Some(thi1),_,_,_) => {
                        o_letter_start_angle -= thi1;
                    }
                    (_,Some(thi2),_,_) => {
                        i_letter_start_angle -= thi2;
                    }
                    _ => (),
                }
            },
            _ => (),
        }
        (InnerAngle(i_letter_start_angle), OuterAngle(o_letter_start_angle))
    }
    fn calc_word_arc_svg_point(&self, angle:f32, inner:RadiusType) -> SvgPosition {
        let con = self.ctx();
        let stroke = con.stroke();
        let (a,b) = angle.sin_cos();
        let (x,y) = self.abs_svg_xy(con.origin());
        //negatives cancel out
        match inner {
            RadiusType::Inner => {
                let i_radius = self.radius() - stroke.i_stroke();
                SvgPosition(x + i_radius * a,  y + i_radius * b)
            }, 
            RadiusType::Outer => {
                let o_radius = self.radius() + stroke.o_stroke();
                SvgPosition(x + o_radius * a,  y + o_radius * b)
            },
            _ => SvgPosition(x + self.radius() * a,  y + self.radius() * b)
        }
    }
    fn calc_letter_thi(&self, letter:&LetterArc) -> (Option<f32>, Option<f32>, Option<f32>, Option<f32>) {
        let con = match &letter.ctx {
            Some(con) => &con,
            None => &self.ctx()
        };
        let l_stroke = con.stroke().clone();
        let (word_r_i, word_r_o) = self.get_radii();
        let lett_r_i = letter.radius - l_stroke.i_stroke();
        let lett_r_o = letter.radius + l_stroke.o_stroke();
        let dist_sq = self.pord().dist_to_sq(letter.pord.as_ref());
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
    fn get_letter_radii(&self, letter:&LetterArc) -> (f32,f32) {
        let con = match &letter.ctx {
            None => self.ctx(),
            Some(con) => con.clone()
        };
        let stroke = con.stroke();
        (letter.radius - stroke.i_stroke(), letter.radius + stroke.o_stroke())
    }
    fn get_radii(&self) -> (f32,f32) {
        let con = self.ctx();
        let stroke = con.stroke();
        (self.radius() - stroke.i_stroke(),self.radius() + stroke.o_stroke())
    }
}

impl Word for WordCircle {
    fn pord(&self) -> Rc<PordOrCord> {
        self.pord.clone()
    }
    fn ctx(&self) -> Context {
        self.default_ctx.clone()
    }
    fn arcs(&mut self) -> &mut Vec<LetterArc> {
        &mut self.arcs
    }
    fn radius(&self) -> f32 {
        self.radius
    }
    fn get_last_letter(&self) -> Option<&LetterArc> {
        Some(self.arcs.last()?)
    }
    fn get_first_letter(&self) -> Option<&LetterArc> {
        Some(self.arcs.first()?)
    }
    fn sorted(&mut self) -> &mut bool {
        &mut self.sorted
    }
    fn new_letter(&mut self, pord:Rc<PordOrCord>,radius:f32,stem_type:StemType,ctx:Option<Context>) -> Weak<PordOrCord> {
        if let Some(last_lett) = self.get_last_letter() {
            if self.sorted {
                let angle = self.angle_to(pord.as_ref());
                let last_angle = self.angle_to(last_lett.pord().as_ref());
                if angle < last_angle {
                    self.sorted = false
                }
            }
        }
        let letter = LetterArc::new(pord.clone(),radius,stem_type,ctx);
        self.arcs().push(letter);
        if stem_type == StemType::S || stem_type == StemType::B {
            self.path_circle = true;
        }
        Rc::downgrade(&pord)
    }
    fn draw(mut self,doc:Document) -> Document {
        println!("drawing {}...",self.name);
        let xy = self.pord.abs_svg_xy(self.default_ctx.origin());
        if !self.path_circle {
            self.draw_circle_only(doc, xy.0, xy.1)
        } else {
            self.sort_letters();
            self.word_arc_loop(doc)
        }
    }
    fn start_path_data(&self, angle:(InnerAngle, OuterAngle)) -> (Data, Data) {
        //let o_data = Vec::new();
        //let i_data = Vec::new();
        let inner_start_xy = self.calc_word_arc_svg_point(angle.0.0, RadiusType::Inner);
        let outer_start_xy = self.calc_word_arc_svg_point(angle.1.0, RadiusType::Outer);
        let o_data = Data::new();
            //.move_to(outer_start_xy);
        let i_data = Data::new();
            //.move_to(inner_start_xy);
        (i_data,o_data)
    }
    fn end_path_data(&self, doc:Document, data:(Data,Data)) -> Document {
        let i_word_arc = Path::new()
            .set("d", data.0.close())
            .set("fill", self.ctx().colour().bg())
            .set("stroke", "none")
            .set("stroke-width", 0.0);
        let o_word_arc = Path::new()
            .set("d", data.1.close())
            .set("fill", self.ctx().colour().stroke())
            .set("stroke", "none")
            .set("stroke-width", 0.0);
        doc.add(o_word_arc).add(i_word_arc)
    }
    
}

impl Word for WordArc {
    fn pord(&self) -> Rc<PordOrCord> {
        self.pord.clone()
    }
    fn ctx(&self) -> Context {
        self.default_ctx.clone()
    }
    fn arcs(&mut self) -> &mut Vec<LetterArc> {
        &mut self.arcs
    }
    fn radius(&self) -> f32 {
        self.radius
    }
    fn get_last_letter(&self) -> Option<&LetterArc> {
        Some(self.arcs.last()?)
    }
    fn get_first_letter(&self) -> Option<&LetterArc> {
        Some(self.arcs.first()?)
    }
    fn sorted(&mut self) -> &mut bool {
        &mut self.sorted
    }
    fn new_letter(&mut self, pord:Rc<PordOrCord>,radius:f32,stem_type:StemType,ctx:Option<Context>) -> Weak<PordOrCord> {
        let angle = self.angle_to(pord.as_ref());
        if angle < self.start_angle() {
            println!("bad angle - too low");
            panic!()            
        }
        if angle > self.end_angle() {
            println!("bad angle - too high");
            panic!()
        }
        let letter = LetterArc::new(pord.clone(),radius,stem_type,ctx);
        self.arcs().push(letter);
        Rc::downgrade(&pord)
    }
    fn start_path_data(&self, angle:(InnerAngle,OuterAngle)) -> (Data, Data) {
        //let o_data = Vec::new();
        //let i_data = Vec::new();
        let start_xy = self.calc_word_arc_svg_point(self.start_angle()-self.arc_tip_length, RadiusType::Average);
        let inner_start_xy = self.calc_word_arc_svg_point(angle.0.0, RadiusType::Inner);
        let outer_start_xy = self.calc_word_arc_svg_point(angle.1.0, RadiusType::Outer);
        let o_data = Data::new();
            // .move_to(start_xy)
            // .elliptical_arc_to((
            //     self.radius(),self.radius(),
            //     0.0, //angle offset
            //     if self.arc_tip_length > PI {1.0} else {0.0}, //large arc
            //     0.0, //sweep dir - 0 anti-clockwise
            //     outer_start_xy.0,outer_start_xy.1,
            // ));
        let i_data = Data::new();
            // .move_to(start_xy)
            // .elliptical_arc_to((
            //     self.radius(),self.radius(),
            //     0.0, //angle offset
            //     if self.arc_tip_length > PI {1.0} else {0.0}, //large arc
            //     0.0, //sweep dir - 0 anti-clockwise
            //     inner_start_xy.0,inner_start_xy.1,
            // ));
        (i_data,o_data)
    }
    fn end_path_data(&self, doc:Document, data:(Data,Data)) -> Document {
        let start_xy = self.calc_word_arc_svg_point(self.end_angle()+self.arc_tip_length, RadiusType::Average);
        let (i_data,o_data) = data;
        
        let i_word_arc = Path::new()
            .set("d", i_data.close())
            .set("fill", self.ctx().colour().bg())
            .set("stroke", "none")
            .set("stroke-width", 0.0);
        let o_word_arc = Path::new()
            .set("d", o_data.close())
            .set("fill", self.ctx().colour().stroke())
            .set("stroke", "none")
            .set("stroke-width", 0.0);
        doc.add(o_word_arc).add(i_word_arc)
    }
    fn draw(mut self,doc:Document) -> Document {
        println!("drawing {}...",self.name);
        self.sort_letters();
        self.word_arc_loop(doc)
    }
}

impl WordCircle {
    pub fn new(name:&str, pord:Rc<PordOrCord>, radius:f32,ctx:Context) -> WordCircle {
        WordCircle { 
            name: name.to_string(), 
            pord, 
            radius, 
            arcs: Vec::new(), 
            default_ctx: ctx,
            path_circle:false,
            sorted:true,
        }
    }
    fn draw_circle_only(self, mut doc: Document, word_x:f32, word_y:f32) ->Document {
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
}

impl WordArc {
    fn start_angle(&self) -> f32{
        self.start_angle
    }
    fn end_angle(&self) -> f32 {
        self.end_angle
    }
}

impl Cartesian for WordCircle {
    fn rel_xy(&self) -> (f32,f32) {
        self.pord().rel_xy()
    }
    fn abs_svg_xy(&self, svg_origin:(f32,f32)) -> (f32,f32) {
        self.pord().abs_svg_xy(svg_origin)
    }
}

impl Cartesian for WordArc {
    fn rel_xy(&self) -> (f32,f32) {
        self.pord().rel_xy()
    }
    fn abs_svg_xy(&self, svg_origin:(f32,f32)) -> (f32,f32) {
        self.pord().abs_svg_xy(svg_origin)
    }
}

impl LetterArc {
    pub fn new(pord: Rc<PordOrCord>, radius: f32, stem_type: StemType, ctx:Option<Context>) -> LetterArc {
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

fn thi_check(top:f32,bot:f32) -> Option<f32> {
    if top.abs() <= bot {
        Some((top/bot).acos())
    } else {None}
}

impl From<f32> for InnerAngle {
    fn from(value: f32) -> Self {
        InnerAngle(value)
    }
}
impl From<f32> for OuterAngle {
    fn from(value: f32) -> Self {
        OuterAngle(value)
    }
}