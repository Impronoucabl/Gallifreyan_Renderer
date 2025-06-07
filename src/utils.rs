
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