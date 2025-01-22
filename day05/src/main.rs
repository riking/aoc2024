use anyhow::anyhow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

type Orderings = HashMap<usize, HashSet<usize>>;

fn build_ordering<B: BufRead>(input: &mut std::io::Lines<B>) -> anyhow::Result<Orderings> {
    let mut res: Orderings = HashMap::new();
    for line in input {
        let line = line?;
        if line == "" {
            return Ok(res);
        }
        let (left, right) = line
            .split_once('|')
            .ok_or_else(|| anyhow!("bad input format"))?;
        let (left, right) = (left.parse()?, right.parse()?);

        res.entry(left)
            .or_insert_with(Default::default)
            .insert(right);
    }
    Err(anyhow!("unexpected EOF"))
}

fn check_job(line: &str, order: &Orderings) -> anyhow::Result<(bool, usize)> {
    let mut seen: HashSet<usize> = Default::default();
    let line = line
        .split(',')
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    let 0 = (line.len() - 1).rem_euclid(2) else {
        panic!("non-odd page count: {}", line.len());
    };
    let mid = (line.len() - 1) / 2;
    let mid = line[mid].clone();
    for page in line {
        let page = page;

        if let Some(banned_precedents) = order.get(&page) {
            if !banned_precedents.is_disjoint(&seen) {
                return Ok((false, mid));
            }
        }
        seen.insert(page);
    }
    Ok((true, mid))
}

fn fix(line: &str, orderings: &Orderings) -> anyhow::Result<usize> {
    let mut line = line
        .split(',')
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    line.sort_by(|a, b| {
        if let Some(m) = orderings.get(a) {
            if m.contains(b) {
                return core::cmp::Ordering::Less;
            }
        }
        if let Some(m) = orderings.get(b) {
            if m.contains(a) {
                return core::cmp::Ordering::Greater;
            }
        }
        return a.cmp(b);
    });

    let mid = (line.len() - 1) / 2;
    let mid = line[mid].clone();
    Ok(mid)
}

fn main() -> Result<(), anyhow::Error> {
    let mut input = std::io::stdin().lock().lines();
    let orderings = build_ordering(&mut input)?;

    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in input {
        let line = line?;
        let (ok, midpage) = check_job(&line, &orderings)?;
        if ok {
            sum1 += midpage;
        } else {
            let midpage = fix(&line, &orderings)?;
            sum2 += midpage;
        }
    }
    println!("part1: {}", sum1);
    println!("part2: {}", sum2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1_DATA: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn example1() {
        let mut lines = BufRead::lines(EXAMPLE1_DATA.as_bytes());
        let ordering = build_ordering(&mut lines).unwrap();
        println!("ordering: {:?}", ordering);
        assert_eq!(ordering.len(), 6);
        assert_eq!(ordering[&97].len(), 6);
        assert_eq!(ordering[&29].len(), 1);
    }

    #[test]
    fn example2() {
        let mut lines = BufRead::lines(EXAMPLE1_DATA.as_bytes());
        let ordering = build_ordering(&mut lines).unwrap();

        assert_eq!(check_job("75,47,61,53,29", &ordering).unwrap(), (true, 61));
        assert_eq!(check_job("97,61,53,29,13", &ordering).unwrap(), (true, 53));
        assert_eq!(check_job("75,29,13", &ordering).unwrap(), (true, 29));
        assert_eq!(check_job("75,97,47,61,53", &ordering).unwrap(), (false, 47));
        assert_eq!(check_job("61,13,29", &ordering).unwrap(), (false, 13));
        assert_eq!(check_job("97,13,75,29,47", &ordering).unwrap(), (false, 75));
    }

    #[test]
    fn example3() {
        let mut lines = BufRead::lines(EXAMPLE1_DATA.as_bytes());
        let ordering = build_ordering(&mut lines).unwrap();

        assert_eq!(fix("75,97,47,61,53", &ordering).unwrap(), 47);
        assert_eq!(fix("61,13,29", &ordering).unwrap(), 29);
        assert_eq!(fix("97,13,75,29,47", &ordering).unwrap(), 47);
    }
}
