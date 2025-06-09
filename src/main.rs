use std::io::Error;

mod test;
//mod hello_world::hello_world;

fn main() -> Result<(), Error> {
    test::test()
}
