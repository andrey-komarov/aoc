use std::io::{BufRead, Write};
use std::ptr::eq;
use nom::bytes::{tag, take_while};
use nom::combinator::{eof, map_res};
use nom::{IResult, Parser};
use nom::character::char;
use nom::character::complete::{line_ending, newline};
use nom::multi::{many0, many1, many_till, separated_list1};
use crate::problems::common::{Readable, Solvable};

struct Equation {
    target: i64,
    numbers: Vec<i64>,
}
struct Input {
    equations: Vec<Equation>,
}

fn number(input: &str) -> IResult<&str, i64> {
    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }
    map_res(
        take_while(is_digit),
        str::parse
    ).parse(input)
}

fn equation(input: &str) -> IResult<&str, Equation> {
    let (input, (target, _, numbers, _)) = (
        number,
        tag(": "),
        separated_list1(char(' '), number),
        line_ending,
    ).parse(input)?;
    Ok((input, Equation { target, numbers }))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, ((equations, _), _)) = (
        many_till(equation, eof),
        eof,
    ).parse(input)?;
    Ok((input, Input { equations }))
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let (_, r) = parse_input(&*s).map_err(|e| anyhow::anyhow!("lol {}", e))?;
        Ok(r)
    }
}
type Output = i64;

pub(crate) struct PartOne {}

impl PartOne {

    fn is_valid(equation: &Equation) -> bool {
        fn go(head: i64, tail: &[i64], target: i64) -> bool {
            if tail.len() == 0 {
                head == target
            } else {
                go(head + tail[0], &tail[1..], target) || go(head * tail[0], &tail[1..], target)
            }
        };
        go(equation.numbers[0], &equation.numbers[1..], equation.target)
    }
    fn solve(&self, input: Input) -> Output {
        input.equations.into_iter().filter(Self::is_valid).map(|eq| {eq.target}).sum()
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

    fn is_valid(equation: &Equation) -> bool {
        fn concat(lhs: i64, rhs: i64) -> i64 {
            format!("{}{}", lhs, rhs).parse().unwrap()
        }
        fn go(head: i64, tail: &[i64], target: i64) -> bool {
            if tail.len() == 0 {
                head == target
            } else {
                go(head + tail[0], &tail[1..], target) 
                    || go(head * tail[0], &tail[1..], target)
                    || go(concat(head, tail[0]), &tail[1..], target)
            }
        };
        go(equation.numbers[0], &equation.numbers[1..], equation.target)
    }
    fn solve(&self, input: Input) -> Output {
        input.equations.into_iter().filter(Self::is_valid).map(|eq| {eq.target}).sum()
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
