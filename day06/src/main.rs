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
    start: (usize, usize),
}

impl Map {
    fn get(&self, pos: (usize, usize)) -> bool {
        todo!()
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn add_diff(&self, pos: (usize, usize), map: &Map) -> Result<(usize, usize), ()> {
        let diff: (isize, isize) = match *self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        };
        let moved = (pos.0.checked_add_signed(diff.0), pos.1.checked_add_signed(diff.1));
        let moved = match moved {
            (_, None) => return Err(()),
            (None, _) => return Err(()),
            (Some(x), Some(y)) => (x, y),
        };

        if moved.0 >= map.obstacles.len() {
            return Err(());
        }
        if moved.1 >= map.obstacles.get(0).map(|v| v.len()).unwrap_or(0) {
            return Err(());
        }
        Ok(moved)
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
    let mut map: Map = Map { start: (usize::MAX, usize::MAX), ..Default::default() };

    for line in input {
        let line = line?;
        if line.is_empty() { break; }
        if let Some(pos) = line.find('^') {
            if map.start != (usize::MAX, usize::MAX) {
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
    if let (usize::MAX, usize::MAX) = map.start {
        Err(anyhow!("no start token found"))?
    }
    Ok(map)
}

fn traverse_map(map: &Map) -> usize {
    let mut visited = HashSet::<(usize, usize)>::new();

    let mut pos: (usize, usize) = map.start;
    let mut dir = Direction::Up;

    loop {
        match dir.add_diff(pos, &*map) {
            Ok(next) => {
                match map.get(next) {
                    false => {
                        visited.insert(next);
                        pos = next;
                    }
                    true => {
                        dir = dir.right_turn();
                    }
                }
            }
            Err(()) => {
                break;
            }
        }
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
