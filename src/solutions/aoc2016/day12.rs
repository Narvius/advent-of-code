/// Run the program, get the final value of `a`.
pub fn one(input: &str) -> crate::Result<i32> {
    run_program(input, 0).ok_or_else(|| "failed to run program".into())
}

/// Run the program with `c` initialized to 1, get the final value of `a`.
pub fn two(input: &str) -> crate::Result<i32> {
    run_program(input, 1).ok_or_else(|| "failed to run program".into())
}

/// Runs a program given by the puzzle input, with the `c` register initialized to the given value.
fn run_program(program: &str, c: i32) -> Option<i32> {
    let program = parse(program);
    let mut reg = [0, 0, c, 0];
    let mut pointer = 0;

    fn get(reg: &[i32], arg: Arg) -> i32 {
        match arg {
            Arg::Const(c) => c,
            Arg::Reg(i) => reg[i],
        }
    }

    while let Some(op) = program.get(pointer as usize) {
        match *op {
            Op::Cpy(src, tgt) => reg[tgt] = get(&reg, src),
            Op::Jnz(check, offset) => {
                if get(&reg, check) != 0 {
                    pointer += offset;
                    continue;
                }
            }
            Op::Inc(tgt) => reg[tgt] += 1,
            Op::Dec(tgt) => reg[tgt] -= 1,
        }

        pointer += 1;
    }

    Some(reg[0])
}

/// An operation.
#[derive(Copy, Clone, Debug)]
enum Op {
    Cpy(Arg, usize),
    Jnz(Arg, i32),
    Inc(usize),
    Dec(usize),
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
            Some(match (tokens.len(), *tokens.first()?) {
                (2, "inc") => Op::Inc((tokens[1].as_bytes()[0] - b'a') as usize),
                (2, "dec") => Op::Dec((tokens[1].as_bytes()[0] - b'a') as usize),
                (3, "cpy") => Op::Cpy(arg(tokens[1])?, (tokens[2].as_bytes()[0] - b'a') as usize),
                (3, "jnz") => Op::Jnz(arg(tokens[1])?, tokens[2].parse().ok()?),
                _ => None?,
            })
        })
        .collect()
}
