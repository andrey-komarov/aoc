use std::cmp::{max, min};
use std::io::BufRead;
use anyhow::Context;
use crate::problems::common::Problem;

#[derive(Clone)]
struct Range {
    from: u64,
    to: u64,
}
pub struct Input {
    ranges: Vec<Range>,
    ids: Vec<u64>,
}

pub struct Problem05 {
    is_part1: bool
}

impl Problem05 {
    pub fn new() -> Self { Self { is_part1: true } }
    pub fn new_part2() -> Self { Self { is_part1: false } }
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
        if self.is_part1 {
            input.ids.iter().filter(|&id| {
                input.ranges.iter().any(|range| {
                    range.from <= *id && *id <= range.to
                })
            }).count()
        } else {
            let mut ranges = input.ranges.clone();
            fn merge(range1: &Range, range2: &Range) -> Option<Range> {
                if range1.to < range2.from || range2.to < range1.from {
                    None
                } else {
                    Some(Range {
                        from: min(range1.from, range2.from),
                        to: max(range1.to, range2.to),
                    })
                }
            }
            'outer: loop {
                for i in 0..ranges.len() {
                    for j in i+1..ranges.len() {
                        if let Some(new_range) = merge(&ranges[i], &ranges[j]) {
                            ranges.remove(j);
                            ranges.remove(i);
                            ranges.push(new_range);
                            continue 'outer;
                        }
                    }
                }
                break;
            }
            ranges.iter().map(|range| {
                (range.to - range.from + 1) as usize
            }).sum()
        }
    }
}