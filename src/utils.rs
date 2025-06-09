
use std::f64::consts::PI;

pub fn ang_iter(num:usize) -> impl Iterator<Item = f64> {
    let step = 2.0*PI/num as f64;
    let obj = 0..num;
    obj.map(move |i|i as f64 * step)
}

pub fn thi_check(top:f64,bot:f64) -> Option<f64> {
    if top.abs() <= bot {
        Some((top/bot).acos())
    } else {None}
}

#[macro_export] 
macro_rules! pord_vec2dot {
    ($pord_vec:expr, $dist_mod:expr, $radius:expr, $ctx:expr, $doc:expr) => {
        for mut loc in $pord_vec {
            loc.add_dist($dist_mod);
            let pord = Rc::new(Pord(loc));
            $doc = basic::circle($doc,pord.as_ref(),$radius,$ctx);
        }
    };
}

#[macro_export]
macro_rules! pord_from_vec_pop {
    ($pord_vec:expr,$pord_name:ident) => {
        let $pord_name = Rc::new(Pord($pord_vec.pop().unwrap()))
    };
}