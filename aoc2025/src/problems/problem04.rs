use std::cmp::PartialEq;
use std::io::BufRead;
use crate::problems::common::Problem;

#[derive(Eq, PartialEq, Clone)]
enum Cell {
    Empty, PaperRoll
}

pub struct Input {
    field: Vec<Vec<Cell>>
}

pub(crate) struct Problem04 {
    is_part1: bool
}

impl Problem04 {
    pub fn new() -> Self { Self { is_part1: true } }

    pub fn new_part2() -> Self { Self { is_part1: false } }
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
        let can_remove = |field: &Vec<Vec<Cell>>, x: usize, y: usize| -> bool {
            let mut count = 0;
            for dx in (-1 as isize)..=1 {
                for dy in (-1 as isize)..=1 {
                    let x = x as isize + dx;
                    let y = y as isize + dy;
                    if 0 <= x && x < rows as isize && 0 <= y && y < cols as isize {
                        if let Cell::PaperRoll = field[x as usize][y as usize] {
                            count += 1;
                        }
                    }
                }
            }
            count <= 4 && field[x][y] == Cell::PaperRoll
        };

        let mut answer = 0;
        if self.is_part1 {
            for i in 0..rows {
                for j in 0..cols {
                    if can_remove(&input.field, i, j) {
                        answer += 1;
                    }
                }
            }
        } else {
            let mut field = input.field.clone();
            loop {
                let mut updated = false;
                for i in 0..rows {
                    for j in 0..cols {
                        if can_remove(&field, i, j) {
                            answer += 1;
                            field[i][j] = Cell::Empty;
                            updated = true;
                        }
                    }
                }
                if !updated {
                    break;
                }
            }
        }
        answer
    }
}