use std::collections::HashMap;
use std::path::PathBuf;
use crate::problems::problem00::Problem00;
use crate::problems::problem01::Problem01;
use crate::problems::problem02::Problem02;
use crate::problems::problem03::Problem03;
use crate::problems::problem04::Problem04;
use crate::problems::problem05::Problem05;
use crate::problems::problem06::{Problem06, Problem06Part2};
use crate::problems::problem07::Problem07;
use crate::problems::problem08::Problem08;
use crate::problems::problem09::Problem09;
use crate::problems::problem10::Problem10;
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
        (4, false),
        (5, false),
        (6, false),
        (7, false),
        (8, false),
        (9, false),
        (10, true),
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

    if run_solver[&5] {
        solve(&Problem05::new(),
              PathBuf::from("data/problem05/sample.in"),PathBuf::from("data/problem05/sample.out"))?;
        solve(&Problem05::new(),
              PathBuf::from("data/problem05/01.in"),PathBuf::from("data/problem05/01.out"))?;
        solve(&Problem05::new_part2(),
              PathBuf::from("data/problem05/sample.in"),PathBuf::from("data/problem05/sample-2.out"))?;
        solve(&Problem05::new_part2(),
              PathBuf::from("data/problem05/01.in"),PathBuf::from("data/problem05/01-2.out"))?;
    }

    if run_solver[&6] {
        solve(&Problem06::new(),
              PathBuf::from("data/problem06/sample.in"),PathBuf::from("data/problem06/sample.out"))?;
        solve(&Problem06::new(),
              PathBuf::from("data/problem06/01.in"),PathBuf::from("data/problem06/01.out"))?;
        solve(&Problem06Part2::new(),
              PathBuf::from("data/problem06/sample.in"),PathBuf::from("data/problem06/sample-2.out"))?;
        solve(&Problem06Part2::new(),
              PathBuf::from("data/problem06/01.in"),PathBuf::from("data/problem06/01-2.out"))?;
    }

    if run_solver[&7] {
        solve(&Problem07::new(),
              PathBuf::from("data/problem07/sample.in"), PathBuf::from("data/problem07/sample.out"))?;
        solve(&Problem07::new(),
              PathBuf::from("data/problem07/01.in"), PathBuf::from("data/problem07/01.out"))?;
        solve(&Problem07::new_part2(),
              PathBuf::from("data/problem07/sample.in"), PathBuf::from("data/problem07/sample-2.out"))?;
        solve(&Problem07::new_part2(),
              PathBuf::from("data/problem07/01.in"), PathBuf::from("data/problem07/01-2.out"))?;
    }

    if run_solver[&8] {
        solve(&Problem08::new(10),
              PathBuf::from("data/problem08/sample.in"), PathBuf::from("data/problem08/sample.out"))?;
        solve(&Problem08::new(1000),
              PathBuf::from("data/problem08/01.in"), PathBuf::from("data/problem08/01.out"))?;
        solve(&Problem08::new_part2(),
              PathBuf::from("data/problem08/sample.in"), PathBuf::from("data/problem08/sample-2.out"))?;
        solve(&Problem08::new_part2(),
              PathBuf::from("data/problem08/01.in"), PathBuf::from("data/problem08/01-2.out"))?;
    }

    if run_solver[&9] {
        solve(&Problem09::new(),
              PathBuf::from("data/problem09/sample.in"), PathBuf::from("data/problem09/sample.out"))?;
        solve(&Problem09::new(),
              PathBuf::from("data/problem09/01.in"), PathBuf::from("data/problem09/01.out"))?;
        solve(&Problem09::new_part2(),
              PathBuf::from("data/problem09/sample.in"), PathBuf::from("data/problem09/sample-2.out"))?;
        solve(&Problem09::new_part2(),
              PathBuf::from("data/problem09/01.in"), PathBuf::from("data/problem09/01-2.out"))?;
    }

    if run_solver[&10] {
        solve(&Problem10::new(),
              PathBuf::from("data/problem10/sample.in"), PathBuf::from("data/problem10/sample.out"))?;
        solve(&Problem10::new(),
              PathBuf::from("data/problem10/01.in"), PathBuf::from("data/problem10/01.out"))?;
    }

    Ok(())
}
