use std::collections::VecDeque;
use std::io::{BufRead, Write};
use crate::problems::common::{Readable, Solvable};

#[derive(Clone)]
enum Cell {
    Empty, Full,
}

#[derive(Debug, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Input {
    bytes: Vec<Pos>,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let positions = s.lines().filter_map(|s| {
            let (a, b) = s.split_once(',')?;
            Some(Pos { x: b.parse().ok()?, y: a.parse().ok()? })
        }).collect();
        Ok(Input { bytes: positions })
    }
}

pub(crate) struct PartOne {
    n: usize, m: usize,
    prefix: usize,
}

type Output = usize;

impl PartOne {
    pub fn new(n: usize, m: usize, prefix: usize) -> Self {
        Self { n, m, prefix }
    }

    fn solve(&self, input: &Input) -> Output {
        let mut field = vec![vec![Cell::Empty; self.n]; self.m];
        for pos in input.bytes.iter().take(self.prefix) {
            field[pos.x][pos.y] = Cell::Full;
        }
        let target = Pos {x: self.n - 1, y: self.m - 1};
        let mut queue = VecDeque::from([(0, Pos {x: 0, y: 0})]);
        let mut dist = vec![vec![None; self.m]; self.n];
        dist[0][0] = Some(0);
        while let Some((d, Pos {x, y})) = queue.pop_front() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (x2, y2) = (x as i32 + dx, y as i32 + dy);
                if !(0 <= x2 && x2 < self.n as i32 && 0 <= y2 && y2 < self.m as i32) {
                    continue;
                }
                let (x2, y2) = (x2 as usize, y2 as usize);
                if let Cell::Full = field[x2][y2] {
                    continue;
                }
                if dist[x2][y2].is_none() {
                    dist[x2][y2] = Some(d + 1);
                    queue.push_back((d + 1, Pos { x: x2, y: y2 }));
                }
            }
        }
        dist[self.n - 1][self.m - 1].unwrap()
    }
}

impl Solvable for PartOne {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(&input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}

pub(crate) struct PartTwo {
    n: usize, m: usize,
}

type Output2 = String;

impl PartTwo {
    pub fn new(n: usize, m: usize) -> Self {
        Self { n, m }
    }

    fn check(&self, input: &Input, prefix: usize) -> bool {
        let mut field = vec![vec![Cell::Empty; self.n]; self.m];
        for pos in input.bytes.iter().take(prefix) {
            field[pos.x][pos.y] = Cell::Full;
        }
        let target = Pos {x: self.n - 1, y: self.m - 1};
        let mut queue = VecDeque::from([(0, Pos {x: 0, y: 0})]);
        let mut dist = vec![vec![None; self.m]; self.n];
        dist[0][0] = Some(0);
        while let Some((d, Pos {x, y})) = queue.pop_front() {
            if (x, y) == (self.n - 1, self.m - 1) {
                return true;
            }
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (x2, y2) = (x as i32 + dx, y as i32 + dy);
                if !(0 <= x2 && x2 < self.n as i32 && 0 <= y2 && y2 < self.m as i32) {
                    continue;
                }
                let (x2, y2) = (x2 as usize, y2 as usize);
                if let Cell::Full = field[x2][y2] {
                    continue;
                }
                if dist[x2][y2].is_none() {
                    dist[x2][y2] = Some(d + 1);
                    queue.push_back((d + 1, Pos { x: x2, y: y2 }));
                }
            }
        }
        false
    }

    fn solve(&self, input: &Input) -> Output2 {
        for i in (0..input.bytes.len()).rev() {
            if self.check(input, i) {
                return format!("{},{}", input.bytes[i].y, input.bytes[i].x);
            }
        }
        unreachable!()
    }
}

impl Solvable for PartTwo {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(&input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}
