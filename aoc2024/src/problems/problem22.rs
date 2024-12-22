use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Write};
use crate::problems::common::{Readable, Solvable};

struct Input {
    secrets: Vec<u64>,
}

impl Readable for Input {
    fn parse_from<R: BufRead>(mut input: R) -> anyhow::Result<Self>
    where
        Self: Sized
    {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        let secrets = s.lines().filter(|l| !l.is_empty()).map(str::parse::<u64>).collect::<Result<_, _>>()?;
        Ok(Input { secrets })
    }
}

struct PRNG {
    seed: u64,
    state: u64,
}

impl PRNG {

    const MOD: u64 = 16777216;

    fn new(seed: u64) -> Self {
        Self { seed, state: seed }
    }

    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x = ((x * 64) ^ x) % Self::MOD;
        x = ((x / 32) ^ x) % Self::MOD;
        x = ((x * 2048) ^ x) % Self::MOD;
        self.state = x;
        x
    }
}

pub(crate) struct PartOne {
    steps: usize,
}

type Output = u64;

impl PartOne {
    pub(crate) fn new(steps: usize) -> Self {
        Self { steps }
    }

    fn solve(&self, input: &Input) -> Output {
        input.secrets.iter().map(|secret| {
            let mut prng = PRNG::new(*secret);
            let mut x = 0;
            for _ in 0..self.steps {
                x = prng.next();
            }
            x
        }).sum()
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

impl PartTwo {
    pub(crate) fn new(steps: usize) -> Self {
        Self { steps }
    }

    fn bananas(num: u64) -> i32 {
        (num % 10) as i32
        // format!("{}", num).chars().filter(|c| *c == '1').count() as i32
    }

    fn changes_and_prices(&self, secret: u64) -> Vec<(i32, i32)> {
        let mut prices = Vec::new();
        let mut prng = PRNG::new(secret);
        let mut x = secret;
        for _ in 0..=self.steps {
            prices.push(Self::bananas(x));
            x = prng.next();
        }
        prices.iter().skip(1).zip(prices.iter()).map(|(a, b)| {
            *a as i32 - *b as i32
        }).zip(prices.iter().skip(1)).map(|(change, price)| (change, *price)).collect()
    }

    fn solve(&self, input: &Input) -> Output {
        let mut total = HashMap::new();
        for secret in input.secrets.iter() {
            let mut seen = HashSet::new();
            let x =  self.changes_and_prices(*secret);
            let i1 = x.iter();
            let i2 = x.iter().skip(1);
            let i3 = x.iter().skip(2);
            let i4 = x.iter().skip(3);
            i1.zip(i2).zip(i3).zip(i4).for_each(|(((v1, v2), v3), v4)| {
                let quad = (v1.0, v2.0, v3.0, v4.0);
                let price = v4.1;
                if !seen.contains(&quad) {
                    seen.insert(quad);
                    *total.entry(quad).or_insert(0) += price;
                }
            })
        }
        *total.values().max().unwrap() as Output
    }
}

pub(crate) struct PartTwo {
    steps: usize,
}

impl Solvable for PartTwo {
    fn solve<R: BufRead, W: Write>(&self, input: R, mut output: W) -> anyhow::Result<()> {
        let input = Input::parse_from(input)?;
        let out = self.solve(&input);
        writeln!(output, "{}", out)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::problem22::*;

    #[test]
    fn test_prng() {
        let mut prng = PRNG::new(123);
        let next = prng.next();
        assert_eq!(next, 15887950);
    }

    #[test]
    fn test_part2() {
        let p2 = PartTwo::new(10);
        let c = p2.changes_and_prices(123);
        println!("{:?}", c);
        assert_eq!(1, 1);
    }
}
