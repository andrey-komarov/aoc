use std::collections::HashSet;
use std::io::{BufRead, Write};
use anyhow::anyhow;
use crate::problems::common::{Readable, Solvable};

#[derive(Copy, Clone)]
enum Direction {
    Left, Up, Right, Down,
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }
}

enum Cell {
    Empty, Crate,
}

struct Pos {
    row: i32,
    col: i32,
    dir: Direction,
}

impl Pos {
    fn step(&self) -> Self {
        match &self.dir {
            Direction::Left => Pos {row: self.row, col: self.col - 1, dir: self.dir},
            Direction::Up => Pos {row: self.row - 1, col: self.col, dir: self.dir},
            Direction::Right => Pos {row: self.row, col: self.col + 1, dir: self.dir},
            Direction::Down => Pos {row: self.row + 1, col: self.col, dir: self.dir},
        }
    }
}

struct Input {
    field: Vec<Vec<Cell>>,
    guard: Pos,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let field = s.lines().filter(|s| !s.is_empty()).map(|s| {
            s.chars()
        });
        let mut pos: Option<Pos> = None;
        let field = (0..).zip(field).map(|(i, row) | {
            (0..).zip(row).map(|(j, cell)| {
                match cell {
                    '#' => Cell::Crate,
                    '.' => Cell::Empty,
                    'v' => {
                        pos = Some(Pos {row: i, col: j, dir: Direction::Down});
                        Cell::Empty
                    },
                    '>' => {
                        pos = Some(Pos {row: i, col: j, dir: Direction::Right});
                        Cell::Empty
                    },
                    '^' => {
                        pos = Some(Pos {row: i, col: j, dir: Direction::Up});
                        Cell::Empty
                    },
                    '<' => {
                        pos = Some(Pos {row: i, col: j, dir: Direction::Left});
                        Cell::Empty
                    },
                    _ => todo!(),
                }
            }).collect()
        }).collect();
        Ok(Input {
            field,
            guard: pos.ok_or_else(|| anyhow!("No guard"))?,
        })
    }
}

type Output = i32;

pub(crate) struct PartOne {}

impl PartOne {
    fn solve(&self, input: Input) -> Output {
        let mut visited = HashSet::new();
        let mut guard = input.guard;
        let inside = |guard: &Pos| -> bool {
            0 <= guard.row && guard.row < input.field.len() as i32
                && 0 <= guard.col && guard.col < input.field[0].len() as i32
        };
        loop {
            visited.insert((guard.row, guard.col));
            let next = guard.step();
            if !inside(&next) {
                break;
            }
            if let Cell::Crate = input.field[next.row as usize][next.col as usize] {
                guard = Pos {row: guard.row, col: guard.col, dir: guard.dir.rotate()};
            } else {
                guard = next
            }
        }
        visited.len() as Output
    }
}

impl Solvable for PartOne {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}
