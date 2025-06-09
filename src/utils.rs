
use std::{f64::consts::PI, rc::Rc};

use crate::pord::{POrd, PordOrCord};

pub enum SweepDirection{Clockwise,AntiClockwise}

pub fn ang_iter(num:usize) -> impl Iterator<Item = f64> {
    let step = 2.0*PI/num as f64;
    let obj = 0..num;
    obj.map(move |i|i as f64 * step)
}

pub fn generate_pord_vector(num:usize, pord:Rc<PordOrCord>,radius:f64) -> Vec<POrd> {
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