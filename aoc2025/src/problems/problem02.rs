use std::io::BufRead;
use anyhow::Context;
use crate::problems::common::Problem;

struct Range {
    left: u64,
    right: u64,
}

pub struct Input {
    ranges: Vec<Range>
}

pub struct Problem02 {
    is_part1: bool
}

impl Problem02 {
    pub fn new() -> Problem02 {
        Self { is_part1: true }
    }

    pub fn new_part2() -> Problem02 {
        Self { is_part1: false }
    }
}

impl Problem for Problem02 {
    type Input = Input;
    type Output = u64;

    fn parse_from<R: BufRead>(&self, buf: R) -> anyhow::Result<Self::Input> {
        let mut ranges = Vec::new();
        for line in buf.lines() {
            for range in line?.split(',') {
                let (lhs, rhs) = range.split_once('-').with_context(|| "Can't parse range")?;
                ranges.push(Range {
                    left: lhs.parse()?,
                    right: rhs.parse()?,
                })
            }
        }
        Ok(Input { ranges })
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        fn is_invalid(num: u64) -> bool {
            let s = num.to_string();
            let (lhs, rhs) = s.split_at(s.len() / 2);
            lhs == rhs
        }
        fn is_invalid2(num: u64) -> bool {
            let s = num.to_string();
            for len in 1..s.len() {
                if s.len() % len != 0 {
                    continue
                }
                let bad = (0..(s.len() / len)).map(|i| {
                    &s[(i * len)..((i+1) * len)] == &s[0..len]
                }).all(|x| x);
                if bad {
                    return true
                }
            }
            false
        }
        let mut sum = 0;
        for range in input.ranges {
            for x in range.left..=range.right {
                if self.is_part1 && is_invalid(x) {
                    sum += x;
                }
                if (!self.is_part1) && is_invalid2(x) {
                    sum += x;
                }
            }
        }
        sum
    }
}