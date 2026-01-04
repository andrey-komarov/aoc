use std::cmp::max;
use std::io::BufRead;
use crate::problems::common::Problem;

pub struct Input {
    batteries: Vec<Vec<u8>>
}

pub(crate) struct Problem03 {
    is_part1: bool
}

impl Problem03 {
    pub(crate) fn new() -> Self { Self { is_part1: true } }

    pub(crate) fn new_part2() -> Self { Self { is_part1: false } }
}

impl Problem for Problem03 {
    type Input = Input;
    type Output = u64;

    fn parse_from<R: BufRead>(&self, buf: R) -> anyhow::Result<Self::Input> {
        let mut batteries = Vec::new();
        for line in buf.lines() {
            let battery = line?.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>();
            batteries.push(battery)
        }
        Ok(Input { batteries })
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        fn solve1(input: &Vec<u8>) -> u64 {
            let mut max: u64 = 0;
            for i in 0..input.len() {
                for j in i+1..input.len() {
                    let here = (10 * input[i] + input[j]) as u64;
                    if here > max {
                        max = here
                    }
                }
            }
            max
        }
        fn concat(n1: u64, n2: u64) -> u64 {
            let s = format!("{}{}", n1, n2);
            s.parse().unwrap_or(0)
        }
        fn solve1_2(input: &Vec<u8>) -> u64 {
            // dp[i][j] = starting
            let mut dp = vec![0u64; input.len() + 1];
            for _x in 0..12 {
                let mut dp2 = vec![0u64; input.len() + 1];
                for i in (0..input.len()).rev() {
                    dp2[i] = dp2[i + 1];
                    dp2[i] = max(dp2[i], concat(input[i] as u64, dp[i + 1]));
                }
                dp = dp2
            }
            dp[0] / 10 // lol
        }
        if self.is_part1 {
            input.batteries.iter().map(solve1).sum()
        } else {
            input.batteries.iter().map(solve1_2).sum()
        }
    }
}