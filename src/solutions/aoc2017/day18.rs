/// Run the program until it blocks, and get the last one.
pub fn one(input: &str) -> crate::Result<String> {
    let mut p = Program::from_input(input, 0);
    let output = p.run_until_blocked(&[]);
    match output.last() {
        Some(v) => Ok(v.to_string()),
        None => Err("no result".into()),
    }
}

/// Run two communicating programs until both halt, and find how many times the second
/// program has sent a value.
pub fn two(input: &str) -> crate::Result<String> {
    let mut p0 = Program::from_input(input, 0);
    let mut p1 = Program::from_input(input, 1);
    let mut pass = vec![];

    loop {
        pass = p0.run_until_blocked(&pass);
        pass = p1.run_until_blocked(&pass);
        if pass.is_empty() {
            break;
        }
    }

    Ok(p1.sends.to_string())
}

/// A runnable program, as per the puzzle description.
struct Program<'a> {
    registers: [i64; 26],
    code: Vec<Op<'a>>,
    pointer: i64,
    sends: usize,
}

impl Program<'_> {
    /// Creates a new runnable program from a code listing.
    fn from_input(input: &str, id: usize) -> Program {
        fn parse_arg(s: &str) -> Option<Arg> {
            Some(if s.chars().next()?.is_alphabetic() {
                Arg::Reg(s.as_bytes()[0] - b'a')
            } else {
                Arg::Const(s.parse().ok()?)
            })
        }

        fn parse_ops(input: &str) -> impl Iterator<Item = Op> + '_ {
            input.lines().filter_map(|line| {
                let tokens: Vec<_> = line.split_whitespace().collect();
                Some(match &tokens[..] {
                    [op, arg] => Op {
                        op,
                        arg1: parse_arg(arg)?,
                        arg2: None,
                    },
                    [op, arg1, arg2] => Op {
                        op,
                        arg1: parse_arg(arg1)?,
                        arg2: Some(parse_arg(arg2)?),
                    },
                    _ => None?,
                })
            })
        }

        let mut p = Program {
            registers: [0; 26],
            code: parse_ops(input).collect(),
            pointer: 0,
            sends: 0,
        };
        p.registers[(b'p' - b'a') as usize] = id as i64;
        p
    }

    /// Runs the program until it blocks on a receive instruction; and returns all sended
    /// values produced in that time, in order.
    #[rustfmt::skip]
    fn run_until_blocked(&mut self, input: &[i64]) -> Vec<i64> {
        if !(0..self.code.len() as i64).contains(&self.pointer) {
            return vec![];
        }

        let mut output = vec![];
        let mut read = 0;

        while let Some(op) = self.code.get(self.pointer as usize) {
            match op {
                Op { op: "snd", arg1, arg2: None } => {
                    output.push(self.read(arg1));
                    self.sends += 1;
                }
                Op { op: "set", arg1: Arg::Reg(r), arg2: Some(arg2) } => {
                    self.registers[*r as usize] = self.read(arg2);
                }
                Op { op: "add", arg1: Arg::Reg(r), arg2: Some(arg2) } => {
                    self.registers[*r as usize] += self.read(arg2);
                }
                Op { op: "mul", arg1: Arg::Reg(r), arg2: Some(arg2) } => {
                    self.registers[*r as usize] *= self.read(arg2);
                }
                Op { op: "mod", arg1: Arg::Reg(r), arg2: Some(arg2) } => {
                    self.registers[*r as usize] %= self.read(arg2);
                }
                Op { op: "rcv", arg1: Arg::Reg(r), arg2: None } => {
                    if read >= input.len() {
                        return output;
                    }

                    self.registers[*r as usize] = input[read];
                    read += 1;
                }
                Op { op: "jgz", arg1, arg2: Some(arg2) } => {
                    if self.read(arg1) > 0 {
                        self.pointer += self.read(arg2);
                        continue;
                    }
                }
                _ => {},
            }
            self.pointer += 1;
        }

        output
    }

    /// Resolves an argument into a concrete value.
    fn read(&self, arg: &Arg) -> i64 {
        match arg {
            Arg::Reg(r) => self.registers[*r as usize],
            Arg::Const(c) => *c,
        }
    }
}

/// A single operand; can either be a constant or a register reference.
enum Arg {
    Reg(u8),
    Const(i64),
}

/// An instruction; made of an opcode and either one or two operands.
struct Op<'a> {
    op: &'a str,
    arg1: Arg,
    arg2: Option<Arg>,
}
