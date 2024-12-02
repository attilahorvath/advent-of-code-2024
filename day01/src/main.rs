use std::error::Error;
use std::fs::File;

use day01::*;

fn main() -> Result<(), Box<dyn Error>> {
    let lists = read_lists(File::open("input.txt")?)?;

    println!("Total distance: {}", total_distance(&lists));
    println!("Similarity score: {}", similarity_score(&lists));

    Ok(())
}
