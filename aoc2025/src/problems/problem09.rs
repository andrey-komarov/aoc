use std::cmp::{max, min, PartialEq};
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::io::BufRead;
use anyhow::Context;
use crate::problems::common::Problem;

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
}

pub struct Input {
    points: Vec<Point>,
}

pub struct Problem09 {
    is_part1: bool,
}

impl Problem09 {
    pub fn new() -> Self { Self { is_part1: true } }
    pub fn new_part2() -> Self { Self { is_part1: false } }
}

#[derive(Clone, Eq, PartialEq)]
enum Cell {
    Unknown, Outside, Boundary
}

struct Polygon {
    x_map: HashMap<i64, usize>,
    y_map: HashMap<i64, usize>,
    field: Vec<Vec<Cell>>
}

impl Polygon {
    fn new(input: &Input) -> Self {
        let x = BTreeSet::from_iter(input.points.iter().map(|p| p.x));
        let y = BTreeSet::from_iter(input.points.iter().map(|p| p.y));
        let x: Vec<_> = x.clone().into_iter().chain([-1, x.iter().max().unwrap() + 1]).chain(x.into_iter().map(|v| {
            v + 1
        })).collect::<BTreeSet<_>>().into_iter().collect();
        let y: Vec<_> = y.clone().into_iter().chain([-1, y.iter().max().unwrap() + 1]).chain(y.into_iter().map(|v| {
            v + 1
        })).collect::<BTreeSet<_>>().into_iter().collect();
        let x_map = HashMap::from_iter(x.iter().enumerate().map(|(i, &p)| (p, i)));
        let y_map = HashMap::from_iter(y.iter().enumerate().map(|(i, &p)| (p, i)));


        let mut field = vec![vec![Cell::Unknown; x.len()]; y.len()];
        for (i, p) in input.points.iter().enumerate() {
            let p2 = &input.points[(i + 1) % input.points.len()];
            let x1 = x_map[&min(p.x, p2.x)];
            let x2 = x_map[&max(p.x, p2.x)];
            let y1 = y_map[&min(p.y, p2.y)];
            let y2 = y_map[&max(p.y, p2.y)];
            for xx in x1..=x2 {
                for yy in y1..=y2 {
                    field[yy][xx] = Cell::Boundary;
                }
            }
        }
        let mut queue = VecDeque::new();
        // assume 0, 0 is outside (lol)
        queue.push_back((0, 0));
        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in [(-1isize, 0), (1, 0), (0, -1isize), (0, 1)] {
                let (x2, y2) = (x + dx, y + dy);
                if !(0 <= x2 && x2 < field[0].len() as isize && 0 <= y2 && y2 < field.len() as isize) {
                    continue
                }
                if let Cell::Unknown = field[y2 as usize][x2 as usize] {
                    field[y2 as usize][x2 as usize] = Cell::Outside;
                    queue.push_back((x2, y2));
                }
            }
        }

        Self {
            x_map,
            y_map,
            field,
        }
    }

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
        if self.is_part1 {
            input.points.iter().map(|p1| {
                input.points.iter().map(|p2| {
                    let dx = (p2.x - p1.x).abs() + 1;
                    let dy = (p2.y - p1.y).abs() + 1;
                    dx * dy
                }).max().unwrap()
            }).max().unwrap()
        } else {
            let polygon = Polygon::new(&input);
            input.points.iter().map(|p1| {
                input.points.iter().filter_map(|p2| {
                    let (x1, x2) = (polygon.x_map[&p1.x], polygon.x_map[&p2.x]);
                    let (x1, x2) = (min(x1, x2), max(x1, x2));
                    let (y1, y2) = (polygon.y_map[&p1.y], polygon.y_map[&p2.y]);
                    let (y1, y2) = (min(y1, y2), max(y1, y2));
                    if (x1..=x2).any(|xx| {
                        (y1..=y2).any(|yy| {
                            polygon.field[yy][xx] == Cell::Outside
                        })
                    }) {
                        return None
                    }
                    let dx = (p2.x - p1.x).abs() + 1;
                    let dy = (p2.y - p1.y).abs() + 1;
                    Some(dx * dy)
                }).max().unwrap_or(0)
            }).max().unwrap_or(0)
        }
    }
}