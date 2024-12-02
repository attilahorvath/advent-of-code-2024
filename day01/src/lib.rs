use std::error::Error;
use std::io::{self, BufRead, Read};

pub fn read_lists(input: impl Read) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut lists = (vec![], vec![]);

    for line in io::BufReader::new(input).lines() {
        let line = line?;
        let mut parts = line.split_ascii_whitespace();

        lists
            .0
            .push(parts.next().unwrap_or_default().parse().unwrap_or_default());

        lists
            .1
            .push(parts.next().unwrap_or_default().parse().unwrap_or_default());
    }

    Ok(lists)
}

pub fn total_distance(lists: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut lists = lists.clone();

    lists.0.sort();
    lists.1.sort();

    lists
        .0
        .iter()
        .zip(lists.1.iter())
        .map(|(left, right)| (right - left).abs())
        .sum()
}

pub fn similarity_score(lists: &(Vec<i32>, Vec<i32>)) -> usize {
    lists
        .0
        .iter()
        .map(|&number| number as usize * lists.1.iter().filter(|&&i| i == number).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn calculate_total_distance() -> Result<(), Box<dyn Error>> {
        let lists = read_lists(INPUT.as_bytes())?;

        assert_eq!(11, total_distance(&lists));

        Ok(())
    }

    #[test]
    fn calculate_similarity_score() -> Result<(), Box<dyn Error>> {
        let lists = read_lists(INPUT.as_bytes())?;

        assert_eq!(31, similarity_score(&lists));

        Ok(())
    }
}
