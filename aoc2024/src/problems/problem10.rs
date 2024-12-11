use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, Write};
use crate::problems::common::{Readable, Solvable};

struct Input {
    field: Vec<Vec<i32>>,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let field = s.lines().filter(|s| !s.is_empty()).map(|s| {
            s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
        }).collect();
        Ok(Input {field})
    }
}

type Output = i32;

pub(crate) struct PartOne;

impl PartOne {
    fn solve(&self, input: Input) -> Output {
        let (n, m) = (input.field.len(), input.field[0].len());
        let mut sum = 0;
        for i in (0..n) {
            for j in (0..m) {
                if input.field[i][j] != 0 {
                    continue;
                }
                let mut visited = vec![vec![false; n]; m];
                let mut nines = 0;
                let mut queue = VecDeque::new();
                queue.push_back((i as i32, j as i32));
                visited[i][j] = true;
                while let Some((x, y)) = queue.pop_front() {
                    if input.field[x as usize][y as usize] == 9 {
                        nines += 1;
                    }
                    for (dx, dy) in [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)] {
                        let (new_x, new_y)  = (x + dx, y + dy);
                        if !(0 <= new_x && new_x < n as i32 && 0 <= new_y && new_y < m as i32) {
                            continue;
                        }
                        let h_old = input.field[x as usize][y as usize];
                        let h_new = input.field[new_x as usize][new_y as usize];
                        if !visited[new_x as usize][new_y as usize] && h_old + 1 == h_new {
                            visited[new_x as usize][new_y as usize] = true;
                            queue.push_back((new_x, new_y));
                        }
                    }
                }
                // println!("New {} {} {}", i, j, nines);
                sum += nines;
            }
        }
        sum
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
