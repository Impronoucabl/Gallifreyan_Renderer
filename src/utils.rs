
use std::{f32::consts::PI, rc::Rc};

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
#[derive(Debug, Clone)]
pub struct PathBuilder{
    positions:Vec<SvgPosition>,
    parameters:Vec<PathParameter>
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