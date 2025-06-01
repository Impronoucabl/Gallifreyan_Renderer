use std::rc::Rc;

use svg::node::element::path::Data;
use svg::Document;
use svg::node::element::{Circle, Path};

use crate::basic;
use crate::ctx::Context;
use crate::pord::{POrd, Polar, PordOrCord};

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
    default_ctx:Context
}

impl Word {
    pub fn new(name:&str, pord:Rc<PordOrCord>, radius:f64,ctx:Context) -> Word {
        Word { 
            name: name.to_string(), 
            pord, 
            radius, 
            arcs: Vec::new(), 
            default_ctx: ctx,
        }
    }
    pub fn new_letter(&mut self, r:f64,theta:f64,radius:f64,stem_type:StemType,ctx:Option<Context>) {
        let location = POrd::new(r,theta,&self.pord.clone());
        let letter = LetterArc::new(PordOrCord::Pord(location),50.0,StemType::J,None);
        self.arcs.push(letter);
    }
    pub fn draw(self,doc:Document) -> Document {
        let (x,y) = basic::p_or_c2svg(&self.pord, self.default_ctx.origin());
        let circle = Circle::new()
            .set("fill", self.default_ctx.colour().fill())
            .set("stroke", self.default_ctx.colour().stroke())
            .set("stroke-width", self.default_ctx.stroke().strokewidth())
            .set("cx", x)
            .set("cy", y)
            .set("r", self.radius);
        doc.add(circle)
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