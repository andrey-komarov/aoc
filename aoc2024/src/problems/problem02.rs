use std::io::{BufRead, Write};
use crate::problems::common::{Readable, Solvable};
use anyhow::Result;

struct Input {
    reports: Vec<Vec<i32>>,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let v = s.lines().map(
            |line| {
                line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            }
        ).collect();
        Ok(Input{reports: v})
    }
}

type Output = i32;

pub(crate) struct PartOne {}

impl PartOne {
    fn solve(&self, input: &Input) -> Output {
        fn check(v: &Vec<i32>) -> bool {
            for i in 0..(v.len() - 1) {
                let (a, b) = (v[i], v[i + 1]);
                if (a < b) != (v[0] < v[1]) {
                    return false;
                }
                if (a - b).abs() < 1 || (a - b).abs() > 3 {
                    return false;
                }
            }
            true
        }
        input.reports.iter().map(check).filter(|x| *x).count() as Output
    }
}

impl Solvable for PartOne {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(&input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}
