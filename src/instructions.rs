use svg;

use crate::pord;
use crate::basic;
use crate::ctx;

pub fn do_this(doc:svg::Document, origin:(f64,f64)) -> svg::Document {
    let colour = ctx::ColourContext::new("none","black");
    let stroke = ctx::StrokeContext::new(20.0);
    let prime_ctx = ctx::Context::new(colour,stroke);        
    basic::circle(doc, origin, 1000.0,prime_ctx)
}