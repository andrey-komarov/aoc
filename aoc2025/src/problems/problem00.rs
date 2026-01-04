use std::io::BufRead;
use crate::problems::common::Problem;

pub struct Problem00 {}

impl Problem for Problem00 {
    type Input = (i32, i32);
    type Output = i32;

    fn parse_from<R: BufRead>(&self, mut buf: R) -> anyhow::Result<(i32, i32)> {
        let mut s = String::new();
        buf.read_to_string(&mut s)?;
        let v =
            s.split_whitespace().map(|w| {
                w.parse::<i32>().map_err(anyhow::Error::new)
            }).collect::<Result<Vec<_>, _>>()?;
        Ok((v[0], v[1]))
    }

    fn solve(&self, input: Self::Input) -> Self::Output {
        input.0 + input.1
    }
}
