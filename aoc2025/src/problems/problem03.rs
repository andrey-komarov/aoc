use std::io::BufRead;
use crate::problems::common::Problem;

pub struct Input {
    batteries: Vec<Vec<u8>>
}

pub(crate) struct Problem03 {}

impl Problem03 {
    pub(crate) fn new() -> Self { Self {} }
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
            println!("max ({:?}): {}", input, max);
            max
        }
        input.batteries.iter().map(solve1).sum()
    }
}