use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, Write};
use crate::problems::common::{Readable, Solvable};

enum Cell {
    Empty, Wall
}

struct Pos {
    x: usize,
    y: usize,
}

struct Input {
    field: Vec<Vec<Cell>>,
    start: Pos,
    finish: Pos,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let (mut start, mut finish) = (None, None);
        let field = s.lines().filter(|line| !line.is_empty()).enumerate().map(|(i, line)| {
            line.chars().enumerate().map(|(j, cell)| {
                match cell {
                    '.' => Some(Cell::Empty),
                    '#' => Some(Cell::Wall),
                    'S' => {
                        start.insert(Pos {x: i, y: j});
                        Some(Cell::Empty)
                    },
                    'E' => {
                        finish.insert(Pos {x: i, y: j});
                        Some(Cell::Empty)
                    }
                    _ => None,
                }
            }).collect()
        }).collect::<Option<_>>().ok_or_else(|| anyhow::anyhow!("cannot parse field"))?;
        let start = start.ok_or_else(|| anyhow::anyhow!("no start"))?;
        let finish = finish.ok_or_else(|| anyhow::anyhow!("no finish"))?;
        Ok(Input {
            field, start, finish
        })
    }
}

pub(crate) struct PartOne {}

type Output = usize;

impl PartOne {

    fn inside(input: &Input, x: isize, y: isize) -> bool {
        let (n, m) = (input.field.len(), input.field[0].len());
        0 <= x && x < n as isize && 0 <= y && y < m as isize
    }

    fn bfs(input: &Input, start: &Pos) -> Vec<Vec<Option<usize>>> {
        let (n, m) = (input.field.len(), input.field[0].len());
        let mut dist = vec![vec![None; m]; n];
        dist[start.x][start.y] = Some(0);
        let mut queue = VecDeque::from([(start.x, start.y)]);
        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let x2 = x as isize + dx;
                let y2 = y as isize + dy;
                if !Self::inside(input, x2, y2) {
                    continue;
                }
                let (x2, y2) = (x2 as usize, y2 as usize);
                if let Cell::Wall = input.field[x2][y2] {
                    continue;
                }
                if dist[x2][y2].is_none() {
                    dist[x2][y2] = dist[x][y].map(|d| d + 1);
                    queue.push_back((x2, y2));
                }
            }
        }
        dist
    }
    fn solve(&self, input: Input) -> Output {
        let forward = Self::bfs(&input, &input.start);
        let backward = Self::bfs(&input, &input.finish);
        let (n, m) = (input.field.len(), input.field[0].len());
        let mut shortcuts = HashMap::new();
        let dist = forward[input.finish.x][input.finish.y].unwrap();
        println!("Finish in {}, start {:?}", dist, backward[input.start.x][input.start.y]);
        for i in 0..n {
            for j in 0..m {
                for (dx, dy) in [(-2, 0), (2, 0), (0, -2), (0, 2)] {
                    let (x2, y2) = (i as isize + dx, j as isize + dy);
                    if !Self::inside(&input, x2, y2) {
                        continue;
                    }
                    let (x2, y2) = (x2 as usize, y2 as usize);
                    if let Some(d) = forward[i][j] {
                        if let Some(d2) = backward[x2][y2] {
                            let short = d + d2 + 2;
                            if short < dist {
                                *shortcuts.entry(dist - short).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
        println!("{:?}", shortcuts);
        shortcuts.into_iter().filter_map(|(k, v)| {
            if k >= 100 {
                Some(v)
            } else {
                None
            }
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
