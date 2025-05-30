
use svg::Document;

mod pord;

const ORIGIN: (f64,f64) = (0.0,0.0);
const WIDTH: i64 = 2048;
const HEIGHT:i64 = 2048;
const SOLID_BACKGROUND: bool = true;
const fn canvas_colour() -> &'static str {"yellow"}

fn main() {
    println!("Hello, world!");
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