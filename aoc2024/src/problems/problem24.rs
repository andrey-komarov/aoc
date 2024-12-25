use std::collections::HashMap;
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
}

#[derive(Debug, Clone)]
struct Gate {
    op: GateOp,
    lhs: String,
    rhs: String,
    target: String,
}

#[derive(Debug)]
struct Input {
    gates: Vec<Gate>,
    start: HashMap<String, bool>,
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
        println!("{:?}", res);
        let n =
            res.iter().map(|(_, b)| {
                if *b { "1" } else { "0" }
            }).collect::<Vec<_>>().join("").to_string();
        println!("{}", n);
        u64::from_str_radix(
            res.into_iter().map(|(_, b)| {
                if b { "1" } else { "0" }
            }).collect::<Vec<_>>().join("").as_str(),
            2).unwrap()
    }
}

impl PartOne {
    fn solve(&self, input: Input) -> Output {
        println!("{:?}", input);
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
