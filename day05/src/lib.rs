use std::error::Error;
use std::io::{self, BufRead, Read};

pub fn sum_middle_pages(input: impl Read, fix: bool) -> Result<usize, Box<dyn Error>> {
    let mut sum = 0;

    let mut rules = vec![];
    let mut updates = vec![];
    let mut rules_read = false;

    for line in io::BufReader::new(input).lines() {
        let line = line?;

        if line.is_empty() {
            rules_read = true;

            continue;
        }

        if !rules_read {
            rules.push(
                line.split("|")
                    .map(|page| page.parse::<usize>().unwrap_or_default())
                    .collect::<Vec<_>>(),
            );
        } else {
            updates.push(
                line.split(",")
                    .map(|page| page.parse::<usize>().unwrap_or_default())
                    .collect::<Vec<_>>(),
            );
        }
    }

    for update in updates {
        let mut sorted = update.clone();

        sorted.sort_by(|a, b| {
            if rules
                .iter()
                .find(|rule| rule[0] == *a && rule[1] == *b)
                .is_some()
            {
                std::cmp::Ordering::Less
            } else if rules
                .iter()
                .find(|rule| rule[0] == *b && rule[1] == *a)
                .is_some()
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        if (!fix && sorted == update) || (fix && sorted != update) {
            sum += sorted[sorted.len() / 2];
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn sum_correct_middle_pages() -> Result<(), Box<dyn Error>> {
        assert_eq!(143, sum_middle_pages(INPUT.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn sum_incorrect_middle_pages() -> Result<(), Box<dyn Error>> {
        assert_eq!(123, sum_middle_pages(INPUT.as_bytes(), true)?);

        Ok(())
    }
}
