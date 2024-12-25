use std::io::{BufRead, Write};
use std::iter::zip;
use crate::problems::common::{Readable, Solvable};

struct Input {
    schemes: Vec<Vec<String>>
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let schemes = s.split("\n\n").map(|block| {
            block.lines().filter(|l| !l.is_empty()).map(|line| {
                line.to_string()
            }).collect()
        }).filter(|b: &Vec<_>| b.len() > 0).collect();
        Ok(Input { schemes })
    }
}

pub(crate) struct PartOne {}

type Output = usize;

impl PartOne {
    fn tumblers(block: &Vec<String>) -> Vec<i32> {
        let mut res = vec![-1; block.iter().map(String::len).max().unwrap_or(0)];
        for line in block {
            for (col, c) in line.chars().enumerate() {
                res[col] += if c == '#' { 1 } else { 0 };
            }
        }
        res
    }
    fn solve(&self, input: Input) -> Output {
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        for block in input.schemes {
            if block[0].starts_with("#") {
                locks.push(Self::tumblers(&block));
            } else {
                keys.push(Self::tumblers(&block));
            }
        }
        // println!("{:?} {:?}", locks, keys);
        locks.iter().map(|lock| {
            keys.iter().map(|key| {
                if zip(lock, key).all(|(l, k)| l + k < 6) {
                    1
                } else {
                    0
                }
            }).sum::<Output>()
        }).sum()
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
