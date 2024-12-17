use std::error::Error;
use std::io::{self, BufRead, Read};

pub fn count_words(input: impl Read, word: &str, x_shape: bool) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    let mut grid = vec![];

    for line in io::BufReader::new(input).lines() {
        grid.push(line?.chars().collect::<Vec<_>>());
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if x_shape {
                if x <= row.len() - word.len()
                    && y <= grid.len() - word.len()
                    && (is_word_at(&grid, word, x, y, 1, 1)
                        || is_word_at(&grid, word, x + word.len() - 1, y + word.len() - 1, -1, -1))
                    && (is_word_at(&grid, word, x + word.len() - 1, y, -1, 1)
                        || is_word_at(&grid, word, x, y + word.len() - 1, 1, -1))
                {
                    count += 1;
                }
            } else {
                count += count_words_at(&grid, word, x, y);
            }
        }
    }

    Ok(count)
}

fn count_words_at(grid: &Vec<Vec<char>>, word: &str, x: usize, y: usize) -> usize {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            if is_word_at(grid, word, x, y, dx, dy) {
                count += 1;
            }
        }
    }

    count
}

fn is_word_at(grid: &Vec<Vec<char>>, word: &str, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    let mut cx = x as i32;
    let mut cy = y as i32;

    for (i, c) in word.chars().enumerate() {
        if grid[cy as usize][cx as usize] != c {
            return false;
        }

        if i == word.len() - 1 {
            return true;
        }

        if (cx == 0 && dx < 0)
            || (cy == 0 && dy < 0)
            || (cx as usize == grid[cy as usize].len() - 1 && dx > 0)
            || (cy as usize == grid.len() - 1 && dy > 0)
        {
            return false;
        }

        cx += dx;
        cy += dy;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn count_xmas() -> Result<(), Box<dyn Error>> {
        assert_eq!(18, count_words(INPUT.as_bytes(), "XMAS", false)?);

        Ok(())
    }

    #[test]
    fn count_x_mas() -> Result<(), Box<dyn Error>> {
        assert_eq!(9, count_words(INPUT.as_bytes(), "MAS", true)?);

        Ok(())
    }
}
