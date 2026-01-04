use std::io::BufRead;
use anyhow::{anyhow};
use crate::problems::common::Problem;

enum Direction {
    Left, Right
}

struct Rotation {
    dir: Direction,
    count: u32
}

pub struct Input {
    rotations: Vec<Rotation>,
}

pub(crate) struct Problem01 {}

impl Problem for Problem01 {
    type Input = Input;
    type Output = i32;

    fn parse_from<R: BufRead>(&self, buf: R) -> anyhow::Result<Self::Input> {
        let rotations = buf.lines().map(|line| {
            let line = line?;
            let (lhs, rhs) = line.split_at(1);
            let dir: Direction = match lhs {
                "L" => Ok(Direction::Left),
                "R" => Ok(Direction::Right),
                _ => Err(anyhow!("illegal rotation")),
            }?;
            let count = rhs.parse().map_err(|e| anyhow!("can't parse count: {} ({})", rhs, e))?;
            Ok(Rotation {
                dir, count
            })
        }).collect::<anyhow::Result<_>>()?;
        Ok(Input { rotations })
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        let mut count = 0;
        let mut pos = 50;
        for rotation in input.rotations {
            match rotation.dir {
                Direction::Left => {
                    pos = (pos + 100 - rotation.count % 100) % 100;
                }
                Direction::Right => {
                    pos = (pos + rotation.count) % 100;
                }
            }
            if pos == 0 {
                count += 1;
            }
        }
        count
    }
}