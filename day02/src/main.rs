use std::error::Error;
use std::fs::File;

use day02::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Safe reports: {}",
        safe_reports(File::open("input.txt")?, false)?
    );

    println!(
        "Safe reports with exceptions: {}",
        safe_reports(File::open("input.txt")?, true)?
    );

    Ok(())
}
