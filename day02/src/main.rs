use std::error::Error;
use std::fs::File;

use day02::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Safe reports: {}", safe_reports(File::open("input.txt")?)?);

    Ok(())
}
