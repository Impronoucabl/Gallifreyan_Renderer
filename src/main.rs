use std::io::Error;

//mod test;
mod hello_world;

fn main() -> Result<(), Error> {
    _ = hello_world::hello_world()?;
    Ok(())
    //test::test()
}
