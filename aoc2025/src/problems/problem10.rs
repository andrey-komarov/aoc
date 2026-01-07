use std::cmp::min;
use std::collections::HashMap;
use std::io::BufRead;
use anyhow::Context;
use crate::problems::common::Problem;

#[derive(Debug)]
pub struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i32>,
}

pub struct MachineState<'a> {
    machine: &'a Machine,
    remaining_joltage: Vec<i32>,
    used_buttons: Vec<bool>,
}

impl MachineState<'_> {
    fn go(&mut self) -> Option<usize> {
        println!("GO {:?} {:?}", self.remaining_joltage, self.used_buttons);
        let mut rarity = HashMap::new();
        if self.remaining_joltage.iter().all(|j| *j == 0) {
            println!("YOOOOOOO");
            return Some(0)
        }
        if self.remaining_joltage.iter().any(|j| *j < 0) || self.used_buttons.iter().all(|b| *b) {
            return None;
        }
        for (i, button) in self.machine.buttons.iter().enumerate() {
            if self.used_buttons[i] {
                continue
            }
            for light in button {
                *rarity.entry(*light).or_insert(0) += 1
            }
        }
        println!("rarity: {:?} {:?} {:?}", rarity, self.remaining_joltage, self.used_buttons);
        let (rarest_light, rarest_count) = rarity.into_iter().min_by_key(|(_light, cnt)| {
            *cnt
        }).unwrap();
        let button_with_rarest_light = self.machine.buttons.iter().enumerate().filter_map(|(i, button)| {
            if !self.used_buttons[i] && button.contains(&rarest_light) {
                Some((i, button.len()))
            } else {
                None
            }
        }).min_by_key(|(_, cnt)| *cnt).unwrap().0;
        let mut result = None;
        let clicks_cnt: Vec<i32> = if rarest_count == 1 {
            [self.remaining_joltage[rarest_light]].into_iter().collect()
        } else {
            (0..=self.remaining_joltage[rarest_light]).collect()
        };
        println!("rarity2: {:?} button {:?} ({:?}), trying {:?}", rarest_light, button_with_rarest_light, self.machine.buttons[button_with_rarest_light], clicks_cnt);
        self.used_buttons[button_with_rarest_light] = true;
        for clicks in clicks_cnt {
            for light in self.machine.buttons[button_with_rarest_light].iter() {
                self.remaining_joltage[*light] -= clicks;
            }
            let res = self.go().map(|r| r + clicks as usize);
            if let Some(old_res) = result {
                if let Some(res) = res {
                    result = Some(min(old_res, res));
                }
            } else {
                result = res;
            }
            for light in self.machine.buttons[button_with_rarest_light].iter() {
                self.remaining_joltage[*light] += clicks;
            }
        }
        self.used_buttons[button_with_rarest_light] = false;
        result
    }
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


    fn min_presses_2(&self) -> usize {
        let buttons = self.buttons.len();
        let mut state = MachineState {
            machine: &self,
            remaining_joltage: self.joltage.clone(),
            used_buttons: vec![false; buttons],
        };
        let res = state.go();
        res.unwrap()
    }
}
pub struct Input {
    machines: Vec<Machine>,
}

pub struct Problem10 {
    is_part1: bool,
}

impl Problem10 {
    pub fn new() -> Self { Self { is_part1: true } }
    pub fn new_part2() -> Self { Self { is_part1: false } }
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
            if self.is_part1 {
                machine.min_presses()
            } else {
                println!("Solved another one {:?}", machine);
                machine.min_presses_2()
            }
        }).sum()
    }
}