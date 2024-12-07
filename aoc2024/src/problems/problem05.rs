use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{BufRead, Write};
use nom::Parser;
use nom::bytes::take_while;
use nom::character::char;
use nom::character::complete::{line_ending, newline};
use nom::combinator::{eof, map_res};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use crate::problems::common::{Readable, Solvable};

struct Order {
    before: i32,
    after: i32,
}

struct Input {
    limits: Vec<Order>,
    updates: Vec<Vec<i32>>,
}

fn number(input: &str) -> IResult<&str, i32> {
    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }
    map_res(
        take_while(is_digit),
        str::parse
    ).parse(input)
}

fn order(input: &str) -> IResult<&str, Order> {
    let (input, (before, _, after, _)) = (
        number,
        char('|'),
        number,
        line_ending,
    ).parse(input)?;
    Ok((input, Order { before, after }))
}

fn update(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, (v, _)) = (
        separated_list1(char(','), number),
        line_ending
    ).parse(input)?;
    Ok((input, v))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, (limits, _, updates)) = (
        many1(order),
        line_ending,
        many1(update),
    ).parse(input)?;
    Ok((input, Input {limits, updates }))
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

type Output = i32;

pub(crate) struct PartOne {}

impl PartOne {
    fn solve(&self, input: Input) -> Output {
        let good: HashSet<(i32, i32)> = HashSet::from_iter(
            input.limits.into_iter().map(|Order { before, after }| {
                (before, after)
            }));
        input.updates.into_iter().filter(|u| {
            (1..u.len()).all(|i| {
                (0..i).all(|j| {
                    !good.contains(&(u[i], u[j]))
                })
            })
        }).map(|u| {
            u[u.len() / 2]
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

pub(crate) struct PartTwo {}

impl PartTwo {
    fn solve(&self, input: Input) -> Output {
        let good: HashSet<(i32, i32)> = HashSet::from_iter(
            input.limits.into_iter().map(|Order { before, after }| {
                (before, after)
            }));
        input.updates.into_iter().filter(|u| {
            (1..u.len()).any(|i| {
                (0..i).any(|j| {
                    good.contains(&(u[i], u[j]))
                })
            })
        }).map(|mut u| {
            u.sort_by(|a, b| {
                match (a == b, good.contains(&(*a, *b))) {
                    (true, _) => Ordering::Equal,
                    (_, true) => Ordering::Less,
                    (_, false) => Ordering::Greater,
                }
            });
            u
        }).map(|u| {
                u[u.len() / 2]
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
