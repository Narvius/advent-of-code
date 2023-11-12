use crate::common::intcode::v2::*;

/// Run the program in test mode.
pub fn one(input: &str) -> crate::Result<i64> {
    let mut program = Program::with_capacity(input, 2000)?;
    program.run_with([1])?;
    program.output.pop_front().ok_or("no result".into())
}

/// Run the program in sensor boost mode.
pub fn two(input: &str) -> crate::Result<i64> {
    let mut program = Program::with_capacity(input, 2000)?;
    program.run_with([2])?;
    program.output.pop_front().ok_or("no result".into())
}
