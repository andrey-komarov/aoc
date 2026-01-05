use std::io::BufRead;
use anyhow::Context;
use regex::Regex;
use crate::problems::common::Problem;

#[derive(Clone)]
enum Operation {
    Plus, Multiply
}

struct Equation {
    operation: Operation,
    operands: Vec<u64>,
}

pub struct Input {
    equations: Vec<Equation>,
}

pub struct Problem06 {}

impl Problem06 {
    pub fn new() -> Self { Self {} }
}

impl Problem for Problem06 {
    type Input = Input;
    type Output = u64;

    fn parse_from<R: BufRead>(&self, buf: R) -> anyhow::Result<Self::Input> {
        let mut rows = Vec::new();
        let mut ops = Vec::new();
        let nums_re = Regex::new(r"^[0-9 ]+$").unwrap();
        for line in buf.lines() {
            let line = line?;
            if nums_re.is_match(&*line) {
                let nums: Vec<u64> = line.split_whitespace().map(|x| {
                    x.parse::<u64>().with_context(|| "can't parse num")
                }).collect::<anyhow::Result<Vec<_>>>()?;
                rows.push(nums);
            } else {
                ops = line.split_whitespace().map(|x| {
                    match x {
                        "+" => Ok(Operation::Plus),
                        "*" => Ok(Operation::Multiply),
                        _ => anyhow::bail!("can't parse operation <{}>", x)
                    }
                }).collect::<anyhow::Result<Vec<_>>>()?;
                break;
            }
        }
        let cols = ops.len();
        let equations = (0..cols).map(|i| {
            let operation = ops[i].clone();
            let operands = (0..rows.len()).map(|j| rows[j][i]).collect();
            Equation { operation, operands }
        }).collect();
        Ok(Input { equations })
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        input.equations.iter().map(|equation| {
            match equation.operation {
                Operation::Plus => equation.operands.iter().sum::<u64>(),
                Operation::Multiply => equation.operands.iter().product::<u64>(),
            }
        }).sum()
    }
}