use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, BufRead, Read};

pub fn antinode_locations(input: impl Read, resonants: bool) -> Result<usize, Box<dyn Error>> {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes = HashSet::new();

    let mut width = 0;
    let mut height = 0;

    for (y, line) in io::BufReader::new(input).lines().enumerate() {
        let line = line?;

        height += 1;

        if width == 0 {
            width = line.len() as i32;
        }

        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().push((x as i32, y as i32));
            }
        }
    }

    for frequency in antennas.keys() {
        for (index, a) in antennas[frequency].iter().enumerate() {
            for b in antennas[frequency][(index + 1)..].iter() {
                let d = (b.0 - a.0, b.1 - a.1);
                let mut antinode = a.clone();

                loop {
                    if !resonants {
                        antinode.0 -= d.0;
                        antinode.1 -= d.1;
                    }

                    if antinode.0 >= 0
                        && antinode.1 >= 0
                        && antinode.0 < width
                        && antinode.1 < height
                    {
                        antinodes.insert(antinode);
                    } else {
                        break;
                    }

                    if !resonants {
                        break;
                    }

                    antinode.0 -= d.0;
                    antinode.1 -= d.1;
                }

                antinode = b.clone();

                loop {
                    if !resonants {
                        antinode.0 += d.0;
                        antinode.1 += d.1;
                    }

                    if antinode.0 >= 0
                        && antinode.1 >= 0
                        && antinode.0 < width
                        && antinode.1 < height
                    {
                        antinodes.insert(antinode);
                    } else {
                        break;
                    }

                    if !resonants {
                        break;
                    }

                    antinode.0 += d.0;
                    antinode.1 += d.1;
                }
            }
        }
    }

    Ok(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn count_unique_antinode_locations() -> Result<(), Box<dyn Error>> {
        assert_eq!(14, antinode_locations(INPUT.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn count_unique_antinode_locations_using_resonants() -> Result<(), Box<dyn Error>> {
        assert_eq!(34, antinode_locations(INPUT.as_bytes(), true)?);

        Ok(())
    }
}
