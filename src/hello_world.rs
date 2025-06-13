use std::f64::consts::PI;
use std::io::Error;
use std::rc::Rc;

use gallifreyan::decorator::Linebuilder;
use gallifreyan as Gal;
use Gal::ctx::{Context, ColourContext, StrokeContext};
use Gal::pord::{POrd, PordOrCord};
use Gal::{basic, decorator, word::{self, Word}, StemType};
use Gal::{pord_vec2dot,pord_from_vec_pop};

const WIDTH: u64 = 2048;
const HEIGHT:u64 = 2048;
const DOT_RADIUS    :f64 = 30.0;
const VOWEL_RADIUS  :f64 = 40.0;
const LETTER_RADIUS :f64 = 120.0;

pub fn hello_world() -> Result<(), Error> {
    let filename = "hello_world";
    let filepath = "Imgs\\".to_owned() + &filename.trim();
    println!("Initialising...");
    let (mut doc, svg_origin) = Gal::canvas_init(WIDTH, HEIGHT, "white");
    let origin = svg_origin.as_ref();
    let gal_origin = PordOrCord::gal_origin();
    let colour = ColourContext::default();
    let stroke = StrokeContext::new(60.0);
    let prime_ctx = Context::new(colour,stroke,origin);
    let w_ctx = prime_ctx.new_strokewidth(40.0);
    let l_ctx = prime_ctx.new_strokewidth(25.0);
    let v_ctx = prime_ctx.new_strokewidth(20.0);

    println!("Starting...");
    let hello_pord = Rc::new(PordOrCord::Pord(POrd::new(450.0, 0.0, gal_origin.clone())));
    let mut hello = word::WordCircle::new("hello",hello_pord.clone(),400.0,w_ctx.clone());
    let (h_pord,mut h_points) = hello.new_letter_with_attach(315.0, 0.0, LETTER_RADIUS, StemType::B, Some(l_ctx.clone()), 2);
    hello.new_letter_from_pordorcord(h_pord.clone(), VOWEL_RADIUS, StemType::J, Some(v_ctx.clone()), 0);
    let (l_pord,l_points) = hello.new_letter_with_attach(150.0, PI, LETTER_RADIUS, StemType::J, Some(l_ctx.clone()), 3);
    let mut o_pord_vec = hello.new_letter_from_pordorcord(l_pord.clone(), LETTER_RADIUS + 70.0, StemType::J, Some(l_ctx.clone()), 1);
    let o_pord = o_pord_vec.pop().unwrap();
    hello.new_letter_from_pord(o_pord, VOWEL_RADIUS, StemType::J, Some(v_ctx.clone()), 0);
    
    let world_pord = Rc::new(PordOrCord::Pord(POrd::new(450.0, PI, gal_origin.clone())));
    let mut world = word::WordCircle::new("world",world_pord.clone(),400.0,w_ctx);
    let (_w_pord,mut w_points) = world.new_letter_with_attach(400.0, 0.0, LETTER_RADIUS, StemType::S, None, 3);
    let mut new_o = w_points.pop().unwrap();
    new_o.set_theta(PI);
    world.new_letter_from_pord(new_o.clone(), VOWEL_RADIUS, StemType::J, Some(v_ctx.clone()),0);
    //We're not going to need anu of these pords
    let (_r_pord,r_points) = world.new_letter_with_attach(400.0, PI/2.0, LETTER_RADIUS, StemType::S, None, 3);
    let (_new_l_pord,new_l_points) = world.new_letter_with_attach(220.0, PI, LETTER_RADIUS, StemType::J, Some(l_ctx.clone()), 3);
    let (_d_pord,d_points) = world.new_letter_with_attach(315.0, PI*1.5, LETTER_RADIUS, StemType::B, Some(l_ctx.clone()), 3);
    
    println!("Drawing word arcs...");
    doc = hello.draw(doc);
    doc = world.draw(doc);

    println!("Drawing dots...");
    let filled = ColourContext::default_path();
    let strokeless = StrokeContext::new(0.0);
    let path_ctx = Context::new(filled,strokeless,origin);
    pord_vec2dot!(l_points,-65.0,DOT_RADIUS, &path_ctx,doc);
    pord_vec2dot!(d_points,-65.0,DOT_RADIUS, &path_ctx,doc);
    pord_vec2dot!(r_points,-65.0,DOT_RADIUS, &path_ctx,doc);
    pord_vec2dot!(new_l_points,-65.0,DOT_RADIUS, &path_ctx,doc);
    
    println!("Drawing lines...");
    let line_ctx = prime_ctx.new_strokewidth(10.0);
    let mut first = Linebuilder::new(&line_ctx);
    pord_from_vec_pop!(h_1,h_points,0.0,Some(PI*2.0/3.0));
    pord_from_vec_pop!(h_2,h_points,0.0,Some(PI*4.0/3.0));
    _ = first.add_pord(h_2);
    _ = first.add_pord(h_1);
    _ = first.add_pord(gal_origin.clone());
    let line1: decorator::CirculcarLine = first.try_into().expect("Let's go!");
    doc = line1.draw_big(doc);

    let mut second = Linebuilder::new(&line_ctx);
    pord_from_vec_pop!(w_1,w_points,0.0,Some(PI*2.0/3.0));
    pord_from_vec_pop!(w_2,w_points,0.0,Some(PI*4.0/3.0));
    _ = second.add_pord(w_1);
    _ = second.add_pord(w_2);
    _ = second.add_pord(world_pord.clone());
    let line2:decorator::CirculcarLine = second.try_into().expect("Noice.");
    doc = line2.draw_small(doc);

    doc = basic::circle(doc, gal_origin.as_ref(), 970.0,&prime_ctx);
    println!("Saving...");
    Gal::save(filepath, &doc)
}