use std::rc::Rc;

use svg::node::element::path::Data;
use svg::Document;
use svg::node::element::{Circle, Path};

use crate::basic;
use crate::ctx::Context;
use crate::pord::{POrd, Polar, PordOrCord};

#[derive(Debug, PartialEq, Eq)]
pub enum StemType {B,J,S,Z}
pub struct LetterArc {
    pord: PordOrCord,
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
        let letter = LetterArc::new(PordOrCord::Pord(location),radius,StemType::J,None);
        self.arcs.push(letter);
        if stem_type == StemType::S || stem_type == StemType::B {
            self.path_circle = true;
        }
    }
    pub fn draw(self,mut doc:Document) -> Document {
        let xy = self.pord.svg_xy(self.default_ctx.origin());
        if !self.path_circle {
            self.draw_circle_only(doc, xy.0, xy.1)
        } else {
            self.loop_word_arc(doc, xy)
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
        for letter in self.arcs {
            let ctx = match letter.ctx {
                None => &self.default_ctx,
                Some(con) => &con.clone()
            };
            let (x,y) = letter.pord.svg_xy(ctx.origin());
            let l_cir = Circle::new()
                .set("fill", ctx.colour().fill())
                .set("stroke", ctx.colour().stroke())
                .set("stroke-width", ctx.stroke().strokewidth())
                .set("cx", x)
                .set("cy", y)
                .set("r", letter.radius);
            doc = doc.add(l_cir);
        }
        doc
    }
    fn loop_word_arc(self, mut doc:Document, word_xy:(f64,f64)) -> Document {
        let mut l_iter = self.arcs.iter();
        let letter = match l_iter.next() {
            None => {panic!("no letters in word arc")},
            Some(lett) => lett,
        };
        let (word_start_angle, letter_start_angle) = match letter.stem_type {
            StemType::J | StemType::Z => (0.0, 0.0), // 2nd value is a dummy
            StemType::B => (-0.1, -0.1), //dummy number for now
            StemType::S => (-0.2, -0.2), // another dummy
        };
        let word_end_angle = word_start_angle;
        let mut data = self.start_path_data(word_start_angle);
        if word_start_angle < letter_start_angle {
            data = self.draw_word_arc(data,letter_start_angle);
        }
        doc
    }
    fn draw_letter_arc(&self, letter:LetterArc, data:(Data,Data)) -> (Data,Data) {
        data
    }
    fn draw_word_arc(&self, data:(Data,Data), end_angle:f64) -> (Data, Data) {
        let stroke = self.default_ctx.stroke();
        let i_radius = self.radius - stroke.i_stroke();
        let o_radius = self.radius + stroke.o_stroke();
        let mut end_xy = self.calc_word_arc_point(end_angle, None);
        let o_end = end_xy.pop().expect("Just calculated");
        let outer_arc = data.1        
            .elliptical_arc_to((
                o_radius,o_radius,
                0.0, //angle offset
                0.0, //large arc
                0.0, //sweep dir - 0 anti-clockwise
                o_end.0,o_end.1,
            ));
        let i_end = end_xy.pop().expect("Just calculated.");
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
    fn start_path_data(&self, angle:f64) -> (Data, Data) {
        let mut start_xy = self.calc_word_arc_point(angle, None);
        let o_data = Data::new()
            .move_to(start_xy.pop()
            .expect("we just calculated it"));
        let i_data = Data::new()
            .move_to(start_xy.pop()
            .expect("We just calculated it"));
        (i_data,o_data)
    }
    fn calc_word_arc_point(&self, angle:f64, inner:Option<bool>) -> Vec<(f64,f64)>{
        let stroke = self.default_ctx.stroke();
        let mut result = Vec::with_capacity(2);
        let (a,b) = angle.sin_cos();
        if inner != Some(false) {
            let i_radius = self.radius - stroke.i_stroke();
            result.push((i_radius * a,  i_radius * -b))
        }
        if inner != Some(true) {
            let o_radius = self.radius + stroke.o_stroke();
            result.push((o_radius * a,  o_radius * -b))
        }
        result
    }
}

impl LetterArc {
    pub fn new(pord: PordOrCord, radius: f64, stem_type: StemType, ctx:Option<Context>) -> LetterArc {
        LetterArc {
            pord,
            radius, 
            stem_type, 
            ctx
        }
    }
}