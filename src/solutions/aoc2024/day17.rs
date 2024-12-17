use std::cmp::Ordering;

/// Run the program, get the output.
pub fn one(input: &str) -> crate::Result<String> {
    let mut m = parse(input).ok_or("parse failed")?;
    m.run();

    Ok(m.output
        .into_iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(","))
}

/// Find the lowest starting value for register `A` that would result in the program printing
/// itself.
///
/// # Concept
///
/// By manually inspecting the program and running it with various test values, we can derive two
/// important properties:
/// - the program produces one output per 3 bits (octet) of `A`
/// - the output value only depends on its own octet AND potentially any octets to the left
///
/// Thus, we can safely construct `A` octet-by-octet starting from the most-significant one--if we
/// got it "right", it will produce an output equal to some tail of the full program. This heavily
/// constrains the search space (which would otherwise be 8^(program length)), so that at every
/// step we only consider octets that match the tail.
///
/// [`find_lowest_code`] performs this search recursively. It runs the program with the `A` it is
/// given, if the input is too short, it checks if it matches the tail of the program at least; if
/// so, it recursively tries all 8 possible next octets. Repeat until the desired output length is
/// reached, at which point we can check for equality to find our final answer.
pub fn two(input: &str) -> crate::Result<i64> {
    let m = parse(input).ok_or("parse failed")?;

    (1..8)
        .filter_map(|a| find_lowest_code(&m, a))
        .next()
        .ok_or("no result".into())
}

/// Finds the answer for [`two`]. See there for details.
fn find_lowest_code(machine: &Machine, a: i64) -> Option<i64> {
    let mut m = machine.clone();
    m.registers[0] = a;
    m.run();

    match m.output.len().cmp(&machine.program.len()) {
        Ordering::Less if machine.program.ends_with(&m.output) => (0..8)
            .map(|n| (a << 3) + n)
            .filter_map(|a| find_lowest_code(machine, a))
            .next(),
        Ordering::Equal => (m.output == machine.program).then_some(a),
        _ => None,
    }
}

/// The computer described in the puzzle.
#[derive(Clone)]
struct Machine {
    registers: [i64; 3],
    program: Vec<u8>,
    pointer: usize,
    output: Vec<u8>,
}

impl Machine {
    /// Resolves a "combo" operand.
    fn resolve_combo(&self, value: u8) -> i64 {
        match value {
            0..=3 => value as i64,
            4..=6 => self.registers[value as usize - 4],
            _ => panic!("invalid combo"),
        }
    }

    /// Performs an operation, mutating the registers, pointer, and output.
    fn step(&mut self, op: u8, operand: u8) {
        match op {
            0 => {
                let (lhs, rhs) = (self.registers[0], self.resolve_combo(operand));
                self.registers[0] = lhs >> rhs;
            }
            1 => {
                self.registers[1] ^= operand as i64;
            }
            2 => {
                self.registers[1] = self.resolve_combo(operand) % 8;
            }
            3 => {
                if self.registers[0] != 0 {
                    self.pointer = operand as usize;
                    return;
                }
            }
            4 => {
                self.registers[1] ^= self.registers[2];
            }
            5 => {
                self.output.push((self.resolve_combo(operand) % 8) as u8);
            }
            6 => {
                let (lhs, rhs) = (self.registers[0], self.resolve_combo(operand));
                self.registers[1] = lhs >> rhs;
            }
            7 => {
                let (lhs, rhs) = (self.registers[0], self.resolve_combo(operand));
                self.registers[2] = lhs >> rhs;
            }
            _ => panic!("unknown opcode {op}"),
        }

        self.pointer += 2;
    }

    /// Runs the program until it halts.
    fn run(&mut self) {
        while let [op, operand, ..] = &self.program[self.pointer..] {
            self.step(*op, *operand);
        }
    }
}

/// Parses the initial computer state and program from the input.
fn parse(input: &str) -> Option<Machine> {
    let mut lines = input
        .lines()
        .filter_map(|line| line.split_once(": ").map(|s| s.1));

    let registers = [
        lines.next()?.parse().ok()?,
        lines.next()?.parse().ok()?,
        lines.next()?.parse().ok()?,
    ];

    let program = (lines.next()?)
        .split(',')
        .filter_map(|t| t.parse().ok())
        .collect();

    Some(Machine {
        registers,
        program,
        pointer: 0,
        output: vec![],
    })
}
