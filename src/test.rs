use std::f32::consts::PI;
use std::io::Error;
use std::rc::Rc;

use gallifreyan as Gal;
use svg::Document;
use Gal::ctx::{Context, ColourContext, StrokeContext};
use Gal::pord::{POrd, PordOrCord::{Pord,Gord}};
use Gal::{basic, decorator, word::{self,Word}, StemType};
use Gal::utils::SweepDirection;

const WIDTH: u64 = 2048;
const HEIGHT:u64 = 2048;
const fn canvas_colour() -> &'static str {"yellow"}

const VOWEL_RADIUS :f32 = 12.0;
const LETTER_RADIUS :f32 = 40.0;

pub fn test(filename:&str) -> Result<Document, Error> {
    //let filename = "test2";
    let filepath = "Imgs\\".to_owned() + &filename.trim();
    println!("Starting...");
    let (mut doc, svg_origin) = Gal::canvas_init(WIDTH, HEIGHT, canvas_colour());
    let origin = svg_origin.as_ref();
    let gal_origin = Rc::new(Gord(0.0,0.0));
    let colour = ColourContext::default();
    let colour2 = ColourContext::new("white","none","red");
    let mut stroke = StrokeContext::new(20.0);
    let prime_ctx = Context::new(colour,stroke,origin);
    let thick_ctx = prime_ctx.new_strokewidth(30.0);
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
    
    let mut test = word::WordCircle::new("test",poi.clone(),200.0,lett_ctx.clone()); 
    let l_pord = test.new_letter_from_data(120.0,PI*0.5,90.0,StemType::B,None);
    test.new_letter_from_pordorcord(l_pord, 140.0,StemType::B, None, 0);
    test.new_letter_from_data(130.0,PI*0.0,LETTER_RADIUS,StemType::J,None);
    let mut test2 = word::WordCircle::new("test2",word_p.clone(),300.0,word_ctx.clone());
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
    doc = curved_line.draw_small(doc);

    doc = basic::circle(doc, gal_origin.as_ref(), 1000.0,&prime_ctx);
    doc = basic::circle(doc, &Gord(0.0,-800.0), 100.0,&prime_ctx);
    doc = basic::arc_big_circle(doc, &Gord(-400.0,-300.0),&Gord(0.0,500.0),500.0,SweepDirection(false), &lett_ctx);
    doc = basic::circle(doc, &Gord(600.0,0.0), 250.0, &word_ctx);
    doc = basic::circle(doc, &poi,300.0, &lett2_ctx);
    doc = basic::arc_path(doc,10.0,&poi,&Gord(0.0,-300.0),300.0,SweepDirection(false),&path_ctx);

    Gal::save(filepath, &doc)?;
    Ok(doc)
}