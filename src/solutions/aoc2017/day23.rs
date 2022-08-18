pub fn one(input: &str) -> crate::Result<usize> {
    let mut p = Program::from_input(input);
    p.run_until_blocked();
    Ok(p.mul_uses)
}

pub fn two(input: &str) -> crate::Result<usize> {
    /*
        Optimizing the program.

        For a = 0: [b = c = 81]
        For a > 0: [b = 108100, c = 125100]

        That's the only difference between the two executions. The code afterwards
        runs the exact same either way, just wasting hundreds of thousands of cycles
        most likely.

        For what it's worth, [h] for part 1 has a value of [1].

        ** Equivalent-ish pseudocode

        Renaming registers:
            a = (eliminated, only appears in initialization logic)
            b = current
            c = end
            d = div2
            e = div1
            f = is_prime
            g = (eliminated, used as a temp value for comparisons)
            h = output

        (initialize b and c)

        'full_restart:
        is_prime = true;
        div2 = 2;
        'restart_outer:
        div1 = 2;
        'restart_inner:
        if div1 * div2 == current {
            is_prime = false;
        }
        div1 += 1;
        if div1 == current {
            goto 'restart;
        }
        div2 += 1;
        if div2 == current {
            goto 'restart_with_e;
        }
        if !is_prime {
            output += 1;
        }
        if current == end {
            return;
        }
        current += 17;
        goto 'full_restart;

        ** Or, rewritten into Rust with loops:

        fn count_composites(mut current: usize, end: usize) -> usize {
            let mut output = 0;
            while current != end {
                let mut is_prime = true;
                for div2 in 2..end {
                    for div1 in 2..end {
                        if div1 * div2 == current {
                            is_prime = false;
                        }
                    }
                }

                if !is_prime {
                    output += 1;
                }
                current += 17;
            }
            output
        }

        So this program counts how many numbers in a range are composites, really slowly.
        Basically, [d] and [e] contain the two (larger than 1) divisors as it loops both up,
        checking each time if they multiply up to [e], the target number. [c] is the end of
        the range. [b] is the current number being checked. The range has a step of 17.

        So, ultimately we're just looking for the count of composite numbers in the range
        `(b..=c).step_by(17)`.
    */

    // Extract the initial values for [b] and [c] by running the initialization part
    // of the input program. This is my best attempt at generalizing this solution to
    // other inputs, though it's more of a blind guess.
    let mut p = Program::from_input(input);
    p.code.truncate(9);
    p.registers[0] = 1;
    p.run_until_blocked();

    let b = p.registers[1] as usize;
    let c = p.registers[2] as usize;

    Ok((b..=c).step_by(17).filter(|&n| !is_prime(n)).count())
}

/// Quick and dirty prime check function.
fn is_prime(n: usize) -> bool {
    if n % 2 == 0 {
        return false;
    }

    let mut divisor = 3;
    loop {
        if divisor * divisor > n {
            return true;
        } else if n % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
}

/// A runnable program, as per the puzzle description.
struct Program<'a> {
    registers: [i64; 8],
    code: Vec<Op<'a>>,
    pointer: i64,
    mul_uses: usize,
}

impl Program<'_> {
    /// Creates a new runnable program from a code listing.
    fn from_input(input: &str) -> Program {
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

        Program {
            registers: [0; 8],
            code: parse_ops(input).collect(),
            pointer: 0,
            mul_uses: 0,
        }
    }

    /// Runs the program until it blocks on a receive instruction; and returns all sended
    /// values produced in that time, in order.
    #[rustfmt::skip]
    fn run_until_blocked(&mut self) {
        if !(0..self.code.len() as i64).contains(&self.pointer) {
            return;
        }

        while let Some(op) = self.code.get(self.pointer as usize) {
            match op {
                Op { op: "set", arg1: Arg::Reg(r), arg2: Some(arg2) } => {
                    self.registers[*r as usize] = self.read(arg2);
                }
                Op { op: "sub", arg1: Arg::Reg(r), arg2: Some(arg2) } => {
                    self.registers[*r as usize] -= self.read(arg2);
                }
                Op { op: "mul", arg1: Arg::Reg(r), arg2: Some(arg2) } => {
                    self.mul_uses += 1;
                    self.registers[*r as usize] *= self.read(arg2);
                }
                Op { op: "jnz", arg1, arg2: Some(arg2) } => {
                    if self.read(arg1) != 0 {
                        self.pointer += self.read(arg2);
                        continue;
                    }
                }
                _ => {},
            }
            self.pointer += 1;
        }
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
