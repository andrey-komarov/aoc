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

pub(crate) struct Problem07 {}

impl Problem07 {
    pub(crate) fn new() -> Self { Self {} }
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
}