use regex::Regex;
use std::io::BufRead;

const DO: &'static str = "do()";
const DONT: &'static str = "don't()";

fn find_muls(input: &[String], part2: bool) -> anyhow::Result<Vec<(usize, usize)>> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)")
        .expect("regex compile fail");
    let mut enabled = true;
    let mut result = vec![];

    for line in input {
    for c in re.captures_iter(&*line) {
        let m = c.get(0).unwrap().as_str(); // Unwrap: Guaranteed to be present for index 0
        if m == DO {
            enabled = true;
            continue;
        } else if m == DONT {
            enabled = false;
            continue;
        }
        let Some(b) = c.get(2) else {
            panic!("regex matches missing after filtering commands: {}", m);
        };
        let b = b.as_str();
        let a = c.get(1).unwrap().as_str();
        if enabled || !part2 {
            result.push((a.parse()?, b.parse()?))
        }
    }
    }
    Ok(result)
}

fn main() -> anyhow::Result<()> {
    let part2 = true;
    let input = std::io::stdin().lock().lines();
    let input = input.collect::<Result<Vec<String>, _>>()?;

    let pairs = find_muls(&input, part2)?;

    println!(
        "sum {}",
        pairs
            .iter()
            .map(|(a, b)| a.checked_mul(*b).expect("overflow"))
            .sum::<usize>()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example() {
        assert_eq!(
            find_muls(
                &[r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()],
                false
            )
            .unwrap(),
            vec![(2, 4), (5, 5), (11, 8), (8, 5)]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_muls(
                &[r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()],
                true
            )
            .unwrap(),
            vec![(2, 4), (8, 5)]
        );
    }
}
