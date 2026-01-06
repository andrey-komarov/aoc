use std::collections::HashSet;
use std::io::BufRead;
use anyhow::Context;
use crate::problems::common::Problem;

enum Cell {
    Empty, Splitter
}

pub struct Input {
    field: Vec<Vec<Cell>>,
    start_column: usize,
}

pub(crate) struct Problem07 {
    is_part1: bool,
}

impl Problem07 {
    pub(crate) fn new() -> Self { Self { is_part1: true } }
    pub(crate) fn new_part2() -> Self { Self { is_part1: false } }

    fn solve_part1(&self, input: Input) -> <Problem07 as Problem>::Output {
        let mut beams = HashSet::from([input.start_column]);
        let mut result = 0;
        for line in input.field {
            let mut new_beams = HashSet::new();
            for beam in beams {
                match line[beam] {
                    Cell::Empty => {
                        new_beams.insert(beam);
                    }
                    Cell::Splitter => {
                        new_beams.insert(beam - 1);
                        new_beams.insert(beam + 1);
                        result += 1;
                    }
                }
            }
            beams = new_beams;
        }
        result
    }

    fn solve_part2(&self, input: Input) -> <Problem07 as Problem>::Output {
        let rows = input.field.len();
        let cols = input.field[0].len();
        let mut dp = vec![vec![0; cols]; rows];
        dp.push(vec![1; cols]);
        for i in (0..rows).rev() {
            for j in 0..cols {
                match input.field[i][j] {
                    Cell::Empty => {
                        dp[i][j] = dp[i + 1][j];
                    }
                    Cell::Splitter => {
                        dp[i][j] = dp[i + 1][j - 1] + dp[i + 1][j + 1];
                    }
                }
            }
        }
        dp[0][input.start_column]
    }
}

impl Problem for Problem07 {
    type Input = Input;
    type Output = usize;

    fn parse_from<R: BufRead>(&self, buf: R) -> anyhow::Result<Self::Input> {
        let mut start_column = None;
        let lines: Vec<String> = buf.lines().map(|line| line.with_context(|| "wtf")).collect::<anyhow::Result<Vec<_>>>()?;
        let field = lines.iter().map(|line| {
            line.chars().map(|c| {
                match c {
                    '^' => Cell::Splitter,
                    _ => Cell::Empty,
                }
            }).collect()
        }).collect();
        for (i, c) in lines[0].chars().enumerate() {
            if c == 'S' {
                start_column = Some(i);
            }
        }
        let start_column = start_column.with_context(|| "no start")?;
        Ok(Input {
            field,
            start_column,
        })
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        if self.is_part1 {
            self.solve_part1(input)
        } else {
            self.solve_part2(input)
        }
    }
}