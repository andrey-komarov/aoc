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
