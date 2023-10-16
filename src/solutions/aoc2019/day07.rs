use crate::common::intcode::v1::*;

/// Find the best thruster configuration, and return the produced output.
pub fn one(input: &str) -> crate::Result<Int> {
    work(input, 0, |programs| {
        let mut input = 0;
        for mut program in programs {
            program.input.push_back(input);
            while let Outcome::Ok = program.step()? {}
            input = program.output.pop_front().ok_or("no output")?;
        }
        Ok(input)
    })
}

/// Find the best thruster configuration with a feedback loop, and return the produced output.
pub fn two(input: &str) -> crate::Result<Int> {
    work(input, 5, |mut programs| {
        let mut input = 0;
        let mut active = 0;
        loop {
            programs[active].input.push_back(input);
            while let Outcome::Ok = programs[active].step()? {}
            match programs[active].output.pop_front() {
                Some(v) => input = v,
                None => break,
            }
            active = (active + 1) % programs.len();
        }
        Ok(input)
    })
}

/// Shared code for both parts.
///
/// For every possible permutation of phase settings, produced a list of [`Program`]s
/// configured with that permutation, then calls `body` with that list; then keeps track of
/// the highest value produced by any of those calls, ultimately returning it.
fn work(
    input: &str,
    key_offset: i32,
    body: fn(Vec<Program>) -> crate::Result<Int>,
) -> crate::Result<Int> {
    let prototype = Program::new(input, [])?;
    let mut maximum = 0;

    for sequence in crate::common::permutations(5) {
        let mut programs = vec![prototype.clone(); 5];
        for (program, key) in programs.iter_mut().zip(sequence) {
            program.input.push_back(key as i32 + key_offset);
        }
        maximum = maximum.max(body(programs)?);
    }

    Ok(maximum)
}
