use std::error::Error;
use std::io::{self, BufRead, Read};
use std::cmp::Ordering;

pub fn safe_reports(input: impl Read) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;

    'reports: for report in io::BufReader::new(input).lines() {
        let report = report?;

        let mut prev_level: Option<i32> = None;
        let mut order = Ordering::Equal;

        for level in report.split_ascii_whitespace() {
            let level = level.parse::<i32>().unwrap_or_default();

            if let Some(prev) = prev_level {
                let curr_order = level.cmp(&prev);
                let difference = (level - prev).abs();

                if order == Ordering::Equal {
                    order = curr_order;
                }

                if curr_order != order || difference < 1 || difference > 3 {
                    continue 'reports;
                }
            }

            prev_level = Some(level);
        }

        count += 1;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_safe_reports() -> Result<(), Box<dyn Error>> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        assert_eq!(2, safe_reports(input.as_bytes())?);

        Ok(())
    }
}
