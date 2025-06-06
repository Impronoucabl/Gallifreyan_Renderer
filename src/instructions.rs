use std::rc::Rc;
use std::f64::consts::PI;
use svg;

use crate::pord::PordOrCord::{Pord,Cord, Gord};
use crate::pord::POrd;
use crate::ctx::{Context, ColourContext, StrokeContext};
use crate::basic;
use crate::word::{Word,StemType};

pub const VOWEL_RADIUS :f64 = 8.0;
pub const LETTER_RADIUS :f64 = 40.0;


fn prelude(origin:(f64,f64)) -> (Context,Context,Context,Context,Context) {
    let colour = ColourContext::new("white","none","black");
    let colour2 = ColourContext::new("white","none","red");
    let mut stroke = StrokeContext::new(20.0);
    let prime_ctx = Context::new(colour,stroke,origin);
    let word_ctx = prime_ctx.new_strokewidth(10.0); 
    let lett_ctx =  prime_ctx.new_strokewidth(8.0); 

    stroke.set_i_stroke(3.0);
    stroke.set_o_stroke(5.0);
    let lett2_ctx = Context::new(colour2,stroke,origin);
    //let lett2_ctx =  prime_ctx.new_strokewidth(10.0);
    
    let filled = ColourContext::new("white","black","none");
    let strokeless = StrokeContext::new(0.0);
    let path_ctx = Context::new(filled,strokeless,origin);
    (path_ctx,prime_ctx,word_ctx,lett_ctx,lett2_ctx)
}

pub fn do_this(mut doc:svg::Document, origin:(f64,f64)) -> svg::Document {
    let gal_origin = Gord(0.0,0.0);
    let svg_origin = Rc::new(Cord(origin.0,origin.1));
    let (path_ctx, prime_ctx,word_ctx,lett_ctx,lett2_ctx) = prelude(origin);
    
    let poi = Rc::new(Pord(POrd::new(400.0,1.5*PI, &svg_origin)));
    let word_p = Rc::new(Pord(POrd::new(400.0,PI, &svg_origin)));
    
    let mut test = Word::new("test",poi.clone(),200.0,word_ctx.clone()); 
    test.new_letter(160.0,PI*0.5,60.0,StemType::B,None);
    test.new_letter(130.0,PI*0.0,LETTER_RADIUS,StemType::J,None);
    let mut test2 = Word::new("test2",word_p.clone(),300.0,word_ctx.clone());
    test2.new_letter(270.0,PI*1.5,80.0,StemType::B,None);
    test2.new_letter(240.0,0.0,VOWEL_RADIUS,StemType::J,None);
    doc = test.draw(doc);
    doc = test2.draw(doc);    
    
    doc = basic::circle(doc, &gal_origin, 1000.0,&prime_ctx);
    doc = basic::circle(doc, &Gord(0.0,-800.0), 100.0,&prime_ctx);
    doc = basic::arc_circle(doc, &Gord(-400.0,-300.0),&Gord(0.0,500.0),500.0,1.0, &lett_ctx);
    doc = basic::circle(doc, &Gord(600.0,0.0), 250.0, &word_ctx);
    doc = basic::circle(doc, &poi,300.0, &lett2_ctx);
    doc = basic::arc_path(doc,10.0,&poi,&Gord(0.0,-300.0),300.0,true,&path_ctx);
    
    doc
}