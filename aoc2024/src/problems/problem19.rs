use std::collections::HashMap;
use std::io::{BufRead, Write};
use crate::problems::common::{Readable, Solvable};

#[derive(Debug)]
struct Input {
    towels: Vec<String>,
    designs: Vec<String>,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let (towels, designs) = s.split_once("\n\n").ok_or_else(|| anyhow::anyhow!("Parse error"))?;
        let towels = towels.split(", ").map(str::to_string).collect();
        let designs = designs.lines().filter(|s| !s.is_empty()).map(str::to_string).collect();
        Ok(Input {towels, designs})
    }
}

struct TrieNode {
    terminal: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode { terminal: false, children: HashMap::new() }
    }

    fn go(&mut self, ch: char) -> &mut Self {
        self.children.entry(ch).or_insert_with(TrieNode::new)
    }

    fn try_go(&self, ch: char) -> Option<&Self> {
        self.children.get(&ch)
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::new() }
    }

    fn insert(&mut self, s: &str) {
        let mut node = &mut self.root;
        for c in s.chars() {
            node = node.go(c);
        }
        node.terminal = true;
    }

    fn prefixes(&self, s: &str) -> Vec<usize> {
        let mut res = Vec::new();
        let mut node = &self.root;
        for (i, ch) in s.chars().enumerate() {
            if let Some(next) = node.try_go(ch) {
                node = next;
                if node.terminal {
                    res.push(i + 1);
                }
            } else {
                break;
            }
        }
        res
    }
}

pub(crate) struct PartOne {}

type Output = usize;

impl PartOne {

    fn check(words: &Trie, design: &str) -> bool {
        let chars = design.chars().collect::<Vec<_>>();
        let mut reachable = vec![false; chars.len() + 1];
        reachable[0] = true;
        for (i, char) in chars.into_iter().enumerate() {
            if !reachable[i] {
                continue;
            }
            for j in words.prefixes(&design[i..]) {
                reachable[i + j] = true;
            }
        }
        *reachable.last().unwrap()
    }
    fn solve(&self, input: &Input) -> Output {
        let mut words = Trie::new();
        input.towels.iter().for_each(|t| words.insert(t.as_str()));
        input.designs.iter().filter(|d| Self::check(&words, d.as_str())).count()
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

impl PartTwo {

    fn check(words: &Trie, design: &str) -> usize {
        let chars = design.chars().collect::<Vec<_>>();
        let mut reachable = vec![0; chars.len() + 1];
        reachable[0] = 1;
        for (i, char) in chars.into_iter().enumerate() {
            for j in words.prefixes(&design[i..]) {
                reachable[i + j] += reachable[i];
            }
        }
        *reachable.last().unwrap()
    }
    fn solve(&self, input: &Input) -> Output {
        let mut words = Trie::new();
        input.towels.iter().for_each(|t| words.insert(t.as_str()));
        input.designs.iter().map(|d| Self::check(&words, d.as_str())).sum()
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
