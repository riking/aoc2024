use std::cmp::Ordering;
use std::io::BufRead;

fn main() -> Result<(), anyhow::Error> {
    let input = std::io::stdin().lock().lines();

    let mut safe_count = 0;
    'next_line: for line in input {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        //let nums = line.split(' ').map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, _>>()?;
        let nums = line
            .split(' ')
            .map(|n| n.parse())
            .collect::<Result<Vec<usize>, _>>()?;
        //let cmps = nums.array_windows::<[usize; 2]>().map(|(a, b)| a.cmp(b)).collect::<Vec<std::cmp::Ordering>>();
        for skip_i in 0..=nums.len() {
            let nums = if skip_i == nums.len() {
                nums.clone();
            } else {
                let mut c = nums.clone();
                c.remove(skip_i);
                c;
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
                safe_count += 1;
                println!("{:?} {:?}, {} {}", nums, cmps, skip_i, is_safe);
                continue 'next_line;
            }
        }
        println!("{:?} {}", nums, false);
    }
    println!("Hello, world! {}", safe_count);
    Ok(())
}
