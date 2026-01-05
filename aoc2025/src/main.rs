use std::collections::HashMap;
use std::path::PathBuf;
use crate::problems::problem00::Problem00;
use crate::problems::problem01::Problem01;
use crate::problems::problem02::Problem02;
use crate::problems::problem03::Problem03;
use crate::problems::problem04::Problem04;
use crate::problems::common::solve;

mod problems;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    solve(&Problem00 {},
          PathBuf::from("data/problem00/00.in"),PathBuf::from("data/problem00/00.out"))?;

    let run_solver = HashMap::from([
        (0, false),
        (1, false),
        (2, false),
        (3, false),
        (4, true),
    ]);

    if run_solver[&1] {
        solve(&Problem01::new_part1(),
              PathBuf::from("data/problem01/sample.in"),PathBuf::from("data/problem01/sample.out"))?;
        solve(&Problem01::new_part1(),
              PathBuf::from("data/problem01/01.in"),PathBuf::from("data/problem01/01.out"))?;
        solve(&Problem01::new_part2(),
              PathBuf::from("data/problem01/sample.in"),PathBuf::from("data/problem01/sample-2.out"))?;
        solve(&Problem01::new_part2(),
              PathBuf::from("data/problem01/01.in"),PathBuf::from("data/problem01/01-2.out"))?;
    }

    if run_solver[&2] {
        solve(&Problem02::new(),
              PathBuf::from("data/problem02/sample.in"),PathBuf::from("data/problem02/sample.out"))?;
        solve(&Problem02::new(),
              PathBuf::from("data/problem02/01.in"),PathBuf::from("data/problem02/01.out"))?;
        solve(&Problem02::new_part2(),
              PathBuf::from("data/problem02/sample.in"),PathBuf::from("data/problem02/sample-2.out"))?;
        solve(&Problem02::new_part2(),
              PathBuf::from("data/problem02/01.in"),PathBuf::from("data/problem02/01-2.out"))?;
    }

    if run_solver[&3] {
        solve(&Problem03::new(),
              PathBuf::from("data/problem03/sample.in"),PathBuf::from("data/problem03/sample.out"))?;
        solve(&Problem03::new(),
              PathBuf::from("data/problem03/01.in"),PathBuf::from("data/problem03/01.out"))?;
        solve(&Problem03::new_part2(),
              PathBuf::from("data/problem03/sample.in"),PathBuf::from("data/problem03/sample-2.out"))?;
        solve(&Problem03::new_part2(),
              PathBuf::from("data/problem03/01.in"),PathBuf::from("data/problem03/01-2.out"))?;
    }

    if run_solver[&4] {
        solve(&Problem04::new(),
              PathBuf::from("data/problem04/sample.in"),PathBuf::from("data/problem04/sample.out"))?;
        solve(&Problem04::new(),
              PathBuf::from("data/problem04/01.in"),PathBuf::from("data/problem04/01.out"))?;
        solve(&Problem04::new_part2(),
              PathBuf::from("data/problem04/sample.in"),PathBuf::from("data/problem04/sample-2.out"))?;
        solve(&Problem04::new_part2(),
              PathBuf::from("data/problem04/01.in"),PathBuf::from("data/problem04/01-2.out"))?;
    }

    Ok(())
}
