use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Default, Clone)]
struct Map {
    /// All items of `obstacles` are the same length.
    ///
    /// [row][col]
    obstacles: Vec<Vec<bool>>,
    /// row, col
    start: (usize, usize),
}

impl Map {
    /// # Panics
    /// Panics if passed an out-of-bounds position.
    fn get(&self, pos: (usize, usize)) -> bool {
        *self.obstacles.get(pos.0).unwrap().get(pos.1).unwrap()
    }

    /// # Panics
    /// Panics if an already obstructed or out-of-range square is passed.
    fn clone_obstruct(&self, pos: (usize, usize)) -> Self {
        let mut dup = self.clone();
        let b = dup.obstacles.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap();
        if *b {
            panic!("tried to obstruct a blocked square, ({}, {})", pos.0, pos.1);
        }
        *b = true;
        dup
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
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
        let moved = (
            pos.0.checked_add_signed(diff.0),
            pos.1.checked_add_signed(diff.1),
        );
        let moved = match moved {
            (_, None) => return Err(()),
            (None, _) => return Err(()),
            (Some(x), Some(y)) => (x, y),
        };

        if moved.0 >= map.obstacles.len() {
            return Err(());
        }
        if moved.1 >= map.obstacles.first().map(|v| v.len()).unwrap_or(0) {
            return Err(());
        }
        Ok(moved)
    }

    fn as_bitmask(self) -> u8 {
        match self {
            Direction::Up => 1 << 0,
            Direction::Right => 1 << 1,
            Direction::Down => 1 << 2,
            Direction::Left => 1 << 3,
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
    let mut map: Map = Map {
        start: (usize::MAX, usize::MAX),
        ..Default::default()
    };

    for line in input {
        let line = line?;
        if line.is_empty() {
            break;
        }
        if let Some(pos) = line.find('^') {
            if map.start != (usize::MAX, usize::MAX) {
                panic!("multiple guard starts");
            }
            map.start = (map.obstacles.len(), pos);
        }
        map.obstacles.push(
            line.bytes()
                .map(|c| -> Result<bool> {
                    match c {
                        b'.' | b'^' => Ok(false),
                        b'#' => Ok(true),
                        _ => Err(anyhow!("invalid character {}", c)),
                    }
                })
                .collect::<Result<Vec<bool>>>()?,
        );
    }
    let exlen = map.obstacles[0].len();
    for row in map.obstacles.iter() {
        if row.len() != exlen {
            Err(anyhow!(
                "inconsistent line length: expected {} got {}",
                exlen,
                row.len()
            ))?
        }
    }
    if let (usize::MAX, usize::MAX) = map.start {
        Err(anyhow!("no start token found"))?
    }
    Ok(map)
}

fn traverse_map(map: &Map) -> Result<HashSet<(usize, usize)>, ()> {
    let row_len = map.obstacles.first().unwrap().len();
    let mut visited_dir: Vec<Vec<u8>> = (0..map.obstacles.len()).map(|_| vec![0u8; row_len]).collect();

    let mut pos: (usize, usize) = map.start;
    let mut dir = Direction::Up;

    let mut insert = |pos: (usize, usize), dir: Direction| -> bool {
        let e = visited_dir.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap();
        if *e & dir.as_bitmask() == 0 {
            *e = *e | dir.as_bitmask();
            true
        } else {
            false
        }
    };
    insert(pos, dir);

    while let Ok(next) = dir.add_diff(pos, map) {
        match map.get(next) {
            false => {
                if !insert(next, dir) {
                    // Detected loop
                    return Err(());
                }
                pos = next;
            }
            true => {
                dir = dir.right_turn();
            }
        }
    }

    let mut visited = HashSet::<(usize, usize)>::new();
    for (i, row) in visited_dir.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if *cell != 0 {
                visited.insert((i, j));
            }
        }
    }

    Ok(visited)
}

fn count_loops(map: &Map, traversed: &HashSet<(usize, usize)>) -> usize {
    let mut count = 0;

    let mut progress = 0;
    let total = traversed.len();
    for candidate in traversed {
        progress += 1;
        if progress % 100 == 0 {
            println!("checking candidate {}/{}", progress, total);
        }
        if *candidate == map.start {
            continue;
        }
        let new_map = map.clone_obstruct(*candidate);
        if let Err(()) = traverse_map(&new_map) {
            count += 1;
        }
    }

    count
}

fn main() -> Result<()> {
    let mut input = std::io::stdin().lock().lines();
    let map = build_map(&mut input)?;

    let steps = traverse_map(&map).map_err(|()| anyhow!("input map has a loop"))?;
    println!("steps: {}", steps.len());

    let loopcount = count_loops(&map, &steps);
    println!("loops: {}", loopcount);
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

        let steps = traverse_map(&map).unwrap();
        assert_eq!(steps.len(), 41);

        let possible = count_loops(&map, &steps);
        assert_eq!(possible, 6);
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
