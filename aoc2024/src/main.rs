use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use anyhow::Result;

mod problems;
use problems::problem00;
use crate::problems::common::Solvable;
use crate::problems::problem01;
use crate::problems::problem02;

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
    ];
    for (dir, input, output, solver) in problems {
        println!("Solving problem {}/{}:", dir, input);
        let input_fname = format!("data/{}/{}", dir, input);
        let output_fname = format!("data/{}/{}", dir, output);
        solver(PathBuf::from(input_fname), PathBuf::from(output_fname))?;
    }
    Ok(())
}
