use std::f32::consts::PI;
use std::rc::{Rc, Weak};

use svg::Document;
use svg::node::element::{Circle, Path};

use crate::ctx::Context;
use crate::pord::{Cartesian, POrd, PordOrCord};
use crate::utils;
use crate::utils::{LargeArcFlag, PathBuilder, SvgPosition, SweepDirection};
use crate::StemType;

const SORT_PRECISION :i32 = 1000;
const B_DIVOT_FUDGE_PRECISION :f32 = 0.2;

enum RadiusType{Inner,Average,Outer}
#[derive(Debug,Clone, Copy,PartialEq, PartialOrd)]
pub struct InnerAngle(f32);
#[derive(Debug,Clone, Copy,PartialEq, PartialOrd)]
pub struct OuterAngle(f32);
const ZERO_ANGLE : (InnerAngle, OuterAngle) = (InnerAngle(0.0),OuterAngle(0.0));
#[derive(Debug, Clone)]
enum CircleOrClosedPath {
    Cir(Circle),
    Closed(Path)
}
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
#[derive(Debug, Clone)]
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
    fn default_word_start_angle(&self) -> f32;
    fn default_word_end_angle(&self) -> f32;
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
        let (ang1,ang2) = self.calc_starting_letter_angle();
        let max_overlap = std::cmp::min(SORT_PRECISION*ang1.0 as i32,SORT_PRECISION*ang2.0 as i32);
        let overlap = max_overlap as f32/SORT_PRECISION as f32 + PI*2.0;
        let mut count = self.arcs().len();
        while let Some(last) = self.arcs().last() {
            if count == 0 {
                println!("Sorting failed, continuously looping.");
                panic!()
            }
            count -= 1;
            if location.angle_to(last.pord().as_ref()) < overlap {
                break;
            }
            let last = self.arcs().pop().expect("We just tested this");
            self.arcs().insert(1,last);
        }
        *self.sorted() = true;
    }
    fn start_path_data(&self, angle:(InnerAngle,OuterAngle)) -> (PathBuilder, PathBuilder);
    fn end_path_data(&self, doc:Document, data:(PathBuilder, PathBuilder)) -> Document;
    fn draw(self,doc:Document) -> Document;
    //This assumes the arc is already sorted.
    fn word_arc_loop(&mut self, mut doc:Document) -> Document {
        let arc_vec = self.arcs().clone();
        let mut l_iter = arc_vec.iter();
        let letter = l_iter.next().expect("no letters in word arc");
        let mut prev_pord = letter.pord();
        let mut circle_letters = Vec::new();
        let (mut i_letter_start_angle, mut o_letter_start_angle) = self.calc_starting_letter_angle();
        let i_word_start_angle = if i_letter_start_angle.0 < self.default_word_start_angle() {
            i_letter_start_angle
        } else {self.default_word_start_angle().into()};
        let o_word_start_angle = if o_letter_start_angle.0 < self.default_word_start_angle() {
            o_letter_start_angle
        } else {self.default_word_start_angle().into()};
        let mut data = self.start_path_data((i_word_start_angle, o_word_start_angle));
        if i_word_start_angle < i_letter_start_angle || o_word_start_angle < o_letter_start_angle {
            data = self.draw_word_arc(data,(i_word_start_angle,o_word_start_angle),(i_letter_start_angle,o_letter_start_angle));
        }
        let mut cir: Option<CircleOrClosedPath>;
        let mut end_angle: (InnerAngle,OuterAngle);
        (cir, data, end_angle) = self.draw_letter_arc(letter, data);
        if let Some(letter_circle) = cir {
            circle_letters.push(letter_circle);
        }; 
        while let Some(letter) = l_iter.next() {
            let mut skip = false;
            if Rc::ptr_eq(&prev_pord, &letter.pord()) {
                skip = true;
            } else if let Some(anchor) = letter.pord().get_anchor() {
                if Weak::ptr_eq(&Rc::downgrade(&prev_pord), &anchor) {
                    skip = true;
                }
            }
            if skip {
                prev_pord = letter.pord();
                //first letter must be innermost
                (cir, data,_) = self.draw_stacked_letter_arc(letter, data);
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
                    if let (Some(thi1),Some(thi2),_,_,_) = self.calc_letter_thi(letter) {
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
            (cir, data,end_angle) = self.draw_letter_arc(letter, data);
            if let Some(letter_circle) =  cir {
                circle_letters.push(letter_circle);
            };
        }
        let ending_angle = (
            if i_word_start_angle.0 < self.default_word_start_angle() {
                i_word_start_angle.0 + self.default_word_end_angle()
            } else {self.default_word_end_angle()}.into(),
            if o_word_start_angle.0 < self.default_word_start_angle() {
                o_word_start_angle.0 + self.default_word_end_angle()
            } else {self.default_word_end_angle()}.into()
        );
        data = self.draw_word_arc(data,end_angle,ending_angle);
        doc = self.end_path_data(doc, data);
        for node in circle_letters {
            doc = match node {
                CircleOrClosedPath::Cir(cir) => doc.add(cir),
                CircleOrClosedPath::Closed(path) => doc.add(path),
            };
        }
        doc
    }
    fn draw_word_arc(&self, mut data:(PathBuilder, PathBuilder), start_angle:(InnerAngle,OuterAngle), end_angle:(InnerAngle,OuterAngle)) -> (PathBuilder, PathBuilder) {
        let (i_radius,o_radius) = self.get_radii();
        let i_end = self.calc_word_arc_svg_point(end_angle.0.0, RadiusType::Inner);
        let o_end = self.calc_word_arc_svg_point(end_angle.1.0, RadiusType::Outer);
        data.1.arc_to(
            o_end,
            o_radius,
            LargeArcFlag(end_angle.1.0 - start_angle.1.0 > PI),
            SweepDirection(false) //sweep dir - 0 anti-clockwise
        );
        data.0.arc_to(
            i_end,
            i_radius,
            LargeArcFlag(end_angle.0.0 - start_angle.0.0 > PI),
            SweepDirection(false), //sweep dir - 0 anti-clockwise
        );
        data
    }
    fn draw_stacked_letter_arc(&self, letter:&LetterArc, mut data:(PathBuilder, PathBuilder)) -> (Option<CircleOrClosedPath>,(PathBuilder, PathBuilder), (InnerAngle,OuterAngle)) {
        let mut inner_path_end_angle = self.angle_to(letter.pord.as_ref());
        let b_divot = match letter.stem_type {
            StemType::J | StemType::Z => {
                return (Some(CircleOrClosedPath::Cir(self.letter_circle_node(letter))),data,(inner_path_end_angle.into(),inner_path_end_angle.into())); 
            }, 
            StemType::S => false,
            StemType::B => true
        };
        let mut outer_path_end_angle = inner_path_end_angle;
        let (mut outer_path_start_angle, mut inner_path_start_angle) = (outer_path_end_angle,inner_path_end_angle); 
        let mut out_large_arc = true;
        if let (_,Some(thi2),Some(thi1),_,Some(theta)) = self.calc_letter_thi(letter) {
            outer_path_end_angle += thi1;
            outer_path_start_angle -= thi1;
            inner_path_end_angle += thi2;
            inner_path_start_angle -= thi2;
            if theta < PI/2.0 {
                out_large_arc = false
            }
        }        
        let point_1 = self.calc_word_arc_svg_point(inner_path_start_angle,RadiusType::Average);
        let point_2 = self.calc_word_arc_svg_point(outer_path_start_angle,RadiusType::Average);
        let point_3 = self.calc_word_arc_svg_point(outer_path_end_angle,RadiusType::Average);
        let point_4 = self.calc_word_arc_svg_point(inner_path_end_angle,RadiusType::Average);
        let (word_radius,_) = self.get_radii();
        let (inner_letter_radius,outer_letter_radius)= self.get_letter_radii(letter);
        let mut path_build = PathBuilder::new();
        path_build.move_to(point_1);
        path_build.arc_to(point_2, word_radius, LargeArcFlag(false), SweepDirection(false));
        path_build.arc_to(point_3, inner_letter_radius + B_DIVOT_FUDGE_PRECISION/2.0, LargeArcFlag(b_divot && out_large_arc), SweepDirection(true));
        path_build.arc_to(point_4, word_radius, LargeArcFlag(false), SweepDirection(false));
        path_build.arc_to(point_1, outer_letter_radius + B_DIVOT_FUDGE_PRECISION, LargeArcFlag(b_divot && out_large_arc), SweepDirection(false));
        let path = Path::new()
            .set("d", path_build.build_data().close())
            .set("fill", self.ctx().colour().stroke())
            .set("stroke", "none")
            .set("stroke-width", 0.0);
        (Some(CircleOrClosedPath::Closed(path)),data,(inner_path_end_angle.into(),inner_path_end_angle.into()))
    }
    fn draw_letter_arc(&self, letter:&LetterArc, mut data:(PathBuilder, PathBuilder)) -> (Option<CircleOrClosedPath>,(PathBuilder, PathBuilder), (InnerAngle,OuterAngle)) {
        let mut i_end_angle = self.angle_to(letter.pord.as_ref());
        let b_divot = match letter.stem_type {
            StemType::J | StemType::Z => {
                return (Some(CircleOrClosedPath::Cir(self.letter_circle_node(letter))),data,(i_end_angle.into(),i_end_angle.into())); 
            }, 
            StemType::S => false,
            StemType::B => true
        };
        let (inner_letter_radius,outer_letter_radius)= self.get_letter_radii(letter);
        let mut o_end_angle = i_end_angle;
        let mut oversized_b = false;
        if let (Some(thi1),Some(thi2),_,_,Some(theta)) = self.calc_letter_thi(letter) {
            i_end_angle += thi2;
            o_end_angle += thi1;
            if b_divot && theta < PI/2.0 {
                oversized_b = true;
            }
        }
        data.0.arc_to(
            self.calc_word_arc_svg_point(i_end_angle,RadiusType::Inner),
            outer_letter_radius,
            LargeArcFlag(b_divot && !oversized_b), //large arc
            SweepDirection(true), //sweep dir - 0 anti-clockwise
        );
        data.1.arc_to(
            self.calc_word_arc_svg_point(o_end_angle,RadiusType::Outer),
            inner_letter_radius,
            LargeArcFlag(b_divot), //large arc
            SweepDirection(true), //sweep dir - 0 anti-clockwise
        );
        (None, data, (i_end_angle.into(),o_end_angle.into()))
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
                    (Some(thi1),Some(thi2),_,_,_) => {
                        i_letter_start_angle -= thi2;
                        o_letter_start_angle -= thi1;
                    }
                    (Some(thi1),_,_,_,_) => {
                        o_letter_start_angle -= thi1;
                    }
                    (_,Some(thi2),_,_,_) => {
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
            _ => {
                let fudge_radius = self.radius() - stroke.i_stroke() + B_DIVOT_FUDGE_PRECISION;
                SvgPosition(x + fudge_radius * a,  y + fudge_radius * b)
            }
        }
    }
    fn calc_letter_thi(&self, letter:&LetterArc) -> (Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<f32>) {
        let (word_r_i, word_r_o) = self.get_radii();
        let (lett_r_i, lett_r_o) = self.get_letter_radii(letter);
        let (word_r_i_sq,word_r_o_sq,lett_r_i_sq,lett_r_o_sq) = (word_r_i.powi(2),word_r_o.powi(2),lett_r_i.powi(2),lett_r_o.powi(2));
        let dist_sq = self.pord().dist_to_sq(letter.pord.as_ref());
        //"outer thi"
        let thi1 = cos_rule_angle_c(dist_sq,word_r_o_sq,lett_r_i_sq);
        //"inner thi"
        let thi2 = cos_rule_angle_c(dist_sq,word_r_i_sq,lett_r_o_sq);
        //inner word boundary thi
        let thi3 = cos_rule_angle_c(dist_sq,word_r_i_sq,lett_r_i_sq);
        //outer word boundary thi
        let thi4 = cos_rule_angle_c(dist_sq,word_r_o_sq,lett_r_o_sq);
        //theta
        let theta = cos_rule_angle_c(dist_sq,lett_r_o_sq, word_r_i_sq);
        (thi1,thi2,thi3,thi4,theta)
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
    fn start_path_data(&self, angle:(InnerAngle, OuterAngle)) -> (PathBuilder, PathBuilder) {
        let mut o_data = utils::PathBuilder::new();
        let mut i_data = utils::PathBuilder::new();
        i_data.move_to(self.calc_word_arc_svg_point(angle.0.0, RadiusType::Inner));
        o_data.move_to(self.calc_word_arc_svg_point(angle.1.0, RadiusType::Outer));
        (i_data,o_data)
    }
    fn end_path_data(&self, doc:Document, data:(PathBuilder, PathBuilder)) -> Document {
        let i_word_arc = Path::new()
            .set("d", data.0.build_data().close())
            .set("fill", self.ctx().colour().bg())
            .set("stroke", "none")
            .set("stroke-width", 0.0);
        let o_word_arc = Path::new()
            .set("d", data.1.build_data().close())
            .set("fill", self.ctx().colour().stroke())
            .set("stroke", "none")
            .set("stroke-width", 0.0);
        doc.add(o_word_arc).add(i_word_arc)
    }
    fn default_word_start_angle(&self) -> f32 {0.0}
    fn default_word_end_angle(&self) -> f32 {2.0*PI}
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
            println!("bad angle: {} - too low {}", angle, self.start_angle());
            panic!()            
        }
        if angle > self.end_angle() && angle < self.start_angle() + PI*2.0 {
            println!("bad angle: {} - too high {}", angle, self.end_angle());
            panic!()
        }
        let letter = LetterArc::new(pord.clone(),radius,stem_type,ctx);
        self.arcs().push(letter);
        Rc::downgrade(&pord)
    }
    fn start_path_data(&self, angle:(InnerAngle,OuterAngle)) -> (PathBuilder, PathBuilder) {
        let mut o_data = PathBuilder::new();
        let mut i_data = PathBuilder::new();
        let (i_rad,o_rad) = self.get_radii();
        let rad = 2.0*o_rad - i_rad;
        let start_xy = self.calc_word_arc_svg_point(self.start_angle()-self.arc_tip_length, RadiusType::Outer);
        let inner_start_xy = self.calc_word_arc_svg_point(angle.0.0, RadiusType::Inner);
        let outer_start_xy = self.calc_word_arc_svg_point(angle.1.0, RadiusType::Outer);
        o_data.move_to(start_xy);
        i_data.move_to(start_xy);
        o_data.arc_to(
            outer_start_xy, 
            o_rad,
            LargeArcFlag(self.arc_tip_length > PI), 
            SweepDirection(false)
        );
        i_data.arc_to(
            inner_start_xy, 
            rad,
            LargeArcFlag(self.arc_tip_length > PI), 
            SweepDirection(false)
        );
        (i_data,o_data)
    }
    fn end_path_data(&self, doc:Document, data:(PathBuilder, PathBuilder)) -> Document {
        let (i_rad,o_rad) = self.get_radii();
        let rad = 2.0*o_rad - i_rad;
        let end_xy = self.calc_word_arc_svg_point(self.end_angle()+self.arc_tip_length, RadiusType::Outer);
        let (mut i_path, mut o_path) = data;
        i_path.arc_to(
            end_xy, 
            rad, 
            LargeArcFlag(self.arc_tip_length > PI), 
            SweepDirection(false)
        );
        o_path.arc_to(
            end_xy, 
            o_rad, 
            LargeArcFlag(self.arc_tip_length > PI), 
            SweepDirection(false)
        );
        let mut o_data = o_path.build_data();
        o_data = i_path.reverse_and_apphend(o_data);
        let o_word_arc = Path::new()
            .set("d", o_data.close())
            .set("fill", self.ctx().colour().stroke())
            .set("stroke", "none")
            .set("stroke-width", 0.0);
        doc.add(o_word_arc)
    }
    fn draw(mut self,doc:Document) -> Document {
        println!("drawing {}...",self.name);
        self.sort_letters();
        self.word_arc_loop(doc)
    }
    fn default_word_start_angle(&self) -> f32 {
        self.start_angle()
    }
    fn default_word_end_angle(&self) -> f32 {
        self.end_angle()
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
    pub fn new(name:&str, pord:Rc<PordOrCord>, radius:f32, start_angle:f32, end_angle:f32, arc_tip_length:f32, ctx:Context) -> WordArc {
        WordArc { 
            name: name.to_string(), 
            pord, 
            radius, 
            arcs: Vec::new(), 
            start_angle,
            end_angle,
            arc_tip_length,
            default_ctx: ctx,
            sorted:true,
        }
    }
    pub fn start_angle(&self) -> f32{
        self.start_angle
    }
    pub fn end_angle(&self) -> f32 {
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

fn cos_rule_angle_c(a_dist_sq:f32,b_dist_sq:f32,c_dist_sq:f32) -> Option<f32> {
    let top = a_dist_sq + b_dist_sq - c_dist_sq;
    let bot = a_dist_sq.sqrt()*(b_dist_sq.sqrt())*2.0;
    thi_check(top, bot)
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