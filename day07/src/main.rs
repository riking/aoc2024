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
// number of possibilities: 1 << (inputs.len() - 1)

impl Eqn {
    fn is_possible(&self) -> Option<usize> {
        'choice: for choice in 0..(1 << (self.inputs.len() - 1)) {
            let mut wchoice = choice;
            let mut result = *self.inputs.first().unwrap();
            for i in 0..(self.inputs.len() - 1) {
                if wchoice & 1 == 1 {
                    #[cfg(test)]
                    println!("mul {} {}", result, self.inputs[i + 1]);
                    if let Some(n) = result.checked_mul(self.inputs[i + 1]) {
                        result = n;
                    } else {
                        continue 'choice;
                    }
                } else {
                    #[cfg(test)]
                    println!("add {} {}", result, self.inputs[i + 1]);
                    result += self.inputs[i + 1];
                }
                wchoice >>= 1;
            }
            if result == self.result {
                return Some(choice);
            }
        }
        None
    }
}

fn main() -> Result<(), anyhow::Error> {
    let input = std::io::stdin().lock().lines();

    let mut total = 0;
    for line in input {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let eqn = line.parse::<Eqn>()?;
        if let Some(_) = eqn.is_possible() {
            total += eqn.result;
        }
    }
    println!("total: {total}");
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
        assert_eq!("190: 10 19".parse::<Eqn>().unwrap().is_possible(), Some(1));
        let multi = "3267: 81 40 27".parse::<Eqn>().unwrap().is_possible();
        assert!(matches!(multi, Some(1) | Some(2)));
        assert_eq!(
            "292: 11 6 16 20".parse::<Eqn>().unwrap().is_possible(),
            Some(2)
        );
        for s in [
            "83: 17 5",
            "156: 15 6",
            "7290: 6 8 6 15",
            "161011: 16 10 13",
            "192: 17 8 14",
            "21037: 9 7 18 13",
        ] {
            assert_eq!(s.parse::<Eqn>().unwrap().is_possible(), None);
        }
    }

    #[test]
    fn example1() {
        let mut total = 0;
        for line in EXAMPLE1_DATA.lines() {
            let eqn = line.unwrap().parse::<Eqn>()?;
            if let Some(_) = eqn.is_possible() {
                total += eqn.result;
            }
        }
        assert_eq!(total, 3749);
    }
}
