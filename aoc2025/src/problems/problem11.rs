use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use anyhow::Context;
use crate::problems::common::Problem;

type Vertex = String;

pub struct Input {
    edges: HashMap<Vertex, Vec<Vertex>>,
}

impl Input {
    fn dfs<'a>(&'a self, v: &'a Vertex, used: &mut HashSet<&'a Vertex>, order: &mut Vec<&'a Vertex>) {
        if let Some(edges) = self.edges.get(v) {
            for to in edges {
                if !used.contains(to) {
                used.insert(to);
                self.dfs(to, used, order);
                }
            }
            order.push(v);
        }
    }

    fn paths(&self, from: &Vertex, to: &Vertex) -> usize {
        let mut order = Vec::new();
        let mut visited = HashSet::new();
        self.dfs(from, &mut visited, &mut order);
        let mut paths = HashMap::new();
        paths.insert(to, 1);
        for v in order {
            for to in &self.edges[v] {
                *paths.entry(v).or_insert(0) += paths[to];
            }
        }
        paths[from]
    }
}

pub struct Problem11 {
}

impl Problem11 {
    pub fn new() -> Self { Self {} }
}

impl Problem for Problem11 {
    type Input = Input;
    type Output = usize;

    fn parse_from<R: BufRead>(&self, buf: R) -> anyhow::Result<Self::Input> {
        let mut edges = HashMap::new();
        for line in buf.lines() {
            let line = line?;
            let (from, to) = line.split_once(": ").with_context(|| "edges")?;
            edges.entry(from.to_string()).or_insert_with(Vec::new).extend(to.split_whitespace().map(String::from));
        }
        Ok(Input{edges})
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        input.paths(&String::from("you"), &String::from("out"))
    }
}