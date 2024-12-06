use std::io::{BufRead, Write};
use crate::problems::common::{Readable, Solvable};
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::{tag, take, take_while_m_n};
use nom::character::{char};
use nom::combinator::{eof, map_res};
use nom::multi::{many0, many_till};

#[derive(Debug, PartialEq)]
struct Mul {
    lhs: i32,
    rhs: i32,
}
struct Input {
    muls: Vec<Mul>,
}

fn mul(input: &str) -> IResult<&str, Mul> {
    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }
    let number = || map_res(
        take_while_m_n(1, 3, is_digit),
        |s: &str| s.parse::<i32>()
    );
    let (input, (_, lhs, _, rhs, _)) = (
        tag("mul("),
        number(),
        char(','),
        number(),
        char(')'),
    ).parse(input)?;
    Ok((input, Mul { lhs, rhs }))
}


impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let mut parser = many_till(alt((
                mul.map(Some),
                take(1usize).map(|_| None),
            )), eof);
        let (_, (x, _)) = parser.parse(&*s).map_err(|e| anyhow::anyhow!("lol {}", e))?;
        let y = x.into_iter().filter_map(|x| x).collect();
        Ok(Input { muls: y })
    }
}


type Output = i32;

pub(crate) struct PartOne {}

impl PartOne {
    fn solve(&self, input: &Input) -> Output {
        input.muls.iter().map(|mul| mul.lhs * mul.rhs).sum()
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

pub(crate) struct PartTwo {}

enum Instruction {
    Mul(Mul),
    Do,
    Dont,
}

struct InputPartTwo {
    instructions: Vec<Instruction>,
}

impl Readable for InputPartTwo {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let mut parser = many_till(alt((
            mul.map(|x| Some(Instruction::Mul(x))),
            tag("do()").map(|_| Some(Instruction::Do)),
            tag("don't()").map(|_| Some(Instruction::Dont)),
            take(1usize).map(|_| None),
        )), eof);
        let (_, (x, _)) = parser.parse(&*s).map_err(|e| anyhow::anyhow!("lol {}", e))?;
        let y = x.into_iter().filter_map(|x| x).collect();
        Ok(InputPartTwo { instructions: y })
    }
}
impl PartTwo {
    fn solve(&self, input: &InputPartTwo) -> Output {
        let mut enabled = true;
        let mut sum = 0;
        for instruction in &input.instructions {
            match instruction {
                Instruction::Mul(mul) => {
                    if enabled {
                        sum += mul.lhs * mul.rhs;
                    }
                }
                Instruction::Do => {
                    enabled = true;
                }
                Instruction::Dont => {
                    enabled = false;
                }
            }
        }
        sum
    }
}

impl Solvable for PartTwo {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = InputPartTwo::parse_from(input)?;
        let out = self.solve(&input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use nom::combinator::eof;
    use nom::multi::{many0, many_till};
    use crate::problems::problem03::*;

    #[test]
    fn parse_mul_ok() {
        let res = mul("mul(1,3)");
        assert_eq!(Ok(("", Mul{lhs: 1, rhs: 3})), res);
    }

    #[test]
    fn parse_take() {
        fn parser(s: &str) -> IResult<&str, &str> {
            take(1usize).parse(s)
        }
        let mut parser = alt((
            mul.map(Some),
            take(1usize).map(|_| None),
        ));
        let res = parser.parse("mul(2,1000)");
        assert_eq!(Ok(("ul(2,1000)", None)), res);
    }

    #[test]
    fn parse_many_ok() {
        let mut parser = many_till(alt((
            mul.map(Some),
            take(1usize).map(|_| None),
        )), eof);
        let Ok((_, (res, _))) = parser.parse("aaaamul(1,2)aa") else { todo!() };
        assert_eq!(vec![None, None, None, None, Some(Mul{lhs: 1, rhs: 2}), None, None], res);
    }
}