use std::io::BufRead;

fn main() -> Result<(), anyhow::Error> {
    let input = std::io::stdin().lock().lines();

    for line in input {
        let line = line?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
    }
}
