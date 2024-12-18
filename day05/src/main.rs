use std::error::Error;
use std::fs::File;

use day05::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Sum of middle pages correctly ordered: {}",
        sum_middle_pages(File::open("input.txt")?, false)?
    );

    println!(
        "Sum of middle pages incorrectly ordered: {}",
        sum_middle_pages(File::open("input.txt")?, true)?
    );

    Ok(())
}
