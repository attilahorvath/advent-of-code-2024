use std::error::Error;
use std::io::{self, BufRead, Read};

struct Equation {
    value: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn is_valid(&self, use_concatenation: bool) -> bool {
        self.fold(self.numbers[0], &self.numbers[1..], use_concatenation) == self.value
    }

    fn fold(&self, total: u64, rest: &[u64], use_concatenation: bool) -> u64 {
        if total > self.value || rest.len() == 0 {
            return total;
        }

        let o1 = self.fold(total + rest[0], &rest[1..], use_concatenation);
        let o2 = self.fold(total * rest[0], &rest[1..], use_concatenation);

        if use_concatenation {
            let t =
                (total * 10u64.pow((rest[0] as f64).log(10.0).floor() as u32 + 1)) as u64 + rest[0];
            let o3 = self.fold(t, &rest[1..], use_concatenation);

            if o1 == self.value {
                o1
            } else if o2 == self.value {
                o2
            } else {
                o3
            }
        } else {
            if o1 == self.value {
                o1
            } else {
                o2
            }
        }
    }
}

pub fn sum_results(input: impl Read, use_concatenation: bool) -> Result<u64, Box<dyn Error>> {
    let mut equations = vec![];

    for line in io::BufReader::new(input).lines() {
        let line = line?;

        let mut parts = line.split(": ");
        let value = parts.next().unwrap_or_default().parse()?;
        let numbers = parts
            .next()
            .unwrap_or_default()
            .split_ascii_whitespace()
            .map(|number| number.parse().unwrap_or_default())
            .collect();

        equations.push(Equation { value, numbers });
    }

    Ok(equations
        .iter()
        .filter(|equation| equation.is_valid(use_concatenation))
        .map(|equation| equation.value)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn sum_calibration_results() -> Result<(), Box<dyn Error>> {
        assert_eq!(3749, sum_results(INPUT.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn sum_calibration_results_with_concatenations() -> Result<(), Box<dyn Error>> {
        assert_eq!(11387, sum_results(INPUT.as_bytes(), true)?);

        Ok(())
    }
}
