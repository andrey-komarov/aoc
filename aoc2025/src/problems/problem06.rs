use std::io::{BufRead};
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

pub(crate) struct Problem06Part2 {}

impl Problem06Part2 {
    pub fn new() -> Self { Self {} }
}

impl Problem for Problem06Part2 {
    type Input = Input;
    type Output = u64;

    fn parse_from<R: BufRead>(&self, buf: R) -> anyhow::Result<Self::Input> {
        let mut lines = buf.lines().map(|line| line.with_context(|| "what")).collect::<anyhow::Result<Vec<_>>>()?;
        let max_len = lines.iter().map(|line| line.len()).max().with_context(|| "no lines")?;
        let mut ops = lines.pop().with_context(|| "no last line")?;
        let mut lines: Vec<Vec<u8>> = lines.iter().map(|line| {
            line.as_bytes().to_vec()
        }).collect();
        for line in lines.iter_mut() {
            while line.len() < max_len {
                line.push(b' ')
            }
        }
        while ops.len() <= max_len {
            ops.push(' ');
        }
        ops.push('+');
        let ops: Vec<_> = ops.chars().collect();

        let mut equations = Vec::new();
        for (i, c) in ops.iter().enumerate() {
            let op;
            match c {
                '+' => op = Operation::Plus,
                '*' => op = Operation::Multiply,
                _ => continue,
            }
            if i == ops.len() - 1 {
                break;
            }
            let j = (i+1..).filter(|j| ops[*j] != ' ').next().with_context(|| "no nex op")?;
            let mut operands = Vec::new();
            for col in (i..j-1).rev() {
                let mut num = Vec::new();
                for row in 0..lines.len() {
                    if lines[row][col] != b' ' {
                        num.push(lines[row][col]);
                    }
                }
                operands.push(String::from_utf8(num)?.as_str().parse()?);
            }
            equations.push(Equation { operation: op, operands });
        }
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