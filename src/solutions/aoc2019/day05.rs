use crate::common::intcode::v1::*;

/// Run the provided Intcode program with ID 1. This will test I/O instructions (3-4).
pub fn one(input: &str) -> crate::Result<Int> {
    run_with_input(input, 1)
}

/// Run the provided Intcode program with ID 5. This will test logic instructions (5-8).
pub fn two(input: &str) -> crate::Result<Int> {
    run_with_input(input, 5)
}

/// Shared code for both parts.
fn run_with_input(code: &str, input: i32) -> crate::Result<Int> {
    let mut program = Program::new(code, [input])?;
    while let Ok(Outcome::Ok) = program.step() {}
    program.output.pop_back().ok_or_else(|| "no output".into())
}
