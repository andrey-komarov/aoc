use std::io::BufRead;
use anyhow::Context;
use crate::problems::common::Problem;

struct Point {
    x: i64,
    y: i64,
}

pub struct Input {
    points: Vec<Point>,
}

pub struct Problem09 {}

impl Problem09 {
    pub fn new() -> Self { Self {} }
}

impl Problem for Problem09 {
    type Input = Input;
    type Output = i64;

    fn parse_from<R: BufRead>(&self, buf: R) -> anyhow::Result<Self::Input> {
        let mut points = Vec::new();
        for line in buf.lines() {
            let line = line?;
            let (x, y) = line.split_once(",").with_context(|| "lol")?;
            points.push(Point {
                x: x.parse()?,
                y: y.parse()?,
            })
        }
        Ok(Input { points })
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        input.points.iter().map(|p1| {
            input.points.iter().map(|p2| {
                let dx = (p2.x - p1.x).abs() + 1;
                let dy = (p2.y - p1.y).abs() + 1;
                dx * dy
            }).max().unwrap()
        }).max().unwrap()
    }
}