use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub trait Problem {
    type Input;
    type Output;

    fn parse_from<R: BufRead>(&self, buf: R) -> anyhow::Result<Self::Input>;
    fn solve(&self, input: Self::Input) -> Self::Output;
}

pub fn solve<P: Problem>(problem: &P, in_filename: PathBuf, out_filename: PathBuf) -> anyhow::Result<()> where
    P::Output: ToString {
    let in_file = BufReader::new(File::open(in_filename)?);
    let input = problem.parse_from(in_file)?;
    let output = problem.solve(input);
    let mut output_file = File::create(out_filename)?;
    output_file.write_all(output.to_string().as_bytes())?;
    Ok(())
}
