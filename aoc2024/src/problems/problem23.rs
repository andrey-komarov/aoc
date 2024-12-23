use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Write};
use crate::problems::common::{Readable, Solvable};

struct Input {
    edges: Vec<(String, String)>,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let edges = s.lines().filter(|line| !line.is_empty()).map(|line| {
            let (s1, s2) = line.split_once('-')?;
            Some((s1.to_string(), s2.to_string()))
        }).collect::<Option<_>>().ok_or_else(|| anyhow::anyhow!("can't parse edges"))?;
        Ok(Input {edges})
    }
}

pub(crate) struct PartOne {}

type Output = usize;

impl PartOne {

    fn solve(&self, input: &Input) -> Output {
        let mut graph = HashMap::new();
        let mut vertices = HashSet::new();
        for edge in input.edges.iter() {
            graph.entry(&edge.0).or_insert_with(Vec::new).push(&edge.1);
            graph.entry(&edge.1).or_insert_with(Vec::new).push(&edge.0);
            vertices.insert(&edge.0);
            vertices.insert(&edge.1);
        }
        let mut triples = Vec::new();
        for v1 in vertices {
            for &v2 in graph.get(&v1).unwrap() {
                if v2 <= &v1 {
                    continue;
                }
                for &v3 in graph.get(&v1).unwrap() {
                    if v3 <= v2 {
                        continue;
                    }
                    if !graph.get(v2).unwrap().contains(&v3) {
                        continue;
                    }
                    triples.push((v1, v2, v3));
                }
            }
        }
        triples.into_iter().filter(|&(v1, v2, v3)| {
            v1.starts_with("t") || v2.starts_with("t") || v3.starts_with("t")
        }).count()
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
