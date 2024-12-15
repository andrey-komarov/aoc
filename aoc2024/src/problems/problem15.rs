use std::{io::{BufRead, Write}, str::FromStr};

use super::common::{Readable, Solvable};


#[derive(Debug)]
enum Direction {
    Left, Up, Right, Down,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
        }
    }
}

#[derive(Debug, Clone)]
enum Cell {
    Empty, Box, Wall
}

#[derive(Debug)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Input {
    field: Vec<Vec<Cell>>,
    instructions: Vec<Direction>,
    start: Pos,
}

impl Readable for Input {
    fn parse_from<R: std::io::BufRead>(mut input: R) -> anyhow::Result<Self> where Self: Sized {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let s: Vec<&str> = s.split("\n\n").collect();
        let mut field = Vec::new();
        let mut start = None;
        for (x, line) in (0..).zip(s[0].lines().filter(|s| !s.is_empty())) {
            let mut f = Vec::new();
            for (y, c) in (0..).zip(line.chars()) {
                let c = match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Wall,
                    'O' => Cell::Box,
                    '@' => {
                        start = Some(Pos {x, y});
                        Cell::Empty
                    },
                    c => return Err(anyhow::anyhow!("<{}> is not a cell", c))
                };
                f.push(c);
            }
            field.push(f);
        }
        let mut instructions = Vec::new();
        for line in s[1].lines() {
            for c in line.chars() {
                instructions.push(match c {
                    'v' => Direction::Down,
                    '^' => Direction::Up,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    c => return Err(anyhow::anyhow!("<{}> is not a direction", c))
                })
            }
        }
        
        let start = start.ok_or_else(|| anyhow::anyhow!("no start pos"))?;
        Ok(Input {
            field, instructions, start
        })
    }
}

type Output = i32;

pub(crate) struct PartOne {}

impl PartOne {
    fn solve(&self, input: Input) -> Output {
        let (n, m) = (input.field.len(), input.field[0].len());
        let mut field = input.field.clone();
        let mut pos = input.start;
        for instruction in input.instructions {
            let (dx, dy) = instruction.offset();
            let mut boxes = 0;
            while let Cell::Box = field[(pos.x + (boxes + 1) * dx) as usize][(pos.y + (boxes + 1) * dy) as usize] {
                boxes += 1;
            }
            let new_x = (pos.x + (boxes + 1) * dx) as usize;
            let new_y = (pos.y + (boxes + 1) * dy) as usize;
            if let Cell::Wall = field[new_x][new_y] {

            } else {
                field[new_x][new_y] = Cell::Box;
                field[(pos.x + dx) as usize][(pos.y + dy) as usize] = Cell::Empty;
                pos = Pos {x: pos.x + dx, y: pos.y + dy}
            }
        }
        (0..).zip(field).map(|(x, line)| {
            (0..).zip(line).map(|(y, c)| {
                match c {
                    Cell::Box => 100 * x + y,
                    _ => 0,
                }
            }).sum::<Output>()
        }).sum()
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