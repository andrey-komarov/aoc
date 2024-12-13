use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

mod problems;
use crate::problems::common::Solvable;
use crate::problems::*;

fn solver<P: Solvable>(p: P) -> impl Fn(PathBuf, PathBuf) -> Result<()> {
    move |input_fname, output_fname| {
        let input_file = BufReader::new(File::open(&input_fname)?);
        let output_file = File::create(&output_fname)?;
        p.solve(BufReader::new(input_file), output_file)
    }
}

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let problems: Vec<(_, _, _, Box<dyn Fn(PathBuf, PathBuf) -> _>)> = vec![
        ("problem00", "00.in", "00.out", Box::new(solver(problem00::Problem{}))),

        ("problem01", "sample.in", "sample.out", Box::new(solver(problem01::Problem{}))),
        ("problem01", "01.in", "01.out", Box::new(solver(problem01::Problem{}))),
        ("problem01", "sample02.in", "sample02.out", Box::new(solver(problem01::PartTwo{}))),
        ("problem01", "02.in", "02.out", Box::new(solver(problem01::PartTwo{}))),

        ("problem02", "sample.in", "sample.out", Box::new(solver(problem02::PartOne{}))),
        ("problem02", "01.in", "01.out", Box::new(solver(problem02::PartOne{}))),
        ("problem02", "sample.in", "sample2.out", Box::new(solver(problem02::PartTwo{}))),
        ("problem02", "01.in", "02.out", Box::new(solver(problem02::PartTwo{}))),

        ("problem03", "sample.in", "sample-part1.out", Box::new(solver(problem03::PartOne{}))),
        ("problem03", "01.in", "01-part1.out", Box::new(solver(problem03::PartOne{}))),
        ("problem03", "sample-part2.in", "sample-part2.out", Box::new(solver(problem03::PartTwo{}))),
        ("problem03", "01.in", "01-part2.out", Box::new(solver(problem03::PartTwo{}))),

        ("problem04", "sample.in", "sample-part1.out", Box::new(solver(problem04::PartOne{}))),
        ("problem04", "01.in", "01-part1.out", Box::new(solver(problem04::PartOne{}))),
        ("problem04", "sample.in", "sample-part2.out", Box::new(solver(problem04::PartTwo{}))),
        ("problem04", "01.in", "01-part2.out", Box::new(solver(problem04::PartTwo{}))),

        ("problem05", "sample.in", "sample-part1.out", Box::new(solver(problem05::PartOne{}))),
        ("problem05", "01.in", "01-part1.out", Box::new(solver(problem05::PartOne{}))),
        ("problem05", "sample.in", "sample-part2.out", Box::new(solver(problem05::PartTwo{}))),
        ("problem05", "01.in", "01-part2.out", Box::new(solver(problem05::PartTwo{}))),

        ("problem06", "sample.in", "sample-part1.out", Box::new(solver(problem06::PartOne{}))),
        ("problem06", "01.in", "01-part1.out", Box::new(solver(problem06::PartOne{}))),
        ("problem06", "sample.in", "sample-part2.out", Box::new(solver(problem06::PartTwo{}))),
        // ("problem06", "01.in", "01-part2.out", Box::new(solver(problem06::PartTwo{}))),

        ("problem07", "sample.in", "sample-part1.out", Box::new(solver(problem07::PartOne{}))),
        ("problem07", "01.in", "01-part1.out", Box::new(solver(problem07::PartOne{}))),
        ("problem07", "sample.in", "sample-part2.out", Box::new(solver(problem07::PartTwo{}))),
        ("problem07", "01.in", "01-part2.out", Box::new(solver(problem07::PartTwo{}))),

        ("problem08", "sample.in", "sample-part1.out", Box::new(solver(problem08::PartOne{}))),
        ("problem08", "01.in", "01-part1.out", Box::new(solver(problem08::PartOne{}))),
        ("problem08", "sample.in", "sample-part2.out", Box::new(solver(problem08::PartTwo{}))),
        ("problem08", "01.in", "01-part2.out", Box::new(solver(problem08::PartTwo{}))),

        ("problem09", "sample.in", "sample-part1.out", Box::new(solver(problem09::PartOne{}))),
        ("problem09", "01.in", "01-part1.out", Box::new(solver(problem09::PartOne{}))),
        ("problem09", "sample.in", "sample-part2.out", Box::new(solver(problem09::PartTwo{}))),
        // ("problem09", "01.in", "01-part2.out", Box::new(solver(problem09::PartTwo{}))),

        ("problem10", "sample.in", "sample-part1.out", Box::new(solver(problem10::PartOne{}))),
        ("problem10", "01.in", "01-part1.out", Box::new(solver(problem10::PartOne{}))),
        ("problem10", "sample.in", "sample-part2.out", Box::new(solver(problem10::PartTwo{}))),
        ("problem10", "01.in", "01-part2.out", Box::new(solver(problem10::PartTwo{}))),

        ("problem11", "sample.in", "sample-part0.out", Box::new(solver(problem11::PartOne::new(6)))),
        ("problem11", "sample.in", "sample-part1.out", Box::new(solver(problem11::PartOne::new(25)))),
        ("problem11", "01.in", "01-part1.out", Box::new(solver(problem11::PartOne::new(25)))),
        ("problem11", "01.in", "01-part2.out", Box::new(solver(problem11::PartOne::new(75)))),

        ("problem12", "sample.in", "sample-part1.out", Box::new(solver(problem12::PartOne{}))),
        ("problem12", "01.in", "01-part1.out", Box::new(solver(problem12::PartOne{}))),
        ("problem12", "sample.in", "sample-part2.out", Box::new(solver(problem12::PartTwo{}))),
        ("problem12", "sample02.in", "sample02-part2.out", Box::new(solver(problem12::PartTwo{}))),
        ("problem12", "01.in", "01-part2.out", Box::new(solver(problem12::PartTwo{}))),

        ("problem13", "sample.in", "sample-part1.out", Box::new(solver(problem13::PartOne{}))),
        ("problem13", "01.in", "01-part1.out", Box::new(solver(problem13::PartOne{}))),
        ("problem13", "sample.in", "sample-part2.out", Box::new(solver(problem13::PartTwo{}))),
        ("problem13", "01.in", "01-part2.out", Box::new(solver(problem13::PartTwo{}))),
    ];
    for (dir, input, output, solver) in problems {
        println!("Solving problem {}/{}:", dir, input);
        let input_fname = format!("data/{}/{}", dir, input);
        let output_fname = format!("data/{}/{}", dir, output);
        solver(PathBuf::from(input_fname), PathBuf::from(output_fname))?;
    }
    Ok(())
}
