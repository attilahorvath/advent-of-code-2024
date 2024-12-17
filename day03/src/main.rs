use std::error::Error;
use std::fs;

use day03::*;

fn main() -> Result<(), Box<dyn Error>> {
    let memory = fs::read_to_string("input.txt")?;

    println!("Result of multiplications: {}", mul(&memory, false));
    println!(
        "Result of multiplications with conditions: {}",
        mul(&memory, true)
    );

    Ok(())
}
