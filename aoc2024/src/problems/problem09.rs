use std::io::{BufRead, Write};
use anyhow::anyhow;
use crate::problems::common::{Readable, Solvable};

struct Input {
    compresed: Vec<u32>,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_line(&mut s)?;
        let x = s.trim().chars().map(
            |c| c.to_digit(10)
                .ok_or_else(|| anyhow!("Parse error"))
        ).collect::<Result<Vec<_>, anyhow::Error>>()?;
        Ok(Input { compresed: x })
    }
}

type Output = u64;

pub(crate) struct PartOne;
impl PartOne {
    fn solve(&self, input: Input) -> Output {
        let mut uncompressed = Vec::new();
        for (i, num) in (0..).zip(&input.compresed)  {
            let elem = if i % 2 == 0 { Some(i / 2) } else { None };
            uncompressed.append(&mut vec![elem; *num as usize]);
        }
        let mut pos_empty = input.compresed[0] as usize;
        loop {
            if let Some(Some(elem)) = uncompressed.last() {
                while (pos_empty < uncompressed.len() && uncompressed[pos_empty].is_some()) {
                    pos_empty += 1;
                }
                if pos_empty == uncompressed.len() {
                    break;
                }
                uncompressed[pos_empty] = uncompressed.pop().unwrap();
            } else {
                uncompressed.pop();
            }
        }
        // println!("{:?}", uncompressed.iter().map(|x| x.unwrap()).collect::<Vec<i32>>());
        (0..).zip(uncompressed).map(|(i, x)| i * x.unwrap() as u64).sum()
    }
}

impl Solvable for PartOne {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}

pub(crate) struct PartTwo;

impl PartTwo {
    fn solve(&self, input: Input) -> Output {
        let mut uncompressed = Vec::new();
        for (i, num) in (0..).zip(&input.compresed)  {
            let elem = if i % 2 == 0 { Some(i / 2) } else { None };
            uncompressed.append(&mut vec![elem; *num as usize]);
        }
        let mut pos = uncompressed.len() - 1;
        let find_gap = |uncompressed: &Vec<Option<_>>, len, rightmost| {
            let mut pos = 0;
            loop {
                if pos >= rightmost {
                    return None;
                }
                if uncompressed[pos].is_some() {
                    pos += 1;
                    continue;
                }
                let mut count = 0;
                while pos <= rightmost && uncompressed[pos].is_none() {
                    count += 1;
                    pos += 1;
                }
                if count >= len {
                    return Some(pos - count);
                }
            }
        };
        loop {
            if pos == 0 {
                break;
            }
            match uncompressed[pos] {
                None => {
                    pos -= 1;
                    continue;
                }
                Some(val) => {
                    let mut count = 0;
                    while pos > 0 && uncompressed[pos] == Some(val) {
                        count += 1;
                        pos -= 1;
                    }
                    // println!("Lol {} {}", val, count);
                    if let Some(gap) = find_gap(&uncompressed, count, pos) {
                        // println!("gap {}", gap);
                        for i in (0..count) {
                            uncompressed[gap + i] = Some(val);
                            uncompressed[pos + i + 1] = None;
                        }
                    }
                }
            }
        }
        // println!("{:?}", uncompressed.iter().map(|x| x.unwrap_or(88)).collect::<Vec<_>>());
        (0..).zip(uncompressed).map(|(i, x)| {
            match x {
                None => 0,
                Some(x) => i * x,
            }
        }).sum()
    }
}

impl Solvable for PartTwo {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}
