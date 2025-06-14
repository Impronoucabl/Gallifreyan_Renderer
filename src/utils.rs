
use std::{f32::consts::PI, rc::Rc};

use svg::node::element::path::{Command, Data};
use svg::node::element::path::Command::{Move, EllipticalArc};
use svg::node::element::path::Position::Absolute as A;

use crate::pord::{POrd, PordOrCord};
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SweepDirection(pub bool);
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct LargeArcFlag(pub bool);
#[derive(Debug, Clone, Copy)]
pub enum PathParameter{
    Move,
    Arc(f32,LargeArcFlag,SweepDirection)
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SvgPosition(pub f32,pub f32);
#[derive(Debug, Default, Clone)]
pub struct PathBuilder{
    positions:Vec<SvgPosition>,
    parameters:Vec<PathParameter>,
    reversed:bool
}

pub fn ang_iter(num:usize) -> impl Iterator<Item = f32> {
    let step = 2.0*PI/num as f32;
    let obj = 0..num;
    obj.map(move |i|i as f32 * step)
}

pub fn ang_iter_from_range(num:usize, min:f32, max:f32) -> impl Iterator<Item = f32> {
    let step = (max - min)/num as f32;
    let obj = 0..num;
    obj.map(move|i|i as f32 * step + min)
}

pub fn generate_pord_vector(num:usize, pord:Rc<PordOrCord>,radius:f32) -> Vec<POrd> {
    let mut result = Vec::with_capacity(num);
    let mut angle_gen = ang_iter(num);
    while let Some(ang) = angle_gen.next() {
        result.push(POrd::new(radius, ang, pord.clone()))
    }
    result
}

impl PathBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn move_to(&mut self, to:SvgPosition) {
        self.parameters.push(PathParameter::Move);
        self.positions.push(to)
    }
    pub fn arc_to(&mut self, to:SvgPosition, radius:f32,arc:LargeArcFlag,sweep:SweepDirection) {
        self.positions.push(to);
        self.parameters.push(
            PathParameter::Arc(radius, arc, sweep)
        );
    }
    pub fn reverse_and_apphend(self, mut data:Data) -> Data {
        let mut pos_iter = self.positions.into_iter().rev();
        let mut param_iter = self.parameters.into_iter().rev();
        let pos = pos_iter.next().expect("Empty vec?");
        data = data.add(Move(A, (pos.0,pos.1).into()));
        while let (Some(pos),Some(param)) = (pos_iter.next(), param_iter.next()) {
            data = match param {
                PathParameter::Move => {
                    data.add(Move(A,(pos.0,pos.1).into()))
                }
                PathParameter::Arc(radius,LargeArcFlag(arc),SweepDirection(sweep)) => {
                    data.add(EllipticalArc(A, (
                        radius,radius,
                        0.0,
                        if arc {1.0} else {0.0},
                        if sweep {0.0} else {1.0}, //swapped since reversed
                        pos.0,pos.1
                    ).into()))
                }
            }
        }
        data
    }
    pub fn build_data(self) -> Data {
        let mut data = Data::new();
        for (pos, param) in self.positions.into_iter().zip(self.parameters) {
            data = match param {
                PathParameter::Move => {
                    data.add(Move(A,(pos.0,pos.1).into()))
                }
                PathParameter::Arc(radius,LargeArcFlag(arc),SweepDirection(sweep)) => {
                    data.add(EllipticalArc(A, (
                        radius,radius,
                        0.0,
                        if arc {1.0} else {0.0},
                        if sweep {1.0} else {0.0},
                        pos.0,pos.1
                    ).into()))
                }
            }
        }
        data
    }
}

#[macro_export] 
macro_rules! pord_vec2dot {
    ($pord_vec:expr, $dist_mod:expr, $radius:expr, $ctx:expr, $doc:expr) => {
        for mut loc in $pord_vec {
            loc.add_dist($dist_mod);
            let pord = Rc::new(PordOrCord::Pord(loc));
            $doc = basic::circle($doc,pord.as_ref(),$radius,$ctx);
        }
    };
}

#[macro_export]
macro_rules! pord_from_vec_pop {
    ($pord_name:ident,$pord_vec:expr,$dist_mod:expr,$new_ang:expr) => {
        let mut loc = $pord_vec.pop().unwrap();
        if let Some(ang) = $new_ang {
            loc.set_theta(ang);
        }
        loc.add_dist($dist_mod);
        let $pord_name = Rc::new(PordOrCord::Pord(loc));
    };
}