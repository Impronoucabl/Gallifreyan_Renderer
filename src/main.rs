use std::io::Error;

use svg::Document;

mod maths;
mod pord;
mod ctx;
mod basic;
mod decorator;
mod word;
mod instructions;

const WIDTH: i64 = 2048;
const HEIGHT:i64 = 2048;
const SOLID_BACKGROUND: bool = true;
const fn canvas_colour() -> &'static str {"yellow"}

fn main() -> Result<(), Error> {
    let filename = "test";
    println!("Starting...");
    let doc = canvasinit();
    let filepath = "Imgs\\".to_owned() + &filename.trim();
    let done_doc = instructions::do_this(doc, (WIDTH as f64/2.0,HEIGHT as f64/2.0));
    save(filepath, &done_doc)
}

fn canvasinit() -> Document {
    let drawn = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));   
    if SOLID_BACKGROUND {
        let background = svg::node::element::Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", canvas_colour())
        .set("stroke", "none");
        drawn.add(background)
    } else {
        drawn
    }
}

pub fn save(filepath: String, doc:&Document) -> Result<(), Error> {
    let filename = filepath + ".svg";
    println!("Saving under {}", filename);
    svg::save(filename, doc)
}