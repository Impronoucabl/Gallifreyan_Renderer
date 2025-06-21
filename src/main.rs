use std::io::Error;

use svg::Document;

//mod test;
mod hello_world;

fn main() -> Result<Document, Error> {
    hello_world::hello_world()
    //test::test()
}
