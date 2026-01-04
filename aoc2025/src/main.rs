use std::path::PathBuf;
use crate::problems::problem00::Problem00;
use crate::problems::common::solve;

mod problems;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    solve(&Problem00 {},
          PathBuf::from("data/problem00/00.in"),PathBuf::from("data/problem00/00.out"))?;
    Ok(())
}
