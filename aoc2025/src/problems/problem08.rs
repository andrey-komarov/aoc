use std::collections::HashMap;
use std::io::BufRead;
use anyhow::Context;
use crate::problems::common::Problem;

struct Point {
    coords: [i64; 3],
}

impl Point {
    fn dist2(&self, other: &Point) -> i64 {
        self.coords.iter().zip(other.coords.iter()).map(|(a, b)| (a - b) * (a - b)).sum()
    }
}

pub struct Input {
    points: Vec<Point>,
}

pub(crate) struct Problem08 {
    is_part1: bool,
    steps: usize,
}

impl Problem08 {
    pub fn new(steps: usize) -> Self { Self { is_part1: true, steps} }

    pub fn new_part2() -> Self { Self { is_part1: false, steps: 0} }

    pub fn solve_part1(&self, input: Input) -> <Problem08 as Problem>::Output {
        let mut dsu = DSU::new(input.points.len());
        let mut distances = Vec::new();
        for i in 0..input.points.len() {
            for j in i + 1..input.points.len() {
                distances.push((input.points[i].dist2(&input.points[j]), i, j))
            }
        }
        distances.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, (_d, p1, p2)) in (0..self.steps).zip(distances) {
            dsu.merge(p1, p2).unwrap();
        }
        let mut count = HashMap::new();
        for i in 0..input.points.len() {
            let class = dsu.get_class(i).unwrap();
            *count.entry(class).or_insert(0) += 1;
        }
        let mut entries = count.iter().collect::<Vec<_>>();
        entries.sort_by(|a, b| a.1.cmp(b.1).reverse());
        (0..3).zip(entries).map(|(_, (_, count))| {
            count
        }).product()
    }

    pub fn solve_part2(&self, input: Input) -> <Problem08 as Problem>::Output {
        let mut dsu = DSU::new(input.points.len());
        let mut distances = Vec::new();
        for i in 0..input.points.len() {
            for j in i + 1..input.points.len() {
                distances.push((input.points[i].dist2(&input.points[j]), i, j))
            }
        }
        distances.sort_by(|a, b| a.0.cmp(&b.0));
        let mut merges = 0;
        for (_d, p1, p2) in distances {
            if dsu.get_class(p1).unwrap() != dsu.get_class(p2).unwrap() {
                dsu.merge(p1, p2).unwrap();
                merges += 1;
                if merges == input.points.len() - 1 {
                    return (input.points[p1].coords[0] * input.points[p2].coords[0]) as u64
                }
            }
        }
        unreachable!()
    }
}

struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    pub fn new(size: usize) -> Self {
        Self { parent: (0..size).collect() }
    }

    pub fn get_class(&mut self, x: usize) -> anyhow::Result<usize> {
        let parent = *self.parent.get(x).with_context(|| "OOB")?;
        if parent == x {
            return Ok(x)
        }
        let parent = self.get_class(parent)?;
        if parent != x {
            self.parent[x] = parent;
        }
        Ok(parent)
    }

    pub fn merge(&mut self, x: usize, y: usize) -> anyhow::Result<()> {
        let p1 = self.get_class(x)?;
        let p2 = self.get_class(y)?;
        if p1 != p2 {
            self.parent[p1] = p2;
        }
        Ok(())
    }
}

impl Problem for Problem08 {
    type Input = Input;
    type Output = u64;

    fn parse_from<R: BufRead>(&self, buf: R) -> anyhow::Result<Self::Input> {
        let mut points = Vec::new();
        for line in buf.lines() {
            let line = line?;
            let p = line.split(',').map(|s| {
                s.parse::<i64>().with_context(|| format!("could not parse number {}", s))
            }).collect::<anyhow::Result<Vec<_>>>()?;

            let p: anyhow::Result<_> = p.as_slice().try_into().with_context(|| "lol");
            points.push(Point { coords: p? });
        }
        Ok(Input { points })
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        if self.is_part1 {
            self.solve_part1(input)
        } else {
            self.solve_part2(input)
        }
    }
}