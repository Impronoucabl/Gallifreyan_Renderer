use std::io::Error;

//mod test;
mod hello_world;

fn main() -> Result<(), Error> {
    hello_world::hello_world()
    //test::test()
}
