use std::io::BufRead;
use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashSet;

#[derive(Default)]
struct Map {
    /// All items of `obstacles` are the same length.
    ///
    /// [row][col]
    obstacles: Vec<Vec<bool>>,
    /// row, col
    start: (isize, isize),
}

impl Map {
    fn get(&self, pos: (isize, isize)) -> bool {
        let (x, y): (usize, usize) = (pos.0.try_into().unwrap(), pos.1.try_into().unwrap());
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn add_diff(&self, pos: (isize, isize), map: &Map) -> Result<(isize, isize), ()> {
        let diff = match *self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        };
        if pos.0 + diff.0 < 0 || pos.1 + diff.1 < 0 {
            return Err(());
        }
        if pos.0 + diff.0 >= map.obstacles.len().try_into().unwrap() {
            return Err(());
        }
        if pos.1 + diff.1 >= map.obstacles.get(0).map(|v| v.len().try_into().ok()).flatten().unwrap_or(0) {
            return Err(());
        }
        Ok((pos.0 + diff.0, pos.1 + diff.1))
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

fn traverse_map(map: &Map) -> usize {
    let visited = HashSet::<(isize, isize)>::new();

    let mut pos: (isize, isize) = map.start;
    let mut dir = Direction::Up;

    loop {
        match dir.add_diff(pos) {
            Some(next) => {
                match map.get(next) {
                    false => {
                        visited.insert(next);
                        pos = next;
                    }
                    true => {
                        dir = dir.turn_right();
                    }
                }
            }
            None => {
                break;
            }
        }
        //let next = pos + dir.to_diff();
        //let next = (pos.0 + dir.to_diff().0, pos.1 + dir.to_diff().1);
    }

    return visited.len();
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

        assert_eq!(map.obstacles.len(), 10);
        assert_eq!(map.obstacles[0].len(), 10);
    }

    #[test]
    fn test_add_diff() {
        let mut lines = BufRead::lines(EXAMPLE1_DATA.as_bytes());
        let map = build_map(&mut lines).unwrap();

        assert_eq!(Direction::Up.add_diff((1, 5), &map), Ok((0, 5)));
        assert_eq!(Direction::Down.add_diff((1, 5), &map), Ok((2, 5)));
        assert_eq!(Direction::Right.add_diff((1, 5), &map), Ok((1, 6)));
        assert_eq!(Direction::Left.add_diff((1, 5), &map), Ok((1, 4)));

        assert_eq!(Direction::Up.add_diff((0, 5), &map), Err(()));
        assert_eq!(Direction::Down.add_diff((9, 5), &map), Err(()));
        assert_eq!(Direction::Right.add_diff((1, 9), &map), Err(()));
        assert_eq!(Direction::Left.add_diff((1, 0), &map), Err(()));
    }
}
