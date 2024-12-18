use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, BufRead, Read};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn positions(input: impl Read, add_extra: bool) -> Result<usize, Box<dyn Error>> {
    let mut map = vec![];
    let mut start = (0, 0);

    for (y, line) in io::BufReader::new(input).lines().enumerate() {
        let line = line?;

        let mut row = vec![];

        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => false,
                '^' => {
                    start = (x as i32, y as i32);
                    true
                }
                _ => true,
            };

            row.push(tile);
        }

        map.push(row);
    }

    Ok(walk(&map, start, add_extra, None).unwrap_or_default())
}

fn walk(
    map: &Vec<Vec<bool>>,
    start: (i32, i32),
    add_extra: bool,
    extra: Option<(i32, i32)>,
) -> Option<usize> {
    let mut position = start;
    let mut direction = Direction::Up;

    let mut visited: HashMap<(i32, i32), HashSet<Direction>> = HashMap::new();
    let mut extras: HashSet<(i32, i32)> = HashSet::new();

    loop {
        if !visited.entry(position).or_default().insert(direction) {
            return None;
        }

        let mut new_position = position.clone();

        match direction {
            Direction::Up => new_position.1 -= 1,
            Direction::Down => new_position.1 += 1,
            Direction::Left => new_position.0 -= 1,
            Direction::Right => new_position.0 += 1,
        }

        if new_position.0 < 0
            || new_position.0 == map[0].len() as i32
            || new_position.1 < 0
            || new_position.1 == map.len() as i32
        {
            if add_extra {
                return Some(extras.len());
            } else {
                return Some(visited.len());
            }
        }

        if map[new_position.1 as usize][new_position.0 as usize]
            && (extra.is_none() || extra.unwrap_or_default() != new_position)
        {
            position = new_position;

            if add_extra
                && extra.is_none()
                && !extras.contains(&position)
                && walk(map, start, add_extra, Some(position)).is_none()
            {
                extras.insert(position);
            }
        } else {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn count_positions() -> Result<(), Box<dyn Error>> {
        assert_eq!(41, positions(INPUT.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn count_positions_with_extra_obstacles() -> Result<(), Box<dyn Error>> {
        assert_eq!(6, positions(INPUT.as_bytes(), true)?);

        Ok(())
    }
}
