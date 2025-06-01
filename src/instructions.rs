use std::rc::Rc;
use std::f64::consts::PI;
use svg;

use crate::pord::PordOrCord::{Pord,Cord};
use crate::pord::POrd;
use crate::ctx::{Context, ColourContext, StrokeContext};
use crate::basic;
use crate::word::{Word,LetterArc,StemType};

pub const VOWEL_RADIUS :f64 = 8.0;


fn prelude(origin:(f64,f64)) -> (Context,Context,Context,Context,Context) {
    let colour = ColourContext::new("none","black");
    let colour2 = ColourContext::new("none","red");
    let mut stroke = StrokeContext::new(20.0);
    let prime_ctx = Context::new(colour,stroke,origin);
    let word_ctx = prime_ctx.new_strokewidth(10.0); 
    let lett_ctx =  prime_ctx.new_strokewidth(8.0); 

    stroke.set_i_stroke(3.0);
    stroke.set_o_stroke(5.0);
    let lett2_ctx = Context::new(colour2,stroke,origin);
    //let lett2_ctx =  prime_ctx.new_strokewidth(10.0);
    
    let filled = ColourContext::new("black","none");
    let strokeless = StrokeContext::new(0.0);
    let path_ctx = Context::new(filled,strokeless,origin);
    (path_ctx,prime_ctx,word_ctx,lett_ctx,lett2_ctx)
}

pub fn do_this(mut doc:svg::Document, origin:(f64,f64)) -> svg::Document {
    let gal_origin = Cord(0.0,0.0);
    let svg_origin = Rc::new(Cord(origin.0,origin.1));
    let (path_ctx, prime_ctx,word_ctx,lett_ctx,lett2_ctx) = prelude(origin);
    
    let poi = Rc::new(Pord(POrd::new(500.0,3.0*PI/2.0, &svg_origin)));
    let word_p = Rc::new(Pord(POrd::new(450.0,0.0, &svg_origin)));
    doc = basic::circle(doc, &gal_origin, 1000.0,&prime_ctx);
    doc = basic::circle(doc, &Cord(0.0,800.0), 100.0,&prime_ctx);
    doc = basic::arc_circle(doc, &Cord(-400.0,-300.0),&Cord(0.0,500.0),500.0,1.0, &lett_ctx);
    doc = basic::circle(doc, &Cord(300.0,0.0), 250.0, &word_ctx);
    doc = basic::circle(doc, &poi,300.0, &lett2_ctx);
    doc = basic::arc_path(doc,10.0,&poi,&Cord(0.0,-300.0),300.0,true,&path_ctx);
    let mut test = Word::new("test",poi.clone(),200.0,word_ctx.clone());
    
    test.new_letter(200.0,PI*1.5,40.0,StemType::Z,None);
    let mut test2 = Word::new("test",word_p.clone(),200.0,word_ctx.clone());
    doc = test.draw(doc);
    doc = test2.draw(doc);
    doc
}