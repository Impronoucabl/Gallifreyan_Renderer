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
        let (x,y) = self.pord.svg_xy(self.default_ctx.origin());
        if !self.path_circle {
            doc = self.draw_circle_only(doc, x, y);
            return doc;
        }
        doc
    }
    fn draw_circle_only(self, mut doc: Document, word_x:f64, word_y:f64) ->Document {
        let mut circles = Vec::with_capacity(self.arcs.len()+1);
        let w_circle = Circle::new()
            .set("fill", self.default_ctx.colour().fill())
            .set("stroke", self.default_ctx.colour().stroke())
            .set("stroke-width", self.default_ctx.stroke().strokewidth())
            .set("cx", word_x)
            .set("cy", word_y)
            .set("r", self.radius);
        circles.push(w_circle);
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
            circles.push(l_cir);
        }
        for cir in circles {
            doc = doc.add(cir)
        }
        doc
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