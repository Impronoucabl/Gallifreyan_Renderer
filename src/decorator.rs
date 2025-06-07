use std::rc::Rc;
use std::convert::TryFrom;

use svg::node::element::Line;
use svg::Document;

use crate::ctx::Context;
use crate::pord::{Cartesian, POrd, PordOrCord};

pub struct Linebuilder {
    pord1:Option<Rc<PordOrCord>>,
    pord2:Option<Rc<PordOrCord>>,
    pord3:Option<Rc<PordOrCord>>,
    ctx:Context
}

pub struct StraightLine {
    pord1:Rc<PordOrCord>,
    pord2:Rc<PordOrCord>,
    ctx:Context
}

macro_rules! set_pord {
    ($fn_name:ident, $fld_name:ident) => {
        fn $fn_name(&mut self, pord:Rc<PordOrCord>) -> Result<(),Rc<PordOrCord>> {
            if self.$fld_name.is_none() {
                self.$fld_name = Some(pord);
                return Ok(())
            }
            Err(pord)
        }
    };
}

impl Linebuilder {
    pub fn new(context:&Context) -> Linebuilder {
        Linebuilder { 
            pord1: None, 
            pord2: None, 
            pord3: None, 
            ctx: context.clone() 
        }
    }
    pub fn add_pord(&mut self, pord:Rc<PordOrCord>) -> Result<(),Rc<PordOrCord>> {
        //callback hell anybody? Might reimplement into Vec if needed
        self.set_pord1(pord)
        .or_else(|p2|self.set_pord2(p2)
            .or_else(|p3|self.set_pord3(p3))
        )
    }
    set_pord!(set_pord1,pord1);
    set_pord!(set_pord2,pord2);
    set_pord!(set_pord3,pord3);
}

impl StraightLine {
    pub fn draw(self, doc:Document) -> Document {
        let (x1,y1) = self.pord1.abs_svg_xy(self.ctx.origin());
        let (x2,y2) = self.pord2.abs_svg_xy(self.ctx.origin());
        let line = Line::new()
            .set("stroke", self.ctx.colour().stroke())
            .set("stroke-width", self.ctx.stroke().strokewidth())
            .set("x1", x1).set("y1", y1)
            .set("x2", x2).set("y2", y2);
        doc.add(line)
    }
}

impl TryFrom<Linebuilder> for StraightLine {
    type Error = ();

    fn try_from(value: Linebuilder) -> Result<Self, Self::Error> {
        if value.pord3.is_some() {
            println!("pord3 being discarded")
        }
        match (value.pord1,value.pord2) {
            (Some(pord1),Some(pord2)) => {
                Ok(StraightLine{pord1,pord2,ctx:value.ctx})
            },
            _ => Err(())
        }
    }
}