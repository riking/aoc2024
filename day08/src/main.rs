use std::collections::HashSet;
use anyhow::{anyhow, Result};
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
    map.antennas.iter()
        .for_each(|row| { result.extend(row.iter().filter(|c| **c != 0))});
    result
}

fn antinodes(map: &Map) -> Vec<Vec<bool>> {
    let row_len = map.antennas.first().unwrap().len();
    let mut antinodes_data: Vec<Vec<bool>> = (0..map.antennas.len()).map(|_| vec![false; row_len]).collect();

    for antenna_id in antenna_ids(map) {

    }
}

fn main() -> Result<(), anyhow::Error> {
    let mut input = std::io::stdin().lock().lines();
    let map = build_map(&mut input)?;

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
    }

    #[test]
    fn example2() {
        let mut lines = BufRead::lines(EXAMPLE2_DATA.as_bytes());
        let map = build_map(&mut lines).unwrap();
        assert_eq!(antenna_ids(&map), HashSet::from([b'a', b'A']));
    }
}
