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

#[cfg(test)]
mod tests {
    use crate::problems::problem22::*;

    #[test]
    fn test_prng() {
        let mut prng = PRNG::new(123);
        let next = prng.next();
        assert_eq!(next, 15887950);
    }
}
