use std::fs::File;
use std::io::BufReader;

mod problems;
use problems::problem00;
use crate::problems::common::Solvable;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let problems = vec![
        ("problem00", "00.in", "00.out", problem00::Problem{}),
    ];
    for (dir, input, output, solver) in problems {
        println!("Solving problem {}/{}:", dir, input);
        let input_fname = format!("data/{}/{}", dir, input);
        let output_fname = format!("data/{}/{}", dir, output);
        let input_file = File::open(&input_fname)?;
        let output_file = File::create(&output_fname)?;
        solver.solve(BufReader::new(input_file), output_file)?;
    }
    Ok(())
}
