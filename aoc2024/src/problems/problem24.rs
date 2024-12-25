use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, Write};
use nom::branch::alt;
use nom::bytes::tag;
use nom::{IResult, Parser};
use nom::character::complete::{alphanumeric1, newline};
use nom::multi::many1;
use nom::sequence::terminated;
use crate::problems::common::{Readable, Solvable};

#[derive(Debug, Clone)]
enum GateOp {
    Or, Xor, And
}

impl GateOp {
    fn apply(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            GateOp::Or => lhs || rhs,
            GateOp::Xor => lhs ^ rhs,
            GateOp::And => lhs && rhs,
        }
    }

    fn shape(&self) -> String {
        match self {
            GateOp::Or => String::from("diamond"),
            GateOp::Xor => String::from("circle"),
            GateOp::And => String::from("box"),
        }
    }
}

#[derive(Debug, Clone)]
struct Gate {
    op: GateOp,
    lhs: String,
    rhs: String,
    target: String,
}

impl Gate {
    fn swap(&self, node1: &String, node2: &String) -> Gate {
        if self.target == *node1 {
            Self {target: node2.clone(), ..self.clone()}
        } else if self.target == *node2 {
            Self {target: node1.clone(), ..self.clone()}
        } else {
            self.clone()
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    gates: Vec<Gate>,
    start: HashMap<String, bool>,
}

impl Input {

    fn check_loop(v: &String, gates: &Vec<Gate>) -> bool {
        let mut up = HashMap::new();
        for gate in gates {
            up.entry(gate.target.clone()).or_insert_with(Vec::new);
            up.entry(gate.target.clone()).or_insert_with(Vec::new).push(gate.lhs.clone());
            up.entry(gate.target.clone()).or_insert_with(Vec::new).push(gate.rhs.clone());
        }
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        if let Some(parents) = up.get(v) {
            for parent in parents {
                queue.push_back(parent);
            }
        }

        while let Some(node) = queue.pop_front() {
            if node == v {
                return true;
            }
            if let Some(parents) = up.get(node) {
                for parent in parents {
                    if !visited.contains(parent) {
                        visited.insert(parent.clone());
                        queue.push_back(parent);
                    }
                }
            }
        }
        false
    }

    fn swap(&self, node1: String, node2: String) -> Option<Self> {
        let gates = self.gates.iter().map(|g| g.swap(&node1, &node2)).collect();

        if Self::check_loop(&node1, &gates) {
            return None
        }
        if Self::check_loop(&node2, &gates) {
            return None
        }

        Some(Self {
            gates,
            start: self.start.clone(),
        })
    }
}

fn gate_op(input: &str) -> IResult<&str, GateOp> {
    alt((
        tag("XOR").map(|_| GateOp::Xor),
        tag("OR").map(|_| GateOp::Or),
        tag("AND").map(|_| GateOp::And),
    )).parse(input)
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        fn start(input: &str) -> IResult<&str, (String, bool)> {
            let (input, (name, _, val, _)) = (
                alphanumeric1,
                tag(": "),
                alphanumeric1,
                newline,
            ).parse(input)?;
            Ok((input, (name.to_string(), if val == "0" { false } else {true})))
        };
        fn gate(input: &str) -> IResult<&str, Gate> {
            let (input, (lhs, op, rhs, target)) = (
                terminated(alphanumeric1, tag(" ")),
                terminated(gate_op, tag(" ")),
                terminated(alphanumeric1, tag(" -> ")),
                terminated(alphanumeric1, newline),
            ).parse(input)?;
            Ok((input, Gate { lhs: lhs.to_string(), rhs: rhs.to_string(), op, target: target.to_string()}))
        }
        let  (input, (start, gates)) = (
            terminated(many1(start), newline),
            terminated(many1(gate), newline),
            ).parse(&*s).map_err(|e| anyhow::anyhow!("Can't parse: {}", e))?;
        Ok(Input {
            gates: gates,
            start: HashMap::from_iter(start.into_iter()),
        })
    }
}

pub(crate) struct PartOne {}

type Output = u64;

struct PartOneMut {
    cache: HashMap<String, bool>,
    targets: HashMap<String, Gate>,
}

impl PartOneMut {
    fn new(input: Input) -> Self {
        Self {
            cache: HashMap::from_iter(input.start.into_iter()),
            targets: HashMap::from_iter(input.gates.into_iter().map(|g| {
                (g.target.clone(), g)
            })),
        }
    }

    fn calc(&mut self, target: &String) -> bool {
        if let Some(val) = self.cache.get(target) {
            *val
        } else {
            let gate = self.targets.get(target).unwrap().clone();
            let lhs = self.calc(&gate.lhs);
            let rhs = self.calc(&gate.rhs);
            let res = gate.op.apply(lhs, rhs);
            self.cache.insert(target.clone(), res);
            res
        }
    }

