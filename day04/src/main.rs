use std::io::Read;

const L_X: u8 = b'X';
const L_M: u8 = b'M';
const L_A: u8 = b'A';
const L_S: u8 = b'S';

const FORWARD: u32 =
    ((L_X as u32) << 24) | ((L_M as u32) << 16) | ((L_A as u32) << 8) | (L_S as u32);
const REVERSE: u32 =
    ((L_S as u32) << 24) | ((L_A as u32) << 16) | ((L_M as u32) << 8) | (L_X as u32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Orient {
    Horiz(bool),
    Vert(bool),
    DiagForward(bool),
    DiagBack(bool),
    XMas(bool, bool),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Match {
    row: usize,
    col: usize,
    orient: Orient,
}

#[derive(Clone, Debug)]
struct Crawler(u32);

impl Crawler {
    fn push(&mut self, data: u8) -> Option<bool> {
        self.0 = (self.0 << 8) | u32::from(data);
        if self.0 == FORWARD {
            Some(false)
        } else if self.0 == REVERSE {
            Some(true)
        } else {
            None
        }
    }
}

fn crawl(data: &[u8], part2: bool) -> anyhow::Result<Vec<Match>> {
    // including newline byte
    let linelen = data.iter().position(|c| *c == b'\n').unwrap();
    let linebytes = linelen + 1;
    let linecount = {
        let n = (data.trim_ascii_end().len() + 1).div_ceil(linebytes);
        for i in 0..n {
            let pos = i * linebytes + linelen;
            if data.get(pos).copied() != Some(b'\n') && !(i == (n - 1) && data.get(pos) == None) {
                return Err(anyhow::anyhow!(
                    "missing newline at byte {}. linelen={} linecount={} actual={:x?}",
                    pos,
                    linelen,
                    n,
                    data.get(pos).copied()
                ));
            }
        }
        n
    };

    let get = |row, col| data.get(row * linebytes + col).map(|c| *c);
    let mut ret = vec![];

    if part2 {
        for rowi in 1..(linecount-1) {
            for coli in 1..(linecount-1) {
                if get(rowi, coli) != Some(L_A) {
                    continue;
                }
                let tl = get(rowi-1, coli-1);
                let dr = get(rowi+1, coli+1);
                let tr = get(rowi-1, coli+1);
                let dl = get(rowi+1, coli-1);
                let tldr: bool = if tl == Some(L_M) && dr == Some(L_S) {
                    false
                } else if tl == Some(L_S) && dr == Some(L_M) {
                    true
                } else {
                    continue;
                };
                let trdl: bool = if dl == Some(L_M) && tr == Some(L_S) {
                    false
                } else if dl == Some(L_S) && tr == Some(L_M) {
                    true
                } else {
                    continue;
                };
                ret.push(Match {
                    row: rowi,
                    col: coli,
                    orient: Orient::XMas(tldr, trdl),
                });
            }
        }
    } else {
        let mut colcrawl = std::iter::repeat_n(Crawler(0), linelen).collect::<Vec<_>>();

        for rowi in 0..linecount {
            let mut rowcrawl = Crawler(0);
            for coli in 0..linelen {
                let c = get(rowi, coli).ok_or_else(|| {
                    anyhow::anyhow!("fail to fetch byte row {} col {}", rowi, coli)
                })?;
                if let Some(dir) = rowcrawl.push(c) {
                    ret.push(Match {
                        row: rowi,
                        col: coli.checked_sub(3).unwrap(),
                        orient: Orient::Horiz(dir),
                    });
                }
                if let Some(dir) = colcrawl.get_mut(coli).unwrap().push(c) {
                    ret.push(Match {
                        row: rowi,
                        col: coli,
                        orient: Orient::Vert(dir),
                    });
                }
                if (rowi + 3) < linecount && (coli + 3) < linelen {
                    let mut diag = Crawler(0);
                    diag.push(c);
                    diag.push(get(rowi + 1, coli + 1).unwrap());
                    diag.push(get(rowi + 2, coli + 2).unwrap());
                    if let Some(dir) = diag.push(get(rowi + 3, coli + 3).unwrap()) {
                        ret.push(Match {
                            row: rowi,
                            col: coli,
                            orient: Orient::DiagForward(dir),
                        });
                    }
                }
                if (rowi + 3) < linecount && ((coli as isize) - 3) >= 0 {
                    let mut diag = Crawler(0);
                    diag.push(c);
                    diag.push(get(rowi + 1, coli - 1).unwrap());
                    diag.push(get(rowi + 2, coli - 2).unwrap());
                    if let Some(dir) = diag.push(get(rowi + 3, coli - 3).unwrap()) {
                        ret.push(Match {
                            row: rowi,
                            col: coli,
                            orient: Orient::DiagBack(dir),
                        });
                    }
                }
            }
        }
        println!("colcrawl: {:x?}", colcrawl);
    }

    Ok(ret)
}

fn main() -> Result<(), anyhow::Error> {
    let mut input = std::io::stdin().lock();
    let mut data = Vec::new();
    input.read_to_end(&mut data)?;
    let data = data;

    let res = crawl(&data, true)?;
    println!("{}", res.len());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let data = r"..X...X
.SAMXM.
.A..A..
XMAS.S.
.X.....";
        let mut res = crawl(data.as_bytes(), false).unwrap();
        res.sort();
        assert_eq!(
            res,
            vec![
                Match {
                    row: 0,
                    col: 2,
                    orient: Orient::DiagForward(false)
                },
                Match {
                    row: 0,
                    col: 6,
                    orient: Orient::DiagBack(false)
                },
                Match {
                    row: 1,
                    col: 1,
                    orient: Orient::Horiz(true)
                },
                Match {
                    row: 3,
                    col: 0,
                    orient: Orient::Horiz(false)
                },
                Match {
                    row: 4,
                    col: 1,
                    orient: Orient::Vert(true)
                },
            ]
        );
    }

    #[test]
    fn example2() {
        let data = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        let mut res = crawl(data.as_bytes(), false).unwrap();
        res.sort();
        println!("{:?}", res);
        assert_eq!(res.len(), 18);
    }

    #[test]
    fn example3() {
        let data = r"M.S
.A.
M.S
";
        let mut res = crawl(data.as_bytes(), true).unwrap();
        res.sort();
        assert_eq!(res, vec![Match{row: 1, col: 1, orient: Orient::XMas(false, false)}]);
    }
    #[test]
    fn example4() {
        let data = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        let mut res = crawl(data.as_bytes(), true).unwrap();
        res.sort();
        println!("{:?}", res);
        assert_eq!(res.len(), 9);
    }
}
