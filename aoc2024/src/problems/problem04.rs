use std::io::{BufRead, Write};
use crate::problems::common::{Readable, Solvable};

struct Input {
    field: Vec<Vec<char>>,
}

impl Input {
    fn rotate(&self) -> Input {
        let (rows, cols) = (self.field.len(), self.field[0].len());
        let field2 = (0..cols).map(
            |col| {
                (0..rows).map(|row| self.field[rows - row - 1][col]).collect()
            }
        ).collect();
        Input { field: field2 }
    }

    fn mirror(&self) -> Input {
        let (rows, cols) = (self.field.len(), self.field[0].len());
        let field2 = (0..rows).map(
            |row| {
                (0..cols).map(|col| self.field[row][cols - col - 1]).collect()
            }
        ).collect();
        Input { field: field2 }
    }
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
    fn count_horizontal(input: &Input, s: &Vec<char>) -> i32 {
        let (rows, cols) = (input.field.len(), input.field[0].len());
        (0..rows).map(
            |row| {
                (0..cols - s.len() + 1).map(|col| {
                    let is_match = (0..s.len()).all(|i| {
                        input.field[row][col + i] == s[i]
                    });
                    if is_match { 1 } else { 0 }
                }).sum::<i32>()
            }
        ).sum()
    }

    fn count_diagonal(input: &Input, s: &Vec<char>) -> i32 {
        let (rows, cols) = (input.field.len(), input.field[0].len());
        (0..rows - s.len() + 1).map(
            |row| {
                (0..cols - s.len() + 1).map(|col| {
                    let is_match = (0..s.len()).all(|i| {
                        input.field[row + i][col + i] == s[i]
                    });
                    if is_match { 1 } else { 0 }
                }).sum::<i32>()
            }
        ).sum()
    }
    fn solve(&self, input: Input) -> Output {
        let mut input2 = input;
        let mut sum = 0;
        let xmas = "XMAS".chars().collect();
        for i in (0..4) {
            sum += Self::count_horizontal(&input2, &xmas);
            sum += Self::count_diagonal(&input2, &xmas);
            input2 = input2.rotate()
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

pub(crate) struct PartTwo {}

impl PartTwo {

    fn count_match(input: &Input, s: &Vec<Vec<char>>) -> i32 {
        let (rows, cols) = (input.field.len(), input.field[0].len());
        let (n, m) = (s.len(), s[0].len());
        (0..rows - n + 1).map(
            |row| {
                (0..cols - n + 1).map(|col| {
                    let is_match = (0..n).all(|i| {
                        (0..m).all(|j| {
                            s[i][j] == '.' || input.field[row + i][col + j] == s[i][j]
                        })
                    });
                    if is_match { 1 } else { 0 }
                }).sum::<i32>()
            }
        ).sum()
    }
    fn solve(&self, input: Input) -> Output {
        let mut input2 = input;
        let mut sum = 0;
        let xmas = vec![
            "M.S".chars().collect(),
            ".A.".chars().collect(),
            "M.S".chars().collect(),
        ];
        for i in (0..4) {
            sum += Self::count_match(&input2, &xmas);
            input2 = input2.rotate()
        }
        sum
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
#[cfg(test)]
mod tests {
    use crate::problems::problem04::*;

    #[test]
    fn rotate() {
        // ABC  -> DA
        // DEF     EB
        //         FC
        let input: Input = Input::parse_from("ABC\nDEF".as_bytes()).unwrap();
        let input2 = input.rotate();
        let input3: Input = Input::parse_from("DA\nEB\nFC".as_bytes()).unwrap();
        assert_eq!(input2.field, input3.field);
    }

    #[test]
    fn mirror() {
        let input: Input = Input::parse_from("ABC\nDEF".as_bytes()).unwrap();
        let input2 = input.mirror();
        let input3: Input = Input::parse_from("CBA\nFED".as_bytes()).unwrap();
        assert_eq!(input2.field, input3.field);
    }
}