    fn solve(&mut self) -> Output {
        let mut res = self.targets.clone().values().filter_map(|g| {
            if g.target.starts_with("z") {
                Some((g.target.clone(), self.calc(&g.target)))
            } else {
                None
            }
        }).collect::<Vec<_>>();
        res.sort_by(|a, b| b.0.cmp(&a.0));
        // println!("{:?}", res);
        let n =
            res.iter().map(|(_, b)| {
                if *b { "1" } else { "0" }
            }).collect::<Vec<_>>().join("").to_string();
        // println!("{}", n);
        u64::from_str_radix(
            res.into_iter().map(|(_, b)| {
                if b { "1" } else { "0" }
            }).collect::<Vec<_>>().join("").as_str(),
            2).unwrap()
    }
}

impl PartOne {
    fn solve(&self, input: Input) -> Output {
        // println!("{:?}", input);
        PartOneMut::new(input).solve()
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

pub(crate) struct PartTwo {}

impl PartTwo {
    fn solve(&self, input: Input) -> Output {
        PartTwoMut::new(input).solve()
    }
}

struct PartTwoMut {
    n: usize,
    value: HashMap<String, bool>,
    links: HashMap<String, Vec<Gate>>,
    input: Input,
}

impl PartTwoMut {
    fn new(input: Input) -> Self {
        let copy = input.clone();
        let mut links = HashMap::new();
        let mut value = HashMap::new();
        for gate in input.gates {
            links.entry(gate.lhs.clone()).or_insert_with(Vec::new).push(gate.clone());
            links.entry(gate.rhs.clone()).or_insert_with(Vec::new).push(gate.clone());
            links.entry(gate.target.clone()).or_insert_with(Vec::new);
            value.insert(gate.target, false);
        }
        let n = input.start.len() / 2;
        for (start, _) in input.start.into_iter() {
            value.insert(start, false);
        }
        Self {
            n,
            value,
            links,
            input: copy,
        }
    }

    fn xyz(&self) -> (u64, u64, u64) {
        let get = |name, n| {
            let mut x = 0;
            for i in 0..n {
                x |= (if *self.value.get(&format!("{}{:0>2}", name, i)).unwrap() {1} else {0}) << i;
            }
            x
        };
        (get("x", self.n), get("y", self.n), get("z", self.n + 1))
    }

    fn ok(&self) -> bool {
        let (x, y, z) = self.xyz();
        x + y == z
    }

    fn flip(&mut self, target: &String, new_val: bool) -> (bool, Vec<String>) {
        // println!("flip: {} {:?}", target, self.value.iter().filter(|(k, v)| {**v}).collect::<Vec<_>>());
        // println!("   xyz {:?}", self.xyz());
        let mut updated = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(target);
        self.value.insert(target.to_string(), new_val);
        while let Some(node) = queue.pop_front() {
            // println!("GO FROM {} {:?}", node, self.links.get(node));
            for gate in self.links.get(node).unwrap() {
                let cur_val = *self.value.get(&gate.target).unwrap();
                let lhs = *self.value.get(&gate.lhs).unwrap();
                let rhs = *self.value.get(&gate.rhs).unwrap();
                let new_val = gate.op.apply(lhs, rhs);
                if new_val != cur_val {
                    queue.push_back(&gate.target);
                    self.value.insert(gate.target.clone(), new_val);
                    updated.push(gate.target.clone());
                }
            }
        }
        (self.ok(), updated)
    }

    fn score(&mut self) -> usize {
        let mut count = 0;
        for i in 0..self.n {
            let x = format!("x{:0>2}", i);
            let (ok, updated) = self.flip(&x, true);
            if !ok {
                // println!("{}: {:?} {:?}", i, updated, self.xyz());
                count += 1;
            }
            self.flip(&x, false);
        }
        count
    }

    fn find_best_score(&mut self, change: String) {
        let mut best = 100;
        for node in self.input.gates.iter() {
            let input2 = self.input.swap(change.to_string(), node.target.clone());
            if let Some(input2) = input2 {
                let mut p2 = PartTwoMut::new(input2);
                let score = p2.score();
                if score <= best {
                    best = score;
                    println!("Check {} - {}: {}", change, node.target, score);
                }
            }
        }
        println!("SCORE {}", self.score());
    }

    fn solve(&mut self) -> Output {
        for i in 0..self.n {
            let x = format!("x{:0>2}", i);
            let (ok, updated) = self.flip(&x, true);
            if !ok {
                println!("{}: {:?} {:?}", i, updated, self.xyz());
            } else {
                let y = format!("y{:0>2}", i);
                let (ok, updated) = self.flip(&y, true);
                if !ok {
                    println!("Step 2 {}: {:?} {:?}", i, updated, self.xyz());
                }
                self.flip(&y, false);
            }
            self.flip(&x, false);
        }
        // self.find_best_score("mwh".to_string());
        println!("SCORE {}", self.score());
        0
    }
}

impl Solvable for PartTwo {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let input = input.swap("fhc".to_string(), "z06".to_string()).unwrap();
        let input = input.swap("qhj".to_string(), "z11".to_string()).unwrap();
        let input = input.swap("hqk".to_string(), "z35".to_string()).unwrap();
        let input = input.swap("mwh".to_string(), "ggt".to_string()).unwrap();

        for g in input.gates.iter() {
            writeln!(output, "{} [shape={}]", g.target, g.op.shape());
            writeln!(output, "{} -> {};", g.lhs, g.target);
            writeln!(output, "{} -> {};", g.rhs, g.target);
        }

        write!(output, "{{ rank = same; ");
        for i in 0..44 {
            write!(output, "x{:0>2}; ", i);
        }
        writeln!(output, "}};");
        write!(output, "{{ rank = same; ");
        for i in 0..44 {
            write!(output, "y{:0>2}; ", i);
        }
        writeln!(output, "}};");
        write!(output, "{{ rank = same; ");
        for i in 0..45 {
            write!(output, "z{:0>2}; ", i);
        }
        writeln!(output, "}};");


        let out = self.solve(input);
        writeln!(output, "{}", out)?;

        Ok(())
    }
}
