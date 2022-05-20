/// Run the provided program with an input of 7.
pub fn one(input: &str) -> Result<String, String> {
    run_program_v2(input, [7, 0, 0, 0]).ok_or_else(|| "failed to run program".into())
}

/// Run the provided program with an input of 12.
pub fn two(input: &str) -> Result<String, String> {
    run_program_v2(input, [12, 0, 0, 0]).ok_or_else(|| "failed to run program".into())
}

/// Runs a program given by the puzzle input, with the `c` register initialized to the given value.
/// Expands upon run program from [`day12`](crate::solutions::aoc2016::day12) adding two
/// major things:
/// - the [`Tgl`](Op::Tgl) (toggle) operation (see [`toggled`] for more information);
/// - an optimization that replaces increment/decrement loops that perform multiplications with
///   a single actual multiplication (see [`optimize_loop`] for more information).
fn run_program_v2(program: &str, mut reg: [i32; 4]) -> Option<String> {
    let mut program = parse(program);
    let mut pointer = 0;

    while let Some(&op) = program.get(pointer as usize) {
        if optimize_loop(&mut program, &mut pointer, &mut reg) {
            continue;
        }

        match op {
            Op::Cpy(src, tgt) => {
                if let Arg::Reg(tgt) = tgt {
                    reg[tgt] = get(&reg, src)
                }
            }
            Op::Jnz(check, offset) => {
                if get(&reg, check) != 0 {
                    pointer += get(&reg, offset);
                    continue;
                }
            }
            Op::Inc(tgt) => {
                if let Arg::Reg(tgt) = tgt {
                    reg[tgt] += 1;
                }
            }
            Op::Dec(tgt) => {
                if let Arg::Reg(tgt) = tgt {
                    reg[tgt] -= 1;
                }
            }
            Op::Tgl(offset) => {
                let index = (pointer + get(&reg, offset)) as usize;
                if let Some(op) = program.get(index) {
                    program[index] = toggled(*op);
                }
            }
        }

        pointer += 1;
    }

    Some(reg[0].to_string())
}

/// Resolves an [`Arg`] into a numeric value.
fn get(reg: &[i32], arg: Arg) -> i32 {
    match arg {
        Arg::Const(c) => c,
        Arg::Reg(i) => reg[i],
    }
}

/// Flips an [`Op`] according to the rules of [`Tgl`](Op::Tgl).
fn toggled(op: Op) -> Op {
    match op {
        Op::Inc(a) => Op::Dec(a),
        Op::Dec(a) => Op::Inc(a),
        Op::Tgl(a) => Op::Inc(a),
        Op::Jnz(a, b) => Op::Cpy(a, b),
        Op::Cpy(a, b) => Op::Jnz(a, b),
    }
}

/// Performs the optimization hinted at in the problem statement. It finds a loop that just
/// performs multiplication via increments and decrements, and executes them as one multiply.
/// Returns whether a loop was optimized away.
fn optimize_loop(program: &mut [Op], pointer: &mut i32, reg: &mut [i32]) -> bool {
    use Op::*;
    let index = *pointer as usize;
    if let [Cpy(src1, _), Inc(Arg::Reg(tgt)), Dec(_), Jnz(_, Arg::Const(-2)), Dec(src2), Jnz(_, Arg::Const(-5))] =
        program[index..index + 6]
    {
        reg[tgt] += get(reg, src1) * get(reg, src2);
        // Accounting for a side effect of the multiplication: This specific source register
        // is zeroed due to how values get moved by increment/decrement pairs.
        if let Arg::Reg(src2) = src2 {
            reg[src2] = 0;
        }
        *pointer += 6;
        return true;
    }
    false
}

/// An operation.
#[derive(Copy, Clone, Debug)]
enum Op {
    Cpy(Arg, Arg),
    Jnz(Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Tgl(Arg),
}

/// An argument that can either be a constant or a register.
#[derive(Copy, Clone, Debug)]
enum Arg {
    Const(i32),
    Reg(usize),
}

/// Parses the puzzle input into a list of operations.
fn parse(input: &str) -> Vec<Op> {
    fn arg(token: &str) -> Option<Arg> {
        Some(if token.as_bytes()[0].is_ascii_alphabetic() {
            Arg::Reg((token.as_bytes()[0] - b'a') as usize)
        } else {
            Arg::Const(token.parse().ok()?)
        })
    }

    input
        .lines()
        .filter_map(|line| {
            let tokens: Vec<_> = line.split(' ').collect();
            Some(match (tokens.len(), *tokens.get(0)?) {
                (2, "inc") => Op::Inc(arg(tokens[1])?),
                (2, "dec") => Op::Dec(arg(tokens[1])?),
                (2, "tgl") => Op::Tgl(arg(tokens[1])?),
                (3, "cpy") => Op::Cpy(arg(tokens[1])?, arg(tokens[2])?),
                (3, "jnz") => Op::Jnz(arg(tokens[1])?, arg(tokens[2])?),
                _ => None?,
            })
        })
        .collect()
}
