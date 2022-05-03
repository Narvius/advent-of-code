/// Find the lowest value for register `a` that will result in an infinite stream of "1, 0"
/// in the output.
pub fn one(input: &str) -> Result<String, String> {
    let program = parse(input);
    for i in 0.. {
        let generator = SignalGenerator::new(&program, i);
        if generator.take(10).eq([0, 1, 0, 1, 0, 1, 0, 1, 0, 1].into_iter()) {
            return Ok(i.to_string());
        }
    }

    Err(format!("unreachable"))
}

/// Freebie!
pub fn two(_input: &str) -> Result<String, String> {
    Ok(format!("done!"))
}

/// An iterator that returns the [`Out`](Op::Out) results from running a program.
struct SignalGenerator<'a> {
    program: &'a [Op],
    reg: [i32; 4],
    pointer: i32,
}

impl<'a> SignalGenerator<'a> {
    /// Builds a new [`SignalGenerator`], setting the first register to the value of `a`.
    fn new(program: &'a [Op], a: i32) -> Self {
        Self {
            program,
            reg: [a, 0, 0, 0],
            pointer: 0,
        }
    }

    /// Resolves an [`Arg`] to a value.
    fn get(&self, arg: Arg) -> i32 {
        match arg {
            Arg::Const(c) => c,
            Arg::Reg(i) => self.reg[i],
        }
    }
}

impl<'a> Iterator for SignalGenerator<'a> {
    type Item = i32;

    /// Generates the next item by running the program until an [`Out`](Op::Out) instruction is
    /// hit, and returning its operand.
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&op) = self.program.get(self.pointer as usize) {
            match op {
                Op::Cpy(src, tgt) => self.reg[tgt] = self.get(src),
                Op::Jnz(check, offset) => {
                    if self.get(check) != 0 {
                        self.pointer += offset;
                        continue;
                    }
                }
                Op::Inc(tgt) => self.reg[tgt] += 1,
                Op::Dec(tgt) => self.reg[tgt] -= 1,
                Op::Out(src) => {
                    self.pointer += 1;
                    return Some(self.get(src));
                }
            }
    
            self.pointer += 1;
        }
    
        None
    }
}



/// An operation.
#[derive(Copy, Clone, Debug)]
enum Op {
    Cpy(Arg, usize),
    Jnz(Arg, i32),
    Inc(usize),
    Dec(usize),
    Out(Arg),
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
                (2, "inc") => Op::Inc((tokens[1].as_bytes()[0] - b'a') as usize),
                (2, "dec") => Op::Dec((tokens[1].as_bytes()[0] - b'a') as usize),
                (3, "cpy") => Op::Cpy(arg(tokens[1])?, (tokens[2].as_bytes()[0] - b'a') as usize),
                (3, "jnz") => Op::Jnz(arg(tokens[1])?, tokens[2].parse().ok()?),
                (2, "out") => Op::Out(arg(tokens[1])?),
                _ => None?,
            })
        })
        .collect()
}
