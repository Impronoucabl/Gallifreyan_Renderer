use svg;

use crate::pord;
use crate::basic;
use crate::ctx;

fn prelude(origin:(f64,f64)) -> (ctx::Context,ctx::Context,ctx::Context,ctx::Context) {
    let colour = ctx::ColourContext::new("none","black");
    let stroke = ctx::StrokeContext::new(20.0);
    let prime_ctx = ctx::Context::new(colour,stroke,origin);
    let word_ctx = prime_ctx.new_strokewidth(10.0); 
    let lett_ctx =  prime_ctx.new_strokewidth(8.0); 
    let lett2_ctx =  prime_ctx.new_strokewidth(10.0);
    (prime_ctx,word_ctx,lett_ctx,lett2_ctx)
}

pub fn do_this(mut doc:svg::Document, origin:(f64,f64)) -> svg::Document {
    let (prime_ctx,word_ctx,lett_ctx,lett2_ctx) = prelude(origin);
    doc = basic::circle(doc, (0.0,0.0), 1000.0,&prime_ctx);
    doc = basic::circle(doc, (0.0,800.0), 100.0,&prime_ctx);
    doc = basic::arc_circle(doc, (-400.0,-300.0),(0.0,500.0),500.0,&lett_ctx);
    doc = basic::circle(doc, (300.0,0.0), 250.0, &word_ctx);
    basic::circle(doc, (0.0,0.0),300.0, &prime_ctx)
}