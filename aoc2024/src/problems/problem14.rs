use std::io::{BufRead, Write};

use nom::{bytes::{is_not, tag, take_while}, character::complete::newline, combinator::{eof, map_res, opt}, multi::{many0, many_till}, IResult, Parser};

use super::common::{Readable, Solvable};


#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: Point,
}

#[derive(Debug)]
struct Input {
    robots: Vec<Robot>,
}

fn number(input: &str) -> IResult<&str, i64> {
    fn is_digit(c: char) -> bool {
        c.is_digit(10) || c == '-'
    }
    map_res(
        take_while(is_digit),
        str::parse
    ).parse(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    // let skip = many0(is_not("0123456789"));
    let (input, (_, x, _, y)) = 
         (many0(is_not("0123456789-")),
          number,
            many0(is_not("0123456789-")),
           number).parse(input)?;
    Ok((input, Point {x, y}))
}

fn robot(input: &str) -> IResult<&str, Robot> {
    let (input, (position, velocity, _)) = (point, point, newline).parse(input)?;
    Ok((input, Robot{position, velocity}))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
        let (input, ((robots, _) )) = 
            (many_till(robot, eof))
        .parse(input)?;
        Ok((input, Input {robots}))
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

pub(crate) struct PartOne {
    n: usize,
    m: usize,
    steps: usize,
}

impl PartOne {
    pub fn new(n: usize, m: usize) -> Self {
        PartOne {n, m, steps: 100}
    }
}

type Output = i64;

impl PartOne {
    fn solve(&self, input: Input) -> Output {
        println!("{:?}", input);
        let wrap = |position, velocity, steps, size| {
            ((position + velocity * steps as i64) % size as i64 + size as i64) as usize % size
        };
        let mut count = vec![0; 5];
        let positions = input.robots.into_iter().map(|r| {
            let x = wrap(r.position.x, r.velocity.x, self.steps, self.n);
            let y = wrap(r.position.y, r.velocity.y, self.steps, self.m);
            let qx = if x < self.n / 2 {
                0
            } else if x > self.n / 2 {
                1
            } else {
                2
            };
            let qy = if y < self.m / 2 {
                0
            } else if y > self.m / 2 {
                1
            } else {
                2
            };
            match (qx, qy) {
                (0, 0) => 0,
                (0, 1) => 1,
                (1, 0) => 2,
                (1, 1) => 3,
                _ => 4,
            }
        }).for_each(|q| {
            count[q] += 1;
        });
        count[0] * count[1] * count[2] * count[3]
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
