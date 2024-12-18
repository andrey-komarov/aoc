use std::io::{BufRead, Write};
use std::str::FromStr;
use nom::Parser;
use nom::bytes::{tag, take_while};
use nom::character::complete::newline;
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use crate::problems::common::{Readable, Solvable};

#[derive(Debug, Clone)]
struct Input {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    program: Vec<u8>,
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }
    map_res(
        take_while(is_digit),
        str::parse
    ).parse(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    fn parse_register<'a>(input: &'a str, reg: &'a str) -> IResult<&'a str, u64> {
        delimited(
            tag(format!("Register {}: ", reg).as_str()),
            number,
            newline).parse(input)
    }
    fn parse_program(input: &str) -> IResult<&str, Vec<u8>> {
        separated_list1(tag(","), number).parse(input)
    }

    fn parse_reg<'a>(reg: String) -> impl Fn(&'a str) -> IResult<&'a str, u64> {
        let parse_register = move |input: &'a str| -> IResult<&'a str, u64> {
            delimited(
                tag(format!("Register {}: ", reg).as_str()),
                number,
                newline).parse(input)
        };;
        parse_register
    };

    let (input, (reg_a, reg_b, reg_c, _, _, program)) = (
        parse_reg("A".to_string()),
        parse_reg("B".to_string()),
        parse_reg("C".to_string()),
        newline,
        tag("Program: "),
        parse_program,
        ).parse(input)?;
    Ok((input, Input { reg_a, reg_b, reg_c, program }))
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let (_, input) = parse_input(&s).map_err(|e| anyhow::anyhow!("lol {}", e))?;
        Ok(input)
    }
}

type Output = Vec<u8>;
type Output2 = u64;

pub(crate) struct PartOne {}

impl PartOne {
    fn solve(&self, input: &Input) -> Output {
        // println!("{:?}", input);
        let mut reg_a = input.reg_a;
        let mut reg_b = input.reg_b;
        let mut reg_c = input.reg_c;
        let mut ip = 0;
        let mut out: Output = Vec::new();
        let combo = |x, reg_a, reg_b, reg_c| -> u64 {
            match x {
                0..=3 => x as u64,
                4 => reg_a,
                5 => reg_b,
                6 => reg_c,
                _ => unreachable!(),
            }
        };
        let x = loop {
            match (input.program.get(ip), input.program.get(ip + 1)) {
                (Some(0), Some(&x)) => {
                    reg_a >>= combo(x, reg_a, reg_b, reg_c);
                },
                (Some(1), Some(&x)) => {
                    reg_b ^= x as u64;
                },
                (Some(2), Some(&x)) => {
                    reg_b = combo(x, reg_a, reg_b, reg_c) % 8;
                },
                (Some(3), Some(&x)) => {
                    if reg_a != 0 {
                        ip = x as usize;
                        continue;
                    }
                },
                (Some(4), Some(_)) => {
                    reg_b ^= reg_c;
                },
                (Some(5), Some(&x)) => {
                    out.push((combo(x, reg_a, reg_b, reg_c) % 8) as u8);
                },
                (Some(6), Some(&x)) => {
                    reg_b = reg_a >> combo(x, reg_a, reg_b, reg_c);
                },
                (Some(7), Some(&x)) => {
                    reg_c = reg_a >> combo(x, reg_a, reg_b, reg_c);
                },
                _ => break,
            };
            ip += 2;
        };
        // println!("{:?}", out);
        out
    }
}

impl Solvable for PartOne {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(&input);
        let out = out.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        writeln!(output, "{}", out)?;
        Ok(())
    }
}

pub(crate) struct PartTwo {}

impl PartTwo {
    fn solve(&self, input: &Input) -> Output2 {
        let test = |a| {
            let p = PartOne{};
            p.solve(&Input { reg_a: a, ..input.clone() })
        };
        let mut answers = Vec::new();
        let mut candidates = Vec::from_iter(0..256u64);
        for (i, digit) in input.program.iter().enumerate() {
            let shift = 8 + 3 * i;
            candidates = candidates.iter().flat_map(|c| {
                (0..8).map(|new_digit| {
                    c.clone() | (new_digit << shift)
                })
            }).filter(|c| {
                test(*c).iter().take(i).zip(&input.program).all(|(a, b)| { *a == *b })
            }).collect();
            for c in &candidates {
                if test(c.clone()) == input.program {
                    answers.push(c.clone());
                }
            }
        }
        // println!("{:?}", answers);
        // println!("{:?}", answers.iter().min());
        answers.into_iter().min().unwrap()
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
