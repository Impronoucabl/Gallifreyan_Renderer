use std::rc::Rc;
use std::convert::TryFrom;

use svg::node::element::Line;
use svg::Document;

use crate::basic;
use crate::ctx::Context;
use crate::pord::{Cartesian, PordOrCord};
use crate::utils;

#[derive(Debug, Clone, Default)]
pub struct Linebuilder {
    pord1:Option<Rc<PordOrCord>>,
    pord2:Option<Rc<PordOrCord>>,
    pord3:Option<Rc<PordOrCord>>,
    ctx:Context
}
#[derive(Debug, Clone)]
pub struct StraightLine {
    pord1:Rc<PordOrCord>,
    pord2:Rc<PordOrCord>,
    ctx:Context
}
#[derive(Debug, Clone)]
pub struct CirculcarLine {
    pord1:Rc<PordOrCord>,
    pord2:Rc<PordOrCord>,
    pord3:Rc<PordOrCord>,
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
    set_pord!(set_pord1,pord1);
    set_pord!(set_pord2,pord2);
    set_pord!(set_pord3,pord3);
    pub fn add_pord(&mut self, pord:Rc<PordOrCord>) -> Result<(),Rc<PordOrCord>> {
        //callback hell anybody? Might reimplement into Vec if needed
        self.set_pord1(pord)
        .or_else(|p2|self.set_pord2(p2)
            .or_else(|p3|self.set_pord3(p3))
        )
    }
    pub fn switch_pord_1_2(&mut self) {
        if let (Some(pord_a), Some(pord_b)) = (self.pord1.clone(),self.pord2.clone()) {
            self.pord1 = Some(pord_b);
            self.pord2 = Some(pord_a);
        } else if let Some(pord0) = self.pord1.clone() {
            self.pord1 = None;
            self.pord2 = Some(pord0);
        } else if let Some(pord0) = self.pord2.clone() {
            self.pord1 = Some(pord0);
            self.pord2 = None;
        }
    }
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

impl CirculcarLine {
    pub fn draw_small(self, doc:Document) -> Document {
        //sweep_dir is hardcoded
        let dist1 = self.pord1.dist_to_sq(self.pord3.as_ref());
        let dist2 = self.pord2.dist_to_sq(self.pord3.as_ref());
        let radius = ((dist1 + dist2)/2.0).sqrt();
        basic::arc_small_circle(doc, self.pord1.as_ref(), self.pord2.as_ref(), radius, utils::SweepDirection::AntiClockwise, &self.ctx)
    }
    pub fn draw_big(self, doc:Document) -> Document {
        //sweep_dir is hardcoded
        let dist1 = self.pord1.dist_to_sq(self.pord3.as_ref());
        let dist2 = self.pord2.dist_to_sq(self.pord3.as_ref());
        let radius = ((dist1 + dist2)/2.0).sqrt();
        basic::arc_big_circle(doc, self.pord1.as_ref(), self.pord2.as_ref(), radius, utils::SweepDirection::AntiClockwise, &self.ctx)
    }
}

impl TryFrom<Linebuilder> for StraightLine {
    type Error = Linebuilder;

    fn try_from(value: Linebuilder) -> Result<Self, Self::Error> {
        if value.pord3.is_some() {
            println!("pord3 being discarded")
        }
        match (&value.pord1,&value.pord2) {
            (Some(pord1),Some(pord2)) => {
                Ok(StraightLine{pord1:pord1.clone(),pord2:pord2.clone(),ctx:value.ctx})
            },
            _ => Err(value.clone())
        }
    }
}

impl TryFrom<Linebuilder> for CirculcarLine {
    type Error = Linebuilder;

    fn try_from(value: Linebuilder) -> Result<Self, Self::Error> {
        match (&value.pord1,&value.pord2, &value.pord3) {
            (Some(pord1),Some(pord2), Some(pord3)) => {
                Ok(CirculcarLine{
                    pord1:pord1.clone(),
                    pord2:pord2.clone(),
                    pord3:pord3.clone(),
                    ctx:value.ctx
                })
            },
            _ => Err(value.clone())
        }
    }
}