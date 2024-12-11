use std::io::{BufRead, Write};
use anyhow::Error;
use crate::problems::common::{Readable, Solvable};

struct Input {
    stones: Vec<i64>,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let stones = s.split_whitespace().map(
            |s| {
                s.parse::<i64>().map_err(|e| anyhow::anyhow!("{}", e))
            }
        ).collect::<Result<Vec<_>, Error>>()?;
        Ok(Input {stones})
    }
}

type Output = i64;

pub(crate) struct PartOne;

impl PartOne {
    fn try_split(stone: i64) -> Option<(i64, i64)> {
        let s = stone.to_string();
        if s.len() % 2 == 0 {
            let (s1, s2) = s.split_at(s.len() / 2);
            Some((s1.parse().ok()?, s2.parse().ok()?))
        } else {
            None
        }
    }
    fn step(stone: i64) -> Vec<i64> {
        if stone == 0 {
            vec![1]
        } else if let Some((s1, s2)) = Self::try_split(stone) {
            vec![s1, s2]
        } else {
            vec![stone * 2024]
        }
    }

    fn go(stone: i64, steps: usize) -> i64 {
        match steps {
            0 => 1,
            _ => Self::step(stone).into_iter().map(|s| Self::go(s, steps - 1)).sum()
        }
    }
    fn solve(&self, input: Input) -> Output {
       input.stones.into_iter().map(|stone| Self::go(stone, 25)).sum()
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
