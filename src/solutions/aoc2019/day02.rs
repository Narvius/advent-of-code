/// Run the intcode program with fixed inputs.
pub fn one(input: &str) -> crate::Result<i32> {
    run(&mut parse(input)?, 12, 2)
}

/// Find the inputs to the intcode program that produce a specific output.
pub fn two(input: &str) -> crate::Result<i32> {
    let program = parse(input)?;

    for noun in 0..100 {
        for verb in 0..100 {
            if let Ok(19690720) = run(&mut program.clone(), noun, verb) {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err("no successful run".into())
}

/// Runs an intcode program, and returns the value in the 0th cell.
fn run(program: &mut [i32], noun: i32, verb: i32) -> crate::Result<i32> {
    let mut pointer = 0;
    program[1] = noun;
    program[2] = verb;

    while let Some(op) = program.get(pointer).copied() {
        match op {
            1 => apply(program, pointer, |a, b| a + b)?,
            2 => apply(program, pointer, |a, b| a * b)?,
            99 => break,
            _ => return Err(format!("unknown opcode {op}").into()),
        }

        pointer += 4;
    }

    Ok(program[0])
}

/// Given an intcode program and the index to an opcode in it, applies a binary operation
/// and stores the result back in the program, as per the rules of the intcode computer.
fn apply(program: &mut [i32], pointer: usize, f: fn(i32, i32) -> i32) -> crate::Result<()> {
    fn resolve(program: &mut [i32], index: usize) -> crate::Result<&mut i32> {
        let index = *program.get(index).ok_or("not enough operands")?;
        let index =
            usize::try_from(index).map_err(|_| format!("invalid indirect pointer {index}"))?;
        Ok(program
            .get_mut(index)
            .ok_or(format!("invalid indirect pointer {index}"))?)
    }

    let a = *resolve(program, pointer + 1)?;
    let b = *resolve(program, pointer + 2)?;
    let target = resolve(program, pointer + 3)?;

    *target = f(a, b);
    Ok(())
}

/// Parses the puzzle input.
fn parse(input: &str) -> Result<Vec<i32>, impl std::error::Error> {
    input.trim().split(',').map(|n| n.parse::<i32>()).collect()
}
