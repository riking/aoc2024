use regex::Regex;
use std::io::BufRead;

fn find_muls(input: &str) -> anyhow::Result<Vec<(usize, usize)>> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("regex compile fail");

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| (a.parse(), b.parse()))
        .map(|(a, b)| -> anyhow::Result<(usize, usize)> { Ok((a?, b?)) })
        .collect()
}

fn main() -> anyhow::Result<()> {
    let input = std::io::stdin().lock().lines();

    let mut pairs = vec![];
    for line in input {
        pairs.extend(find_muls(&(line?))?);
    }

    println!(
        "sum {}",
        pairs.iter().map(|(a, b)| a.checked_mul(*b).expect("overflow")).sum::<usize>()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example() {
        assert_eq!(
            find_muls(r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))").unwrap(),
            vec![(2, 4), (5, 5), (11, 8), (8, 5)]
        );
    }
}
