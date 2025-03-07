use anyhow::Result;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Eqn {
    result: usize,
    inputs: Vec<usize>,
}

impl FromStr for Eqn {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((result, inputs)) = s.split_once(':') else {
            return Err(anyhow::anyhow!("no colon"));
        };
        let inputs = inputs
            .trim_start()
            .split_ascii_whitespace()
            .map(|st| st.parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;
        let result = result.parse::<usize>()?;
        Ok(Eqn { result, inputs })
    }
}

// encoding: first two operands = least sig bit
// 0 = add
// 1 = mul
// number of possibilities: 1 << (2 * inputs.len() - 2)

enum Op {
    Plus = 0,
    Mul = 1,
    Concat = 2,
    Err = 3,
}

impl From<usize> for Op {
    fn from(n: usize) -> Op {
        match n {
            0 => Op::Plus,
            1 => Op::Mul,
            2 => Op::Concat,
            3 => Op::Err,
            _ => panic!("Op: out of range"),
        }
    }
}

impl Eqn {
    fn is_possible(&self) -> Option<usize> {
        'choice: for choice in 0..(1 << (2 * self.inputs.len() - 2)) {
            let mut wchoice = choice;
            let mut result = *self.inputs.first().unwrap();
            for i in 0..(self.inputs.len() - 1) {
                match Op::from(wchoice & 3) {
                    Op::Mul => {
                        #[cfg(test)]
                        println!("mul {} {}", result, self.inputs[i + 1]);
                        if let Some(n) = result.checked_mul(self.inputs[i + 1]) {
                            result = n;
                        } else {
                            continue 'choice;
                        }
                    }
                    Op::Plus => {
                        #[cfg(test)]
                        println!("add {} {}", result, self.inputs[i + 1]);
                        if let Some(n) = result.checked_add(self.inputs[i + 1]) {
                            result = n;
                        } else {
                            continue 'choice;
                        }
                    }
                    Op::Concat => {
                        let s = format!("{}{}", result, self.inputs[i+1]);
                        if let Ok(n) = s.parse::<usize>() {
                            result = n;
                        } else {
                            continue 'choice;
                        }
                    }
                    Op::Err => continue 'choice,
                }
                wchoice >>= 2;
            }
            if result == self.result {
                return Some(choice);
            }
        }
        None
    }
}

fn used_concat(mut choice: usize) -> bool {
    while choice != 0 {
        if choice & 3 == (Op::Concat as usize) {
            return true
        }
        choice >>= 2;
    }
    false
}

fn main() -> Result<(), anyhow::Error> {
    let input = std::io::stdin().lock().lines();

    let mut total1 = 0;
    let mut total2 = 0;
    for line in input {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let eqn = line.parse::<Eqn>()?;
        if let Some(choice) = eqn.is_possible() {
            total2 += eqn.result;
            if !used_concat(choice) {
                total1 += eqn.result;
            }
        }
    }
    println!("total part1: {total1}");
    println!("total part2: {total2}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1_DATA: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn example() {
        assert_eq!(
            "21037: 9 7 18 13".parse::<Eqn>().unwrap(),
            Eqn {
                result: 21037,
                inputs: vec![9, 7, 18, 13],
            }
        );
        assert_eq!("190: 10 19".parse::<Eqn>().unwrap().is_possible(), Some(Op::Mul as usize));
        let multi = "3267: 81 40 27".parse::<Eqn>().unwrap().is_possible();
        const CASE1: usize = Op::Mul as usize;
        const CASE2: usize = (Op::Mul as usize) << 2;
        assert!(matches!(multi, Some(CASE1) | Some(CASE2)));
        assert_eq!(
            "292: 11 6 16 20".parse::<Eqn>().unwrap().is_possible(),
            Some((Op::Mul as usize) << 2)
        );
        assert_eq!(
            "156: 15 6".parse::<Eqn>().unwrap().is_possible(),
            Some((Op::Concat as usize) << 0)
        );
        assert_eq!(
            "7290: 6 8 6 15".parse::<Eqn>().unwrap().is_possible(),
            Some((Op::Mul as usize) << 0 | (Op::Concat as usize) << 2 | (Op::Mul as usize) << 4)
        );
        assert_eq!(
            "192: 17 8 14".parse::<Eqn>().unwrap().is_possible(),
            Some((Op::Concat as usize) << 0)
        );
        for s in [
            "83: 17 5",
            "161011: 16 10 13",
            "21037: 9 7 18 13",
        ] {
            assert_eq!(s.parse::<Eqn>().unwrap().is_possible(), None);
        }
    }

    #[test]
    fn example1() {
        let mut total1 = 0;
        let mut total2 = 0;
        for line in EXAMPLE1_DATA.lines() {
            let eqn = line.parse::<Eqn>().unwrap();
            if let Some(choice) = eqn.is_possible() {
                total2 += eqn.result;
                if !used_concat(choice) {
                    total1 += eqn.result;
                }
            }
        }
        assert_eq!(total1, 3749);
        assert_eq!(total2, 11387);
    }
}
