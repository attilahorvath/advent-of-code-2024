use std::cmp::Ordering;
use std::error::Error;
use std::io::{self, BufRead, Read};

pub fn safe_reports(input: impl Read, allow_exception: bool) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;

    for report in io::BufReader::new(input).lines() {
        let report = report?
            .split_ascii_whitespace()
            .map(|level| level.parse::<i32>().unwrap_or_default())
            .collect::<Vec<_>>();

        if check_report(&report) {
            count += 1;
        } else if allow_exception {
            for i in 0..report.len() {
                let mut report_with_exception = report.clone();
                report_with_exception.remove(i);

                if check_report(&report_with_exception) {
                    count += 1;
                    break;
                }
            }
        }
    }

    Ok(count)
}

fn check_report(report: &[i32]) -> bool {
    let mut prev_level: Option<i32> = None;
    let mut order = Ordering::Equal;

    for &level in report {
        if let Some(prev) = prev_level {
            let curr_order = level.cmp(&prev);
            let difference = (level - prev).abs();

            if order == Ordering::Equal {
                order = curr_order;
            }

            if curr_order != order || difference < 1 || difference > 3 {
                return false;
            }
        }

        prev_level = Some(level);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn count_safe_reports() -> Result<(), Box<dyn Error>> {
        assert_eq!(2, safe_reports(INPUT.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn count_safe_reports_with_exceptions() -> Result<(), Box<dyn Error>> {
        assert_eq!(4, safe_reports(INPUT.as_bytes(), true)?);

        Ok(())
    }
}
