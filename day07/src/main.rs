use std::error::Error;
use std::fs::File;

use day07::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Total calibration result: {}",
        sum_results(File::open("input.txt")?, false)?
    );

    println!(
        "Total calibration result with concatenations: {}",
        sum_results(File::open("input.txt")?, true)?
    );

    Ok(())
}
