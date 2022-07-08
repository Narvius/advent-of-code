/// Count the number of jump instructions executed before pointing outside the list, using
/// simple instruction change rules.
pub fn one(input: &str) -> crate::Result<i32> {
    run_jumps(input, |offset| *offset += 1)
}

/// Count the number of jump instructions executed before pointing outside the list, using
/// slightly more complex instruction change rules.
pub fn two(input: &str) -> crate::Result<i32> {
    run_jumps(input, |offset| *offset += if *offset >= 3 { -1 } else { 1 })
}

/// Shared logic for both parts. `morph` mutates the pointed-at instruction after executing
/// it.
fn run_jumps(input: &str, morph: fn(&mut i32)) -> crate::Result<i32> {
    let mut offsets = parse(input)?;
    let mut pointer = 0;
    let mut steps = 0;

    while let Some(offset) = pointer
        .try_into()
        .ok()
        .and_then(|i: usize| offsets.get_mut(i))
    {
        steps += 1;
        pointer += *offset;
        morph(offset);
    }

    Ok(steps)
}

/// Parses the puzzle input into a list of numbers.
fn parse(input: &str) -> Result<Vec<i32>, String> {
    input
        .lines()
        .map(|line| line.parse().map_err(|_| "failed to parse num".into()))
        .collect()
}
