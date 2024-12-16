use std::{collections::{BTreeMap, BTreeSet, HashSet}, io::{BufRead, Write}};

use nom::multi;

use super::common::{Readable, Solvable};


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Direction {
    Left, Up, Right, Down,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
        }
    }

    fn clockwise(&self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }

    fn counterclockwise(&self) -> Self {
        self.clockwise().clockwise().clockwise()
    }

    pub fn any() -> Vec<Self> {
        vec![Direction::Left, Direction::Up, Direction::Right, Direction::Down]
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Pos {
    x: usize,
    y: usize,
    direction: Direction,
}

enum Cell {
    Empty, Wall
}

struct Input {
    field: Vec<Vec<Cell>>,
    start: Pos,
    finish: BTreeSet<Pos>,
}

impl Readable for Input {
    fn parse_from<R: std::io::BufRead>(mut input: R) -> anyhow::Result<Self> where Self: Sized {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let mut start = None;
        let mut end = None;
        let field = (0..).zip(s.lines().filter(|s| !s.is_empty())).map(|(i, line)| {
            (0..).zip(line.chars()).map(|(j, c)| {
                match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'S' => {
                        start = Some((i, j));
                        Cell::Empty
                    },
                    'E' => {
                        end = Some((i, j));
                        Cell::Empty
                    },
                    _ => panic!()
                }
            }).collect()
        }).collect();
        let start = start.ok_or_else(|| anyhow::anyhow!("Can't find start"))?;
        let end = end.ok_or_else(|| anyhow::anyhow!("Can't find end"))?;
        Ok(Input {
            field,
            start: Pos {
                x: start.0, y: start.1, direction: Direction::Right,
            },
            finish: Direction::any().into_iter().map(|d| {
                Pos {
                    x: end.0, y: end.1, direction: d
                }
            }).collect(),
        })
    }
}

type Output = i32;

pub(crate) struct PartOne {}

impl PartOne {
    fn solve(&self, input: &Input) -> Output {
        let mut queue = BTreeSet::new();
        queue.insert((0, input.start.clone()));
        let mut dist = BTreeMap::from([(input.start.clone(), 0)]);
        while let Some((cur_dist, pos)) = queue.pop_first() {
            if input.finish.contains(&pos) {
                return cur_dist;
            }
            let (dx, dy) = pos.direction.offset();
            let (x2, y2) = ((pos.x as isize+ dx) as usize, (pos.y as isize + dy) as usize);
            let mut candidates = Vec::new();
            candidates.push((cur_dist + 1000, Pos {direction: pos.direction.clockwise(), ..pos }));
            candidates.push((cur_dist + 1000, Pos {direction: pos.direction.counterclockwise(), ..pos }));
            if let Cell::Empty = input.field[x2][y2] {
                candidates.push((cur_dist + 1, Pos {x: x2, y: y2, direction: pos.direction}));
            }
            for (new_dist, new_pos) in candidates {
                if let Some(old_dist) = dist.get(&new_pos) {
                    if old_dist <= &new_dist {
                        continue;
                    }
                }
                dist.insert(new_pos.clone(), new_dist);
                queue.insert((new_dist, new_pos));
            }
        }
        panic!()
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

fn dijkstra(field: &Vec<Vec<Cell>>, start: Vec<Pos>, multiplier: isize) -> BTreeMap<Pos, i32> {
    let mut queue = BTreeSet::new();
    let mut dist = BTreeMap::new();
    for start in start {
        queue.insert((0, start.clone()));
        dist.insert(start.clone(), 0);
    }
    while let Some((cur_dist, pos)) = queue.pop_first() {
        let (dx, dy) = pos.direction.offset();
        let (dx, dy) = (dx * multiplier, dy * multiplier);
        let (x2, y2) = ((pos.x as isize+ dx) as usize, (pos.y as isize + dy) as usize);
        let mut candidates = Vec::new();
        candidates.push((cur_dist + 1000, Pos {direction: pos.direction.clockwise(), ..pos }));
        candidates.push((cur_dist + 1000, Pos {direction: pos.direction.counterclockwise(), ..pos }));
        if let Cell::Empty = field[x2][y2] {
            candidates.push((cur_dist + 1, Pos {x: x2, y: y2, direction: pos.direction}));
        }
        for (new_dist, new_pos) in candidates {
            if let Some(old_dist) = dist.get(&new_pos) {
                if old_dist <= &new_dist {
                    continue;
                }
            }
            dist.insert(new_pos.clone(), new_dist);
            queue.insert((new_dist, new_pos));
        }
    }
    dist
}

impl PartTwo {
    fn solve(&self, input: &Input) -> Output {
        let dist = dijkstra(&input.field, vec![input.start.clone()], 1);
        let answer = input.finish.iter().map(|pos| {
            dist.get(pos).unwrap()
        }).min().unwrap();
        // println!("answer {:?}", answer);
        let rdist = dijkstra(&input.field, input.finish.clone().into_iter().collect(), -1);
        (0..).zip(input.field.iter()).map(|(x, line)| {
            (0..).zip(line.into_iter()).map(|(y, c)| {
                let on_optimal_path = Direction::any().into_iter().flat_map(|direction| {
                    let pos = Pos {x, y, direction};
                    let d = dist.get(&pos)?;
                    let rd = rdist.get(&pos)?;
                    Some(d + rd)
                }).any(|d| d == *answer);
                if on_optimal_path { 1 } else { 0 }
            }).sum::<Output>()
        }).sum()
    }
}

impl Solvable for PartTwo {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(&input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}
