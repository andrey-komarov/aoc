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

pub(crate) struct PartTwo{}

impl PartTwo {

    fn rotate(field: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let (rows, cols) = (field.len(), field[0].len());
        let field2 = (0..cols).map(
            |col| {
                (0..rows).map(|row| field[rows - row - 1][col]).collect()
            }
        ).collect();
        field2
    }

    fn find_top_left_corners(field: &Vec<Vec<usize>>, corners: &mut Vec<i32>) {
        let (n, m) = (field.len(), field[0].len());
        for j in 0..m {
            if j > 0 && field[0][j] == field[0][j - 1] {
                continue;
            }
            corners[field[0][j]] += 1;
        }
        for i in 1..n {
            corners[field[i][0]] += if field[i - 1][0] == field[i][0] { 0 } else { 1 }
        }
        for i in 1..n {
            for j in 1..m {
                if field[i][j] == field[i][j - 1] {
                    continue;
                }
                let good = field[i][j] == field[i - 1][j] && field[i][j] != field[i - 1][j - 1];
                corners[field[i][j]] += if good { 0 } else { 1 }
            }
        }
    }

    fn solve(&self, input: Input) -> Output {
        let (n, m) = (input.field.len(), input.field[0].len());
        let mut colour = vec![vec![0; m]; n];
        let mut current_colour = 0;
        for i in 0..n {
            for j in 0..m {
                if colour[i][j] != 0 {
                    continue;
                }
                current_colour += 1;
                colour[i][j] = current_colour;
                let mut queue = VecDeque::new();
                queue.push_back((i as i32, j as i32));
                while let Some((x, y)) = queue.pop_front() {
                    for (dx, dy) in [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)] {
                        let (new_x, new_y) = (x + dx, y + dy);
                        if !(0 <= new_x && new_x < n as i32 && 0 <= new_y && new_y < m as i32) 
                          || input.field[x as usize][y as usize] != input.field[new_x as usize][new_y as usize] {
                            continue;
                        }
                        if colour[new_x as usize][new_y as usize] == 0 {
                            colour[new_x as usize][new_y as usize] = current_colour;
                            queue.push_back((new_x, new_y));
                        }
                    }
                }
            }
        }
        // println!("{:?}", colour);
        let mut corners = vec![0; current_colour + 1];
        let mut c2 = colour;
        for i in 0..4 {
            Self::find_top_left_corners(&c2, &mut corners);
            // println!("{:?}", corners);
            c2 = Self::rotate(c2);
        }
        let mut area = vec![0; current_colour + 1];
        for row in c2 {
            for x in row {
                area[x] += 1;
            }
        }
        corners.into_iter().zip(area).map(|(a, b)| a * b).sum()
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
