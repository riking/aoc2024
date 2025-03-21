use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Default, Clone)]
struct Map {
    /// All items of `obstacles` are the same length.
    ///
    /// [row][col]
    antennas: Vec<Vec<u8>>,
}

fn build_map<B: BufRead>(input: &mut std::io::Lines<B>) -> Result<Map> {
    let mut map: Map = Default::default();

    for line in input {
        let line = line?;
        if line.is_empty() {
            break;
        }
        map.antennas.push(
            line.bytes()
                .map(|c| -> Result<u8> {
                    match c {
                        b'.' => Ok(0),
                        b'#' => Ok(0),
                        c if c.is_ascii_alphanumeric() => Ok(c),
                        _ => Err(anyhow!("invalid map char {}", c)),
                    }
                })
                .collect::<Result<Vec<u8>>>()?,
        );
    }
    let exlen = map.antennas.first().unwrap().len();
    for row in map.antennas.iter() {
        if row.len() != exlen {
            Err(anyhow!(
                "inconsistent line length: expected {} got {}",
                exlen,
                row.len()
            ))?
        }
    }
    Ok(map)
}

fn antenna_ids(map: &Map) -> HashSet<u8> {
    let mut result = HashSet::new();
    map.antennas
        .iter()
        .for_each(|row| result.extend(row.iter().filter(|c| **c != 0)));
    result
}

fn single_antinodes(map: &Map, a: (usize, usize), b: (usize, usize)) -> Option<(usize, usize)> {
    let na = (
        a.0.wrapping_sub(b.0.wrapping_sub(a.0)),
        a.1.wrapping_sub(b.1.wrapping_sub(a.1)),
    );
    if na.0 > map.antennas.len() {
        return None;
    }
    if na.1 > map.antennas.first().unwrap().len() {
        return None;
    }
    Some(na)
}

fn antinodes(map: &Map) -> Vec<Vec<bool>> {
    let row_len = map.antennas.first().unwrap().len();
    let mut antinodes_data: Vec<Vec<bool>> = (0..map.antennas.len())
        .map(|_| vec![false; row_len])
        .collect();

    let mut set = |(x, y): (usize, usize)| {
        *antinodes_data.get_mut(x).unwrap().get_mut(y).unwrap() = true;
    };

    for antenna_id in antenna_ids(map) {
        let matches = map
            .antennas
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter().enumerate().map(
                    move |(j, c)| {
                        if *c == antenna_id {
                            Some((i, j))
                        } else {
                            None
                        }
                    },
                )
            })
            .flatten()
            .flatten()
            .collect::<Vec<(usize, usize)>>();
        use itertools::Itertools;

        for cvec in matches.iter().combinations(2) {
            let [&lpos, &rpos] = cvec.try_into().unwrap();
            let na = single_antinodes(map, lpos, rpos);
            if let Some(na) = na {
                set(na);
            }
            let nb = single_antinodes(map, rpos, lpos);
            if let Some(nb) = nb {
                set(nb);
            }
        }
    }
    antinodes_data
}

fn main() -> Result<(), anyhow::Error> {
    let mut input = std::io::stdin().lock().lines();
    let map = build_map(&mut input)?;
    let antinodes = antinodes(&map);

    let num_antinodes: u32 = antinodes
        .iter()
        .map(|v| v.iter().map(|b| if *b { 1 } else { 0 }).sum::<u32>())
        .sum();
    println!("{}", num_antinodes);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1_DATA: &str = r"............
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
    const EXAMPLE1_ANTINODES: &str = r"......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....#.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.";

    const EXAMPLE2_DATA: &str = r"..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......A...
..........
..........";
    #[test]
    fn example1() {
        let mut lines = BufRead::lines(EXAMPLE1_DATA.as_bytes());
        let map = build_map(&mut lines).unwrap();
        assert_eq!(antenna_ids(&map), HashSet::from([b'0', b'A']));

        let expected_antinodes = EXAMPLE1_ANTINODES
            .as_bytes()
            .lines()
            .map(|line| {
                line.unwrap()
                    .as_bytes()
                    .iter()
                    .map(|c| *c == b'#')
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();
        let antinodes = antinodes(&map);
        assert_eq!(antinodes, expected_antinodes);
        let num_antinodes: u32 = antinodes
            .iter()
            .map(|v| v.iter().map(|b| if *b { 1 } else { 0 }).sum::<u32>())
            .sum();
        assert_eq!(num_antinodes, 14);
    }

    #[test]
    fn example2() {
        let mut lines = BufRead::lines(EXAMPLE2_DATA.as_bytes());
        let map = build_map(&mut lines).unwrap();
        assert_eq!(antenna_ids(&map), HashSet::from([b'a', b'A']));

        let expected_antinodes = {
            let mut expected_antinodes = EXAMPLE2_DATA
                .as_bytes()
                .lines()
                .map(|line| {
                    line.unwrap()
                        .as_bytes()
                        .iter()
                        .map(|c| *c == b'#')
                        .collect::<Vec<bool>>()
                })
                .collect::<Vec<Vec<bool>>>();
            expected_antinodes[7][6] = true;
            expected_antinodes
        };
        let antinodes = antinodes(&map);
        assert_eq!(antinodes, expected_antinodes);
    }
}
