use svg::node::element::path::Data;
use svg::Document;
use svg::node::element::{Circle, Path};

use crate::ctx::Context;
use crate::pord::{Polar, PordOrCord};

pub enum StemType {B,J,S,Z}
struct LetterArc {
    pord: PordOrCord,
    radius:f64,
    stem_type:StemType,
    ctx:Option<Context>,
}
pub struct Word {
    name:String,
    pord:PordOrCord,
    radius:f64,
    thickness:f64,
    arcs: Vec<LetterArc>,
    default_ctx:Context
}