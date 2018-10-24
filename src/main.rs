extern crate sensehat;
use sensehat::{Colour, SenseHat, SenseHatResult};

fn main() -> SenseHatResult<()> {
    let mut hat = SenseHat::new()?;
    println!("Pressure: {:?}", hat.get_pressure());
    hat.text(" Hello from Rust!  ", Colour::RED, Colour::BLACK)?;
    Ok(())
}
