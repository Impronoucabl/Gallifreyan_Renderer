use std::{io::Error, rc::Rc};

use svg::Document;


pub mod ctx;
pub mod pord;
pub mod basic;
pub mod decorator;
pub mod word;
pub mod utils;

pub fn canvas_init(width:u64, height:u64, solid_background:&str) -> (Document, Rc<(f64,f64)>) {
    let drawn = Document::new().set("viewBox", (0, 0, width, height));   
    let background = svg::node::element::Rectangle::new()
    .set("x", 0)
    .set("y", 0)
    .set("width", width)
    .set("height", height)
    .set("fill", solid_background)
    .set("stroke", "none");
    (drawn.add(background), Rc::new(((width/2) as f64,(height/2) as f64)))
}

pub fn save(filepath: String, doc:&Document) -> Result<(), Error> {
    let filename = filepath + ".svg";
    println!("Saving under {}", filename);
    svg::save(filename, doc)
}

