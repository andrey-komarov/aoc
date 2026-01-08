use std::array;
use std::collections::HashSet;
use std::io::BufRead;
use anyhow::Context;
use crate::problems::common::Problem;

type Fig = [[bool; 3]; 3];

struct Present {
    id: usize,
    shape: Fig,
}

impl Present {
    fn new(shape: Fig) -> Self {
        Self { id: 0, shape }
    }
    fn flip(&self) -> Fig {
        array::from_fn(|i| {
            array::from_fn(|j| {
                self.shape[i][self.shape[i].len() - j - 1]
            })
        })
    }

    fn rotate(&self) -> Fig {
        array::from_fn(|i| {
            array::from_fn(|j| {
                self.shape[self.shape.len() - j - 1][i]
            })
        })
    }

    fn variants(&self) -> Vec<Fig> {
        let mut set = HashSet::new();
        let mut fig = self.shape;
        for _ in 0..4 {
            set.insert(fig);
            let p = Present::new(fig);
            set.insert(p.flip());
            fig = p.rotate();
        }
        set.into_iter().collect()
    }

    fn weight(&self) -> usize {
        self.shape.map(|line| {
            line.into_iter().filter(|b| *b).count()
        }).into_iter().sum()
    }
}

struct Region {
    dimensions: [usize; 2],
    present_counts: Vec<usize>,
}

pub struct Input {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

struct Bruteforcer<'a> {
    input: &'a Input,
    region: &'a Region,

    variants: Vec<Vec<Fig>>,
    field: Vec<Vec<bool>>,
    remaining_presents: Vec<usize>,
    skippable_remaining: usize,
}

impl<'a> Bruteforcer<'a> {
    fn new(input: &'a Input, region: &'a Region) -> Self {
        Self {
            input,
            region,
            variants: input.presents.iter().map(Present::variants).collect(),
            field: vec![vec![false; region.dimensions[1]]; region.dimensions[0]],
            remaining_presents: region.present_counts.clone(),
            skippable_remaining: 0,
        }
    }

    fn next_pos(&self, pos: [usize; 2]) -> [usize; 2] {
        if pos[1] + 1 == self.region.dimensions[1] {
            [pos[0] + 1, 0]
        } else {
            [pos[0], pos[1] + 1]
        }
    }

    fn can_fit(&self, pos: [usize; 2], shape: &Fig) -> bool {
        if pos[0] + shape.len() > self.region.dimensions[0] {
            return false;
        }
        if pos[1] + shape[0].len() > self.region.dimensions[1] {
            return false;
        }
        shape.iter().enumerate().all(|(i, row)| {
            row.iter().enumerate().all(|(j, cell)| {
                // println!("!!!! {} {} {} {}", pos[0] + i, pos[1] + j, self.field.len(), self.field[0].len());
                !cell || !self.field[pos[0] + i][pos[1] + j]
            })
        })
    }

    fn xor(&mut self, pos: [usize; 2], shape: &Fig) {
        shape.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, cell)| {
                if *cell {
                    self.field[pos[0] + i][pos[1] + j] ^= true
                }
            })
        })
    }

    fn go(&mut self, pos: [usize; 2]) -> bool {
        if self.remaining_presents.iter().all(|p| *p == 0) {
            return true;
        }
        if pos[0] >= self.region.dimensions[0] {
            return false;
        }
        let mut pos = pos;
        while self.field[pos[0]][pos[1]] {
            pos = self.next_pos(pos);
        }
        let next_pos = self.next_pos(pos);
        let res = self.variants.clone().into_iter().enumerate().zip(self.remaining_presents.clone().into_iter()).any(|((i, variants), remaining)| {
            if remaining == 0 {
                return false
            }
            variants.iter().any(|variant| {
                if !self.can_fit(pos, variant) {
                    return false
                }
                self.xor(pos, variant);
                self.remaining_presents[i] -= 1;
                let res = self.go(next_pos);
                self.xor(pos, variant);
                self.remaining_presents[i] += 1;
                res
            })
        });
        if res {
            return true
        }
        if self.skippable_remaining == 0 {
            return false
        }
        self.skippable_remaining -= 1;
        let res = self.go(next_pos);
        self.skippable_remaining += 1;
        res
    }

    fn solve(mut self) -> bool {
        let area_needed: usize = self.input.presents.iter().zip(self.region.present_counts.iter()).map(|(p, c)| {
            p.weight() * c
        }).sum();
        let total_area = self.region.dimensions.iter().product();
        if area_needed > total_area {
            return false;
        }
        self.skippable_remaining = total_area - area_needed;
        let res = self.go([0, 0]);
        res
    }

}

pub struct Problem12 {
}

impl Problem12 {
    pub fn new() -> Self { Self {} }
}

impl Problem for Problem12 {
    type Input = Input;
    type Output = usize;

    fn parse_from<R: BufRead>(&self, mut buf: R) -> anyhow::Result<Self::Input> {
        let mut s = String::new();
        buf.read_to_string(&mut s)?;
        let mut presents = Vec::new();
        let mut regions = Vec::new();
        for block in s.split("\n\n") {
            if block.contains("x") {
                for block in block.lines() {
                    let (dimensions, counts) = block.split_once(": ").with_context(|| "dim: cnt")?;
                    let (d1, d2) = dimensions.split_once("x").with_context(|| "dim")?;
                    let (d1, d2) = (d1.parse()?, d2.parse()?);
                    let present_counts = counts.split_whitespace().map(|c| {
                        c.parse::<usize>().with_context(|| "count")
                    }).collect::<anyhow::Result<_>>()?;
                    regions.push(Region {
                        dimensions: [d1, d2],
                        present_counts,
                    })
                }
            } else {
                let mut lines: Vec<_> = block.lines().collect();
                let id = lines.remove(0);
                let id = id.trim_end_matches(":").parse()?;
                let lines: Vec<Vec<_>> = lines.into_iter().map(|line| {
                    line.chars().collect()
                }).collect();
                let shape = array::from_fn(|i| {
                    array::from_fn(|j| {
                        lines[i][j] == '#'
                    })
                });
                presents.push(Present {
                    id,
                    shape,
                })
            }
        }
        Ok(Input {
            presents,
            regions,
        })
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        for (i, present) in input.presents.iter().enumerate() {
            println!("Present {} has {} variants", i, present.variants().len())
        }

        let mut count = 0;
        for (i, region) in input.regions.iter().enumerate() {
            println!("Solving region {} ({} x {})", i, region.dimensions[0], region.dimensions[1]);
            let bruteforcer = Bruteforcer::new(&input, region);
            if bruteforcer.solve() {
                println!("it fits");
                count += 1;
            }
        }
        count
    }
}