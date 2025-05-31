use svg;

use crate::pord;
use crate::basic;
use crate::ctx;

pub fn do_this(mut doc:svg::Document, origin:(f64,f64)) -> svg::Document {
    let colour = ctx::ColourContext::new("none","black");
    let stroke = ctx::StrokeContext::new(20.0);
    let prime_ctx = ctx::Context::new(colour,stroke, origin);        
    let doc2 = basic::circle(doc, (0.0,0.0), 1000.0,&prime_ctx);
    doc = basic::circle(doc2, (0.0,800.0), 100.0,&prime_ctx);
    let thin_ctx = prime_ctx.new_strokewidth(10.0);
    doc = basic::circle(doc, (500.0,0.0), 250.0, &thin_ctx);
    basic::circle(doc, (0.0,0.0),300.0, &prime_ctx)
}