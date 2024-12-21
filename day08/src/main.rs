use std::error::Error;
use std::fs::File;

use day08::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Unique locations with antinodes: {}",
        antinode_locations(File::open("input.txt")?, false)?
    );

    println!(
        "Unique locations with antinodes using resonant harmonics: {}",
        antinode_locations(File::open("input.txt")?, true)?
    );

    Ok(())
}
