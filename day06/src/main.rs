use std::io::BufRead;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashSet;

#[derive(Default)]
struct Map {
    /// [row][col]
    obstacles: Vec<Vec<bool>>,
    /// row, col
    start: (isize, isize),
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_diff(&self) -> (isize, isize) {
        match *self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        }
    }

    fn right_turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn build_map<B: BufRead>(input: &mut std::io::Lines<B>) -> Result<Map> {
    let mut map: Map = Map { start: (isize::MAX, isize::MAX), ..Default::default() };

    for line in input {
        let line = line?;
        if line.is_empty() { break; }
        if let Some(pos) = line.find('^') {
            if map.start != (isize::MAX, isize::MAX) {
                panic!("multiple guard starts");
            }
            map.start = (map.obstacles.len().try_into().expect("not out of range"), pos.try_into().unwrap());
        }
        map.obstacles.push(line.bytes().map(|c| -> Result<bool> {
            match c {
                b'.' | b'^' => Ok(false),
                b'#' => Ok(true),
                _ => Err(anyhow!("invalid character {}", c)),
            }
        }).collect::<Result<Vec<bool>>>()?);
    }
    let exlen = map.obstacles[0].len();
    for row in map.obstacles.iter() {
        if row.len() != exlen {
            Err(anyhow!("inconsistent line length: expected {} got {}", exlen, row.len()))?
        }
    }
    if let (isize::MAX, isize::MAX) = map.start {
        Err(anyhow!("no start token found"))?
    }
    Ok(map)
}

fn traverse_map(map: &Map) -> () {
    let visited = HashSet::<(usize, usize)>::new();

    let pos: (isize, isize) = map.start;
    let dir = Direction::Up;

}

fn main() -> Result<()> {
    let mut input = std::io::stdin().lock().lines();
    let map = build_map(&mut input)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1_DATA: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn example() {
        let mut lines = BufRead::lines(EXAMPLE1_DATA.as_bytes());
        let map = build_map(&mut lines).unwrap();
    }
}
