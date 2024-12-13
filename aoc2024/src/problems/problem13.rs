use std::{cmp::min, io::{BufRead, Write}};

use nom::{bytes::{is_not, take_while}, character::complete::newline, combinator::{eof, map_res}, multi::{many0, many_till, separated_list0}, IResult, Parser};

use super::common::{Readable, Solvable};


#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Claw {
    a: Point, 
    b: Point,
    target: Point,
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

fn point(input: &str) -> IResult<&str, Point> {
    // let skip = many0(is_not("0123456789"));
    let (input, (_, x, _, y, _)) = 
         (many0(is_not("0123456789")),
          number,
            many0(is_not("0123456789")),
           number, newline).parse(input)?;
    Ok((input, Point {x, y}))
}

fn claw(input: &str) -> IResult<&str, Claw> {
    let (input, (a, b, target, _)) = (point, point, point, newline).parse(input)?;
    Ok((input, Claw {a, b, target}))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
        let (input, ((claws, _) )) = 
            (many_till(claw, eof))
        .parse(input)?;
        Ok((input, Input {claws}))
}

#[derive(Debug)]
struct Input {
    claws: Vec<Claw>,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let (_, r) = parse_input.parse(&s).map_err(|e| anyhow::anyhow!("lol {}", e))?;
        Ok(r)
    }
}

type Output = i64;

pub(crate) struct PartOne {}

impl PartOne {
    fn check(claw: &Claw) -> Option<i64> {
        let limit = min(claw.target.x / claw.a.x, claw.target.y / claw.a.y);
        let mut possible = Vec::new();
        for a in 0..(limit+1) {
            let (x, y) = (claw.target.x - claw.a.x * a, claw.target.y - claw.a.y * a);
            if x % claw.b.x != 0 || y % claw.b.y != 0 {
                continue;
            }
            let b = x / claw.b.x;
            if x != b * claw.b.x || y != b * claw.b.y {
                continue;
            }
            possible.push(3 * a + b);
        }
        possible.into_iter().min()
    }

    fn solve(&self, input: Input) -> Output {
        // println!("{:?}", input);
        input.claws.into_iter().map(|claw| {
            if Self::check(&claw) != check2(&claw) {
                println!("BAD {:?} ", claw)
            }
            if claw.a.x * claw.b.y == claw.a.y * claw.b.x {
                println!("DEPENDENT :( {:?}", claw)
            }
            Self::check(&claw).unwrap_or(0)
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

fn check2(claw: &Claw) -> Option<i64> {
    // x1 * A + x2 * B = x
    // y1 * A + y2 * B = y
    //
    // x1 y1 A + x2 y1 B = x y1
    // y1 x1 A + y2 x1 B = y x1
    //
    //       0 + (y2 x1 - x2 y1) B = y x1 - x y1
    // B = (y x1 - x y1) / (y2 x1 - x2 y1)
    let b = (claw.a.x * claw.target.y - claw.a.y * claw.target.x) / 
            (claw.a.x * claw.b.y - claw.a.y * claw.b.x);
    let a = (claw.target.x - claw.b.x * b) / claw.a.x;
    if a < 0 || b < 0 || claw.a.x * a + claw.b.x * b != claw.target.x || claw.a.y * a + claw.b.y * b != claw.target.y {
        None
    } else {
        Some(3 * a + b)
    }
}

impl PartTwo {
    fn solve(&self, input: Input) -> Output {
        // println!("{:?}", input);
        input.claws.into_iter().map(|claw| {
            check2(&claw).unwrap_or(0)
        }).sum()
    }
}

impl Solvable for PartTwo {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let input = Input { claws: input.claws.into_iter().map(|Claw{a, b, target}| {
            Claw {
                a: a,
                b: b,
                target: Point {
                    x: target.x + 10000000000000,
                    y: target.y + 10000000000000,
                },
            }
        }).collect()};
        let out = self.solve(input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}
