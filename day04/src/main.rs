use std::error::Error;
use std::fs::File;

use day04::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Number of times XMAS appears: {}",
        count_words(File::open("input.txt")?, "XMAS", false)?
    );

    println!(
        "Number of times X-MAS appears: {}",
        count_words(File::open("input.txt")?, "MAS", true)?
    );

    Ok(())
}
