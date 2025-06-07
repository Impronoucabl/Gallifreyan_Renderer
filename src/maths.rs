

pub fn thi_check(top:f64,bot:f64) -> Option<f64> {
    if top.abs() <= bot {
        Some((top/bot).acos())
    } else {None}
}