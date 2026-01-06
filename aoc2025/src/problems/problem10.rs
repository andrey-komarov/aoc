use std::io::BufRead;
use anyhow::Context;
use crate::problems::common::Problem;

pub struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

impl Machine {
    fn min_presses(&self) -> usize {
        let buttons = self.buttons.len();
        (0..(1 << buttons)).filter(|mask| {
            let mut enabled = vec![false; self.lights.len()];
            (0..buttons).for_each(|i| {
                if (mask & (1 << i)) != 0 {
                    self.buttons[i].iter().for_each(|b| {
                        enabled[*b] ^= true;
                    })
                }
            });
            enabled == self.lights
        }).map(|mask: u32| {
            mask.count_ones()
        }).min().unwrap() as usize
    }
}
pub struct Input {
    machines: Vec<Machine>,
}

pub struct Problem10 {
}

impl Problem10 {
    pub fn new() -> Self { Self {} }
}

impl Problem for Problem10 {
    type Input = Input;
    type Output = usize;

    fn parse_from<R: BufRead>(&self, buf: R) -> anyhow::Result<Self::Input> {
        let mut machines = Vec::new();
        for line in buf.lines() {
            let line = line?;
            let mut lights = None;
            let mut buttons = Vec::new();
            let mut joltage = None;
            for part in line.split_whitespace() {
                match part.chars().next() {
                    Some('[') => {
                        let part = part.replace("[", "").replace("]", "");
                        lights = Some(part.chars().map(|c| c == '#').collect::<Vec<_>>());
                    },
                    Some('(') => {
                        let part = part.replace("(", "").replace(")", "");
                        let button = part.split(",").into_iter().map(|x| {
                                x.parse().with_context(|| format!("Can't parse {}", x))
                            }).collect::<anyhow::Result<Vec<_>>>()?;
                        buttons.push(button);
                    },
                    Some('{') => {
                        let part = part.replace("{", "").replace("}", "");
                        joltage = Some(part.split(",").into_iter().map(|x| {
                            x.parse().with_context(|| format!("Can't parse {}", x))
                        }).collect::<anyhow::Result<Vec<_>>>()?)
                    },
                    _ => anyhow::bail!("don't know what to do with {}", part),
                }
            }
            machines.push(Machine {
                lights: lights.with_context(|| "no []")?,
                buttons,
                joltage: joltage.with_context(|| "no {}")?,
            })
        }
        Ok(Input {
            machines,
        })
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        input.machines.into_iter().map(|machine| {
            machine.min_presses()
        }).sum()
    }
}