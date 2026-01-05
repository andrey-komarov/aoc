use std::io::BufRead;
use anyhow::Context;
use crate::problems::common::Problem;

struct Range {
    from: u64,
    to: u64,
}
pub struct Input {
    ranges: Vec<Range>,
    ids: Vec<u64>,
}

pub struct Problem05 {}

impl Problem05 {
    pub fn new() -> Self { Self {} }
}

impl Problem for Problem05 {
    type Input = Input;
    type Output = usize;

    fn parse_from<R: BufRead>(&self, mut buf: R) -> anyhow::Result<Self::Input> {
        let mut s = String::new();
        buf.read_to_string(&mut s)?;
        let (ranges_str, ids_str) = s.split_once("\n\n").with_context(|| "can't split")?;

        let mut ranges = Vec::new();
        for range in ranges_str.lines() {
            let (from, to) = range.split_once('-').with_context(|| "can't split range")?;
            ranges.push(Range { from: from.parse()?, to: to.parse()? });
        }

        let ids = ids_str.lines().map(|line| {
            line.parse::<u64>().with_context(|| format!("can't parse id {}", line))
        }).collect::<anyhow::Result<_>>()?;

        Ok(Input { ranges, ids })
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        input.ids.iter().filter(|&id| {
            input.ranges.iter().any(|range| {
                range.from <= *id && *id <= range.to
            })
        }).count()
    }
}