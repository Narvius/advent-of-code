/// Run the program as given until it begins to loop; find the accumulator at that point.
pub fn one(input: &str) -> crate::Result<i32> {
    match run(parse(input)) {
        Ok(_) => Err("unexpectedly terminated with success".into()),
        Err(result) => Ok(result),
    }
}

/// Fix the code by swapping one jmp to a nop (or vice versa); find the accumulator after
/// running that fixed program.
pub fn two(input: &str) -> crate::Result<i32> {
    let code = parse(input);
    Ok((0..code.len())
        .filter_map(|i| with_swapped(&code, i))
        .map(run)
        .find(|result| result.is_ok())
        .ok_or("no result")?
        .unwrap())
}

/// Runs the provided code. If it would loop forever, returns `Err(accumulator after one loop)`,
/// if terminates, returns `Ok(accumulator after terminating)`.
fn run(mut code: Vec<Op>) -> Result<i32, i32> {
    let mut pointer = 0;
    let mut acc = 0;

    while let Some(op) = code.get_mut(pointer as usize) {
        if op.executed {
            return Err(acc);
        }
        op.executed = true;

        match op.code {
            "acc" => acc += op.val,
            "jmp" => pointer += op.val - 1,
            _ => {}
        }
        pointer += 1;
    }

    Ok(acc)
}

/// Returns a copy of the code where the `index`th operation is swapped (jmp <-> nop). If
/// that operation is not swappable, returns `None`.
fn with_swapped<'a>(code: &[Op<'a>], index: usize) -> Option<Vec<Op<'a>>> {
    (code[index].code != "acc").then(|| {
        let mut result: Vec<_> = code.to_vec();
        result[index].swap();
        result
    })
}

/// Parses the puzzle input into a list of operations.
fn parse(input: &str) -> Vec<Op<'_>> {
    input
        .lines()
        .filter_map(|line| {
            let (code, val) = line.split_once(' ')?;
            Some(Op {
                code,
                val: val.parse().ok()?,
                executed: false,
            })
        })
        .collect()
}

/// A single code operation.
#[derive(Clone)]
struct Op<'a> {
    code: &'a str,
    val: i32,
    executed: bool,
}

impl Op<'_> {
    /// Swaps this operation. Has no effect on `acc` opcodes.
    fn swap(&mut self) {
        match self.code {
            "jmp" => self.code = "nop",
            "nop" => self.code = "jmp",
            _ => {}
        }
    }
}
