use std::io::{BufRead, Write};
use crate::problems::common::{Readable, Solvable};
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub(crate) struct Input {
    a: i32,
    b: i32,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Input>
    {
        let mut s = String::new();
        input.read_line(&mut s)?;
        let nums: Vec<i32> = s.split_whitespace().map(|s| {
            s.parse().map_err(anyhow::Error::new)
        }).collect::<Result<_>>()?;
        Ok(Input { a: nums[0], b: nums[1] })
    }
}

type Output = i32;

pub(crate) struct Problem {}

impl Problem {
    fn solve(&self, input: Input) -> Output {
        input.a + input.b
    }
}

impl Solvable for Problem {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use crate::problems::common::Readable;
    use crate::problems::problem00::{Input, Problem};

    #[test]
    fn execute_test() {
        let input = Input { a: 1, b: 2 };
        let problem = Problem {};
        assert_eq!(problem.solve(input), 3);
    }

    #[test]
    fn parse_test() {
        let text = "1 2";
        let input = Input::parse_from(BufReader::new(text.as_bytes())).unwrap();
        assert_eq!(input, Input { a: 1, b: 2 });
    }
}