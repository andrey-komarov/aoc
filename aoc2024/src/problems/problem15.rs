use core::fmt;
use std::{io::{BufRead, Write}, str::FromStr};

use super::common::{Readable, Solvable};


#[derive(Debug, Clone)]
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

    fn is_horizontal(&self) -> bool {
        match self {
            Direction::Left | Direction::Right => true,
            Direction::Up | Direction::Down => false,
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

#[derive(Clone, Copy)]
enum Cell2 {
    Wall, Empty, BoxLeft, BoxRight
}

impl fmt::Debug for Cell2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell2::Wall => "#",
            Cell2::Empty => ".",
            Cell2::BoxLeft => "[",
            Cell2::BoxRight => "]",
        })
    }
}

struct Input2 {
    field: Vec<Vec<Cell2>>,
    start: Pos,
    instructions: Vec<Direction>,
}

impl From<Input> for Input2 {
    fn from(value: Input) -> Self {
        Self {
            field: value.field.into_iter().map(|line| {
                line.into_iter().flat_map(|c| {
                    match c {
                        Cell::Empty => [Cell2::Empty, Cell2::Empty],
                        Cell::Box => [Cell2::BoxLeft, Cell2::BoxRight],
                        Cell::Wall => [Cell2::Wall, Cell2::Wall],
                    }
                }).collect()
            }).collect(),
            start: Pos {x: value.start.x, y: value.start.y * 2},
            instructions: value.instructions,
        }
    }
}

pub(crate) struct PartTwo {}

impl PartTwo {
    fn solve(&self, input: Input2) -> Output {
        let mut x = PartTwoMut::new(input);
        x.solve()
    }
}

struct PartTwoMut {
    field: Vec<Vec<Cell2>>,
    pos: Pos,
    instructions: Vec<Direction>,
}

impl fmt::Debug for PartTwoMut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut field: Vec<Vec<char>> = self.field.iter().map(|line| {
            line.iter().flat_map(|c| {
                format!("{:?}", c).chars().collect::<Vec<_>>()
            }).collect()
        }).collect();
        field[self.pos.x as usize][self.pos.y as usize] = '@';
        for line in field {
            let s: String = line.into_iter().collect();
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl PartTwoMut {
    fn new(input: Input2) -> Self {
        PartTwoMut {
            field: input.field,
            pos: input.start,
            instructions: input.instructions,
        }
    }

    fn dual(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self.field[x][y] {
            Cell2::BoxLeft => Some((x, y + 1)),
            Cell2::BoxRight => Some((x, y - 1)),
            _ => None,
        }
    }

    fn can_shift(&self, x: usize, y: usize, dir: &Direction) -> bool {
        let (dx, dy) = dir.offset();
        let (x2, y2) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
        match self.field[x2][y2] {
            Cell2::Wall => false,
            Cell2::Empty => true,
            Cell2::BoxLeft | Cell2::BoxRight => {
                if dir.is_horizontal() {
                    let y3 = (y as isize + 2 * dy) as usize;
                    self.can_shift(x, y3, dir)
                } else {
                    let (x3, y3) = self.dual(x2, y2).unwrap();
                    self.can_shift(x2, y2, dir) && self.can_shift(x3, y3, dir)
                }
            },
        }
    }

    fn shift(&mut self, x: usize, y: usize, dir: &Direction) -> () {
        let (dx, dy) = dir.offset();
        let (x2, y2) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
        match self.field[x2][y2] {
            Cell2::Wall => panic!(),
            Cell2::Empty => (),
            Cell2::BoxLeft | Cell2::BoxRight => {
                if dir.is_horizontal() {
                    let y3 = (y as isize + 2 * dy) as usize;
                    let y4 = (y as isize + 3 * dy) as usize;
                    self.shift(x, y3, dir);
                    self.field[x][y4] = self.field[x][y3];
                    self.field[x][y3] = self.field[x][y2];
                    self.field[x][y2] = Cell2::Empty;
                } else {
                    let  (_, y2b) = self.dual(x2, y2).unwrap();
                    let x3 = (x as isize + 2 * dx) as usize;
                    self.shift(x2, y2, dir);
                    self.shift(x2, y2b, dir);
                    self.field[x3][y2] = self.field[x2][y2];
                    self.field[x3][y2b] = self.field[x2][y2b];
                    self.field[x2][y2] = Cell2::Empty;
                    self.field[x2][y2b] = Cell2::Empty;
                }
            },
        }
    }

    fn solve(&mut self) -> Output {
        for dir in self.instructions.clone() {
            // println!("Before step {:?}:\n{:?}", dir, self);
            if self.can_shift(self.pos.x as usize, self.pos.y as usize, &dir) {
                self.shift(self.pos.x as usize, self.pos.y as usize, &dir);
                let (dx, dy) = dir.offset();
                self.pos.x += dx;
                self.pos.y += dy;
            }
        }
        // println!("{:?}", self.field);
        (0..).zip(self.field.clone()).map(|(x, line)| {
            (0..).zip(line).map(|(y, c)| {
                match c {
                    Cell2::BoxLeft => 100 * x + y,
                    _ => 0,
                }
            }).sum::<Output>()
        }).sum()
    }
}

impl Solvable for PartTwo {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(input.into());
        writeln!(output, "{}", out)?;
        Ok(())
    }
}