use std::cmp::Ordering;
use std::io::BufRead;

fn is_safe(line: String) -> Result<bool, anyhow::Error>{
    let nums = line
        .split(' ')
        .map(|n| n.parse())
        .collect::<Result<Vec<usize>, _>>()?;
    //let cmps = nums.array_windows::<[usize; 2]>().map(|(a, b)| a.cmp(b)).collect::<Vec<std::cmp::Ordering>>();
    for skip_i in 0..=nums.len() {
        let nums = if skip_i == nums.len() {
            nums.clone()
        } else {
            let mut c = nums.clone();
            c.remove(skip_i);
            c
        };
        let cmps = {
            let mut cmps = Vec::<Option<std::cmp::Ordering>>::new();
            for i in 0..nums.len() - 1 {
                let a = nums[i];
                let b = nums[i + 1];
                if ((b as isize) - (a as isize)).abs() > 3 {
                    cmps.push(None);
                } else {
                    cmps.push(Some(a.cmp(&b)));
                }
            }
            cmps
        };
        let is_safe = cmps.iter().all(|o| *o == Some(Ordering::Greater))
            || cmps.iter().all(|o| *o == Some(Ordering::Less));
        if is_safe {
            println!("{:?} {:?}, {} {}", nums, cmps, skip_i, is_safe);
            return Ok(true);
        }
    }
    println!("{:?} {}", nums, false);
    Ok(false)
}

fn main() -> Result<(), anyhow::Error> {
    let input = std::io::stdin().lock().lines();

    let mut safe_count = 0;
    for line in input {
        let line = line?;
        if line.is_empty() {
            break;
        }
        if is_safe(line)? {
            safe_count += 1;
        }
        //let nums = line.split(' ').map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
    }
    println!("Hello, world! {}", safe_count);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::is_safe;

    #[test]
    fn non_numbers() {
        let wrong = is_safe("1 2 3 4 five".to_string());
        assert!(wrong.is_err());
    }

    #[test]
    fn examples() {
        assert_eq!(is_safe("7 6 4 2 1".to_string()).unwrap(), true);
        assert_eq!(is_safe("1 2 7 8 9".to_string()).unwrap(), false);
        assert_eq!(is_safe("9 7 6 2 1".to_string()).unwrap(), false);
        assert_eq!(is_safe("1 3 2 4 5".to_string()).unwrap(), true);
        assert_eq!(is_safe("8 6 4 4 1".to_string()).unwrap(), true);
        assert_eq!(is_safe("1 3 6 7 9".to_string()).unwrap(), true);
    }
}
