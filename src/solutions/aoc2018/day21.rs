use std::collections::HashSet;

/// Find the smallest number for register #0 that will cause the program to halt.
pub fn one(input: &str) -> crate::Result<i64> {
    Ok(input.parse::<Machine>()?.get_comparison_value())
}

/// Find the largest number for register #0 that will cause the program to halt.
///
/// The solution is quite slow; it boils down to brute force: We keep running into the
/// ending comparison (where register 0 is actually used) until that value repeats.
/// The value before the repeating one is the answer.
///
/// Calculating it faster would probably mean reimplementing the calculation by hand.
pub fn two(input: &str) -> crate::Result<i64> {
    let mut m: Machine = input.parse()?;
    let mut previous = 0;
    let mut seen = HashSet::new();

    loop {
        let candidate = m.get_comparison_value();
        if !seen.insert(candidate) {
            return Ok(previous);
        }
        previous = candidate;
    }
}

/// The machine described in the puzzle input.
#[derive(Clone)]
struct Machine {
    /// Index of the register mapped to the instruction pointer.
    ip_register: usize,
    /// The instruction pointer.
    ip: usize,
    /// The program stored on the machine.
    rom: Vec<[i64; 4]>,
    /// The current state of the registers.
    ram: [i64; 6],
}

impl Machine {
    /// Performs one instruction.
    fn step(&mut self) {
        if self.ip >= self.rom.len() {
            return;
        }

        self.ram[self.ip_register] = self.ip as i64;
        eval(
            OPS[self.rom[self.ip][0] as usize].1,
            &self.rom[self.ip],
            &mut self.ram,
        );
        self.ip = self.ram[self.ip_register] as usize + 1;
    }

    /// Runs the machine until a comparison with register #0 is made; and then returns the
    /// value is gets compared to. Based on the observation that register 0 is only ever
    /// used in that specific comparison at the end, and is used for terminating the
    /// entire program.
    fn get_comparison_value(&mut self) -> i64 {
        // Run until a "gtrr" instruction is reached; but step at least once, in case we're
        // already paused on one.
        self.step();
        while self.rom[self.ip][0] != 15 {
            self.step();
        }
        // Find the non-zero operand. Since one is always zero, we can just add them to do that,
        // instead of some convoluted selection logic.
        let register = self.rom[self.ip][1] + self.rom[self.ip][2];
        self.ram[register as usize]
    }
}

/// Executes an operation with the given arguments.
fn eval(op: Op, args: &[i64], reg: &mut [i64]) {
    let ra = reg.get(args[1] as usize).copied().unwrap_or(i64::MAX);
    let rb = reg.get(args[2] as usize).copied().unwrap_or(i64::MAX);

    reg[args[3] as usize] = op(ra, args[1], rb, args[2]);
}

impl std::str::FromStr for Machine {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let ip_register = lines[0].split_once(' ').ok_or("failed parse")?.1.parse()?;
        let rom = {
            let mut rom = vec![];
            for line in &lines[1..] {
                let mut entry = Vec::with_capacity(4);
                let mut tokens = line.split_ascii_whitespace();
                let token = tokens.next().ok_or("failed parse")?;
                entry.push(
                    OPS.iter()
                        .enumerate()
                        .find(|(_, &(name, _))| name == token)
                        .ok_or("parse failed")?
                        .0 as i64,
                );
                entry.extend(tokens.filter_map(|v| v.parse::<i64>().ok()));
                let entry: [i64; 4] = entry.try_into().ok().ok_or("failed parse")?;
                rom.push(entry);
            }
            rom
        };

        Ok(Machine {
            ip_register,
            ip: 0,
            rom,
            ram: [0; 6],
        })
    }
}

type Op = fn(i64, i64, i64, i64) -> i64;

const OPS: [(&str, Op); 16] = [
    ("addr", |ra, _, rb, _| ra + rb),
    ("addi", |ra, _, _, vb| ra + vb),
    ("mulr", |ra, _, rb, _| ra * rb),
    ("muli", |ra, _, _, vb| ra * vb),
    ("banr", |ra, _, rb, _| ra & rb),
    ("bani", |ra, _, _, vb| ra & vb),
    ("borr", |ra, _, rb, _| ra | rb),
    ("bori", |ra, _, _, vb| ra | vb),
    ("setr", |ra, _, _, _| ra),
    ("seti", |_, va, _, _| va),
    ("gtir", |_, va, rb, _| i64::from(va > rb)),
    ("gtri", |ra, _, _, vb| i64::from(ra > vb)),
    ("gtrr", |ra, _, rb, _| i64::from(ra > rb)),
    ("eqir", |_, va, rb, _| i64::from(va == rb)),
    ("eqri", |ra, _, _, vb| i64::from(ra == vb)),
    ("eqrr", |ra, _, rb, _| i64::from(ra == rb)),
];
