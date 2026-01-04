use std::path::PathBuf;
use crate::problems::problem00::Problem00;
use crate::problems::problem01::Problem01;
use crate::problems::problem02::Problem02;
use crate::problems::common::solve;

mod problems;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    solve(&Problem00 {},
          PathBuf::from("data/problem00/00.in"),PathBuf::from("data/problem00/00.out"))?;

    solve(&Problem01::new_part1(),
          PathBuf::from("data/problem01/sample.in"),PathBuf::from("data/problem01/sample.out"))?;
    solve(&Problem01::new_part1(),
          PathBuf::from("data/problem01/01.in"),PathBuf::from("data/problem01/01.out"))?;
    solve(&Problem01::new_part2(),
          PathBuf::from("data/problem01/sample.in"),PathBuf::from("data/problem01/sample-2.out"))?;
    solve(&Problem01::new_part2(),
          PathBuf::from("data/problem01/01.in"),PathBuf::from("data/problem01/01-2.out"))?;

    solve(&Problem02::new(),
          PathBuf::from("data/problem02/sample.in"),PathBuf::from("data/problem02/sample.out"))?;
    solve(&Problem02::new(),
          PathBuf::from("data/problem02/01.in"),PathBuf::from("data/problem02/01.out"))?;
    solve(&Problem02::new_part2(),
          PathBuf::from("data/problem02/sample.in"),PathBuf::from("data/problem02/sample-2.out"))?;
    solve(&Problem02::new_part2(),
          PathBuf::from("data/problem02/01.in"),PathBuf::from("data/problem02/01-2.out"))?;

    Ok(())
}
