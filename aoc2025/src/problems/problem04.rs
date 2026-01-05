use std::cmp::PartialEq;
use std::io::BufRead;
use crate::problems::common::Problem;

#[derive(Eq, PartialEq)]
enum Cell {
    Empty, PaperRoll
}

pub struct Input {
    field: Vec<Vec<Cell>>
}

pub(crate) struct Problem04 {}

impl Problem04 {
    pub fn new() -> Self { Self {} }
}

impl Problem for Problem04 {
    type Input = Input;
    type Output = u64;

    fn parse_from<R: BufRead>(&self, buf: R) -> anyhow::Result<Self::Input> {
        let mut field = Vec::new();
        for line in buf.lines() {
            let line = line?;
            field.push(line.chars().map(|c| {
                match c {
                    '@' => Cell::PaperRoll,
                    _ => Cell::Empty,
                }
            }).collect())
        }
        Ok(Input { field })
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        let rows = input.field.len();
        let cols = input.field[0].len();
        let mut answer = 0;
        for i in 0..rows {
            for j in 0..cols {
                let mut count = 0;
                for dx in (-1 as isize)..=1 {
                    for dy in (-1 as isize)..=1 {
                        let x= i as isize + dx;
                        let y = j as isize + dy;
                        if 0 <= x && x < rows as isize && 0 <= y && y < cols as isize {
                            if let Cell::PaperRoll = input.field[x as usize][y as usize] {
                                count += 1;
                            }
                        }
                    }
                }
                if count <= 4 && input.field[i][j] == Cell::PaperRoll {
                    answer += 1;
                }
            }
        }
        answer
    }
}