use std::io::{BufRead, Write};
use crate::problems::common::{Readable, Solvable};
use anyhow::Result;

struct Input {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let nums: Vec<&str> = s.split_whitespace().collect();
        let mut left = Vec::new();
        let mut right = Vec::new(); 
        for i in 0..(nums.len() / 2) {  
            left.push(nums[2 * i].parse()?);
            right.push(nums[2 * i + 1].parse()?);
        }
        Ok(Input { left, right }) 
    }
}

type Output = i32;

pub(crate) struct Problem {}

impl Problem {
    fn solve(&self, mut input: Input) -> Output {
        input.left.sort();
        input.right.sort();
        input.left.iter().zip(input.right).map(|(a, b)| (a - b).abs()).sum()
    }
}

impl Solvable for Problem {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}