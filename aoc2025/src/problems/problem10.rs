use std::cmp::min;
use std::collections::HashMap;
use std::io::BufRead;
use std::ops::Mul;
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
    cache: HashMap<Vec<i32>, Option<usize>>,
    best: usize,
    current: usize,
}

impl MachineState<'_> {
    fn go(&mut self) -> Option<usize> {
        // println!("GO {:?} {:?}", self.remaining_joltage, self.used_buttons);
        if self.current + (*self.remaining_joltage.iter().max().unwrap() as usize) >= self.best {
            return None;
        }
        if self.remaining_joltage.iter().all(|j| *j == 0) {
            // println!("YOOOOOOO");
            if self.current < self.best {
                println!("... found {} < {}", self.current, self.best);
            }
            self.best = min(self.best, self.current);
            return Some(0)
        }
        if self.remaining_joltage.iter().any(|j| *j < 0) || self.used_buttons.iter().all(|b| *b) {
            return None;
        }
        if let Some(res) = self.cache.get(&self.remaining_joltage) {
            return *res;
        }
        let mut rarity = HashMap::new();
        for (i, button) in self.machine.buttons.iter().enumerate() {
            if self.used_buttons[i] {
                continue
            }
            for light in button {
                *rarity.entry(*light).or_insert(0) += 1
            }
        }
        // println!("rarity: {:?} {:?} {:?}", rarity, self.remaining_joltage, self.used_buttons);
        let (rarest_light, rarest_count) = rarity.into_iter().min_by_key(|(_light, cnt)| {
            *cnt
        }).unwrap();
        let button_with_rarest_light = self.machine.buttons.iter().enumerate().filter_map(|(i, button)| {
            if !self.used_buttons[i] && button.contains(&rarest_light) {
                Some((i, button.len()))
            } else {
                None
            }
        }).min_by_key(|(i, cnt)| {
            (*cnt, self.machine.buttons[*i].iter().map(|light| self.remaining_joltage[*light]).min())
            // (self.machine.buttons[*i].iter().map(|light| self.remaining_joltage[*light]).min(), *cnt)
        }).unwrap().0;
        let mut result = None;
        let clicks_cnt: Vec<i32> = if rarest_count == 1 {
            [self.remaining_joltage[rarest_light]].into_iter().collect()
        } else {
            let mn = self.machine.buttons[button_with_rarest_light].iter().map(|light| {
                self.remaining_joltage[*light]
            }).min().unwrap();
            (0..=mn).collect()
        };
        // println!("rarity2: {:?} button {:?} ({:?}), trying {:?}", rarest_light, button_with_rarest_light, self.machine.buttons[button_with_rarest_light], clicks_cnt);
        self.used_buttons[button_with_rarest_light] = true;
        for clicks in clicks_cnt {
            for light in self.machine.buttons[button_with_rarest_light].iter() {
                self.remaining_joltage[*light] -= clicks;
            }
            self.current += clicks as usize;
            let res = self.go().map(|r| r + clicks as usize);
            self.current -= clicks as usize;
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
        self.cache.insert(self.remaining_joltage.clone(), result);
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
            cache: HashMap::new(),
            best: usize::max_value(),
            current: 0,
        };
        let res = state.go();
        res.unwrap()
    }

    fn min_presses_2_z3(&self) -> usize {
        let solver = z3::Solver::new();
        let clicks = self.buttons.iter().enumerate().map(|(i, button)| {
            z3::ast::Int::fresh_const(&format!("click_{}", i))
        }).collect::<Vec<_>>();
        clicks.iter().for_each(|click| {
            solver.assert(click.ge(0))
        });
        let mut joltages = vec![vec![]; self.joltage.len()];
        for (click, button) in clicks.iter().zip(self.buttons.iter()) {
            for light in button {
                joltages[*light].push(click)
            }
        }
        joltages.into_iter().zip(self.joltage.iter()).for_each(|(clicks, target)| {
            solver.assert(z3::ast::Int::add(&*clicks).eq(*target))
        });
        let mut best = 1_000_000;
        loop {
            let s2 = solver.clone();
            s2.assert(z3::ast::Int::add(&clicks).lt(best));
            if let Some(clicks) = s2.check_and_get_model(&clicks, true) {
                let sum = clicks.into_iter().map(|c| {
                    c.as_u64().unwrap()
                }).sum();
                println!("YAY {sum}");
                best = sum;
            } else {
                break
            }
        }
        // let sols = solver.solutions(clicks, true).take(5).collect::<Vec<_>>();
        best as usize
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
                // machine.min_presses_2()
                machine.min_presses_2_z3()
            }
        }).sum()
    }
}