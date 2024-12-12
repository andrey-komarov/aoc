use std::{collections::VecDeque, io::{BufRead, Write}};

use super::common::{Readable, Solvable};

struct Input {
    field: Vec<Vec<char>>,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let field = s.lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().collect()).collect();
        Ok(Input { field })
    }
}

pub(crate) struct PartOne{}

type Output = i32;

impl PartOne {
    fn solve(&self, input: Input) -> Output {
        let (n, m) = (input.field.len(), input.field[0].len());
        let mut visited = vec![vec![false; m]; n];
        let mut total = 0;
        for i in 0..n {
            for j in 0..m {
                if visited[i][j] {
                    continue;
                }
                visited[i][j] = true;
                let (mut area, mut perimeter) = (0, 0);
                let mut queue = VecDeque::new();
                queue.push_back((i as i32, j as i32));
                while let Some((x, y)) = queue.pop_front() {
                    area += 1;
                    for (dx, dy) in [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)] {
                        let (new_x, new_y) = (x + dx, y + dy);
                        if !(0 <= new_x && new_x < n as i32 && 0 <= new_y && new_y < m as i32) 
                          || input.field[x as usize][y as usize] != input.field[new_x as usize][new_y as usize] {
                            perimeter += 1;
                            continue;
                        }
                        if !visited[new_x as usize][new_y as usize] {
                            visited[new_x as usize][new_y as usize] = true;
                            queue.push_back((new_x, new_y));
                        }
                    }
                }
                total += area * perimeter;
            }
        }
        total
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
