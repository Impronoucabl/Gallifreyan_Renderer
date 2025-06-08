use std::f64::consts::PI;
use std::io::Error;
use std::rc::Rc;

use gallifreyan_renderer as Gal;
use Gal::ctx::{Context, ColourContext, StrokeContext};
use Gal::pord::{POrd, PordOrCord::{Pord,Gord}};
use Gal::{basic, decorator, word::{self, StemType}};
use Gal::pord_vec2dot;

const WIDTH: u64 = 2048;
const HEIGHT:u64 = 2048;
const fn canvas_colour() -> &'static str {"yellow"}

const VOWEL_RADIUS :f64 = 12.0;
const LETTER_RADIUS :f64 = 40.0;

fn main() -> Result<(), Error> {
    if true {
        return hello_world()
    }
    let filename = "test2";
    let filepath = "Imgs\\".to_owned() + &filename.trim();
    println!("Starting...");
    let (mut doc, svg_origin) = Gal::canvas_init(WIDTH, HEIGHT, canvas_colour());
    let origin = svg_origin.as_ref();
    let gal_origin = Rc::new(Gord(0.0,0.0));
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
    
    let poi = Rc::new(Pord(POrd::new(400.0,1.5*PI, gal_origin.clone())));
    let word_p = Rc::new(Pord(POrd::new(400.0,PI, gal_origin.clone())));
    
    let mut test = word::Word::new("test",poi.clone(),200.0,word_ctx.clone()); 
    test.new_letter_from_data(155.0,PI*0.5,60.0,StemType::B,None);
    test.new_letter_from_data(130.0,PI*0.0,LETTER_RADIUS,StemType::J,None);
    let mut test2 = word::Word::new("test2",word_p.clone(),300.0,word_ctx.clone());
    test2.new_letter_from_data(200.0,PI*1.5,80.0,StemType::S,None);
    test2.new_letter_from_data(240.0,0.0,VOWEL_RADIUS,StemType::J,None);
    doc = test.draw(doc);
    doc = test2.draw(doc);    
    
    let mut line1 = decorator::Linebuilder::new(&lett2_ctx);
    _ = line1.add_pord(poi.clone());
    _ = line1.add_pord(word_p.clone());
    let mut line2 = line1.clone();
    _ = line2.add_pord(gal_origin.clone());
    line2.switch_pord_1_2();
    let real_line: decorator::StraightLine = line1.try_into().expect("I said so.");
    let curved_line: decorator::CirculcarLine = line2.try_into().expect("I said so too.");

    doc = real_line.draw(doc);
    doc = curved_line.draw(doc);

    doc = basic::circle(doc, gal_origin.as_ref(), 1000.0,&prime_ctx);
    doc = basic::circle(doc, &Gord(0.0,-800.0), 100.0,&prime_ctx);
    doc = basic::arc_big_circle(doc, &Gord(-400.0,-300.0),&Gord(0.0,500.0),500.0,1.0, &lett_ctx);
    doc = basic::circle(doc, &Gord(600.0,0.0), 250.0, &word_ctx);
    doc = basic::circle(doc, &poi,300.0, &lett2_ctx);
    doc = basic::arc_path(doc,10.0,&poi,&Gord(0.0,-300.0),300.0,true,&path_ctx);

    Gal::save(filepath, &doc)
}

fn hello_world() -> Result<(), Error> {
    let Vowel_RADIUS :f64 = 40.0;
    let Letter_RADIUS :f64 = 120.0;

    let filename = "hello";
    let filepath = "Imgs\\".to_owned() + &filename.trim();
    println!("Initialising...");
    let (mut doc, svg_origin) = Gal::canvas_init(WIDTH, HEIGHT, "white");
    let origin = svg_origin.as_ref();
    let gal_origin = Rc::new(Gord(0.0,0.0));
    let colour = ColourContext::new("white","none","black");
    let stroke = StrokeContext::new(60.0);
    let prime_ctx = Context::new(colour,stroke,origin);
    let w_ctx = prime_ctx.new_strokewidth(40.0);
    let l_ctx = prime_ctx.new_strokewidth(25.0);
    let v_ctx = prime_ctx.new_strokewidth(20.0);

    println!("Starting...");
    let hello_pord = Rc::new(Pord(POrd::new(450.0, 0.0, gal_origin.clone())));
    let mut hello = word::Word::new("hello",hello_pord.clone(),400.0,w_ctx.clone());
    let (h_pord,h_points) = hello.new_letter_with_attach(315.0, 0.0, Letter_RADIUS, StemType::B, Some(l_ctx.clone()), 2);
    hello.new_letter_from_pord(h_pord, Vowel_RADIUS, StemType::J, Some(v_ctx.clone()), 0);
    let (l_pord,l_points) = hello.new_letter_with_attach(150.0, PI, Letter_RADIUS, StemType::J, Some(l_ctx.clone()), 3);
    let mut o_pord_vec = hello.new_letter_from_pord(l_pord.clone(), Letter_RADIUS + 70.0, StemType::J, Some(l_ctx.clone()), 1);
    let o_pord = Rc::new(Pord(o_pord_vec.pop().unwrap()));
    hello.new_letter_from_pord(o_pord, Vowel_RADIUS, StemType::J, Some(v_ctx.clone()), 0);
    
    let world_pord = Rc::new(Pord(POrd::new(450.0, PI, gal_origin.clone())));
    let mut world = word::Word::new("world",world_pord.clone(),400.0,w_ctx);
    let (w_pord,mut w_points) = world.new_letter_with_attach(400.0, 0.0, Letter_RADIUS, StemType::S, None, 3);
    let new_o = Rc::new(Pord(w_points.pop().unwrap()));
    world.new_letter_from_pord(new_o, Vowel_RADIUS, StemType::J, Some(v_ctx.clone()),0);
    let (r_pord,r_points) = world.new_letter_with_attach(400.0, PI/2.0, Letter_RADIUS, StemType::S, None, 3);
    let (new_l_pord,mut new_l_points) = world.new_letter_with_attach(220.0, PI, Letter_RADIUS, StemType::J, Some(l_ctx.clone()), 3);
    let (d_pord,mut d_points) = world.new_letter_with_attach(315.0, PI*1.5, Letter_RADIUS, StemType::B, Some(l_ctx.clone()), 3);
    
    println!("Drawing word arcs...");
    doc = hello.draw(doc);
    doc = world.draw(doc);
    println!("Drawing dots...");
    let filled = ColourContext::new("white","black","none");
    let strokeless = StrokeContext::new(0.0);
    let path_ctx = Context::new(filled,strokeless,origin);
    pord_vec2dot!(l_points,-65.0,Vowel_RADIUS - 10.0, &path_ctx,doc);
    pord_vec2dot!(d_points,-65.0,Vowel_RADIUS - 10.0, &path_ctx,doc);
    pord_vec2dot!(r_points,-65.0,Vowel_RADIUS - 10.0, &path_ctx,doc);
    pord_vec2dot!(new_l_points,-65.0,Vowel_RADIUS - 10.0, &path_ctx,doc);
    
    println!("Drawing lines...");
    doc = basic::circle(doc, gal_origin.as_ref(), 970.0,&prime_ctx);
    println!("Saving...");
    Gal::save(filepath, &doc)
}



