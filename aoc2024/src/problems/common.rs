use std::io::{BufRead, Write};
use anyhow::Result;

pub trait Readable {
    fn parse_from<R: BufRead>(input: R) -> Result<Self> where Self: Sized;
}

pub trait Solvable {
    fn solve<R: BufRead, W: Write>(&self, input: R, output: W) -> Result<()>;
}