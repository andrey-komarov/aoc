use std::collections::HashMap;
use std::io::{BufRead, Write};
use crate::problems::common::{Readable, Solvable};

struct Input {
    field: Vec<Vec<Option<char>>>,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let field = s.lines().filter(|s| !s.is_empty()).map(|s| {
            s.chars().map(|c| {
                if c == '.' { None } else { Some(c) }
            }).collect()
        }).collect();
        Ok(Input {field})
    }
}

type Output = i32;

pub(crate) struct PartOne;


impl PartOne {
    fn solve(&self, input: Input) -> Output {
        let (n, m) = (input.field.len(), input.field[0].len());
        let mut antennae = HashMap::new();
        for (i, line) in (0..).zip(input.field.into_iter()) {
            for (j, c) in (0..).zip(line.into_iter()) {
                if let Some(c) = c {
                    antennae.entry(c).or_insert(Vec::new()).push((i, j));
                }
            }
        }
        let mut antinode = vec![vec![false; m]; n];
        let inside = |x, y| {
            0 <= x && x < n as i32 && 0 <= y && y < m as i32
        };
        for positions in antennae.into_values() {
            for pos1 in &positions {
                for pos2 in &positions {
                    if pos1 == pos2 {
                        continue
                    }
                    let (x, y) = (pos1.0 + 2 * (pos2.0 - pos1.0), pos1.1 + 2 * (pos2.1 - pos1.1));
                    if !inside(x, y) {
                        continue
                    }
                    antinode[x as usize][y as usize] = true;
                }
            }

        }
        antinode.into_iter().map(|v| {
           v.into_iter().map(|b| if b { 1 } else { 0 }).sum::<i32>()
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

pub(crate) struct PartTwo;

impl PartTwo {
    
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
    
    fn solve(&self, input: Input) -> Output {
        let (n, m) = (input.field.len(), input.field[0].len());
        let mut antennae = HashMap::new();
        for (i, line) in (0..).zip(input.field.into_iter()) {
            for (j, c) in (0..).zip(line.into_iter()) {
                if let Some(c) = c {
                    antennae.entry(c).or_insert(Vec::new()).push((i, j));
                }
            }
        }
        let mut antinode = vec![vec![false; m]; n];
        let inside = |x, y| {
            0 <= x && x < n as i32 && 0 <= y && y < m as i32
        };
        for positions in antennae.into_values() {
            for pos1 in &positions {
                for pos2 in &positions {
                    if pos1 == pos2 {
                        continue
                    }
                    let (x, y) = (pos1.0 + 2 * (pos2.0 - pos1.0), pos1.1 + 2 * (pos2.1 - pos1.1));
                    let (dx, dy) = (pos2.0 - pos1.0, pos2.1 - pos1.1);
                    let (mut x, mut y) = (pos1.0, pos1.1);
                    while inside(x, y) {
                        antinode[x as usize][y as usize] = true;
                        x += dx;
                        y += dy;
                    }
                }
            }

        }
        antinode.into_iter().map(|v| {
            v.into_iter().map(|b| if b { 1 } else { 0 }).sum::<i32>()
        }).sum()
    }
}

impl Solvable for PartTwo {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}