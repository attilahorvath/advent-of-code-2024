use std::error::Error;
use std::fs::File;

use day06::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Positions visited: {}",
        positions(File::open("input.txt")?, false)?
    );

    println!(
        "Possible positions for obstacles: {}",
        positions(File::open("input.txt")?, true)?
    );

    Ok(())
}
