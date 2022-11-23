/// Execute the program.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(sum_of_divisors(input.parse::<Machine>()?.get_target(0)))
}

/// Execute the program with altered initial conditions. Note that if just ran, it takes
/// a very long time. It's an inefficient algorithm for finding all divisors of a number (see
/// data/aoc2018/day19_annotated.txt). So, we just run enough to find the target number,
/// and then do a primitive way of finding the same number in pure Rust.
pub fn two(input: &str) -> crate::Result<i32> {
    Ok(sum_of_divisors(input.parse::<Machine>()?.get_target(1)))
}

/// The machine described in the puzzle input.
struct Machine {
    /// Index of the register mapped to the instruction pointer.
    ip_register: usize,
    /// The instruction pointer.
    ip: usize,
    /// The program stored on the machine.
    rom: Vec<[i32; 4]>,
    /// The current state of the registers.
    ram: [i32; 6],
}

impl Machine {
    /// Performs one instruction.
    fn step(&mut self) {
        if self.ip >= self.rom.len() {
            return;
        }

        self.ram[self.ip_register] = self.ip as i32;
        eval(
            OPS[self.rom[self.ip][0] as usize].1,
            &self.rom[self.ip],
            &mut self.ram,
        );
        self.ip = self.ram[self.ip_register] as usize + 1;
    }

    /// Gets the target number (see description of [`two`]).
    fn get_target(&mut self, input: i32) -> i32 {
        self.ram[0] = input;
        while self.ip != 3 {
            self.step();
        }
        self.ram[5]
    }
}

/// Finds the sum of all divisors of a number (including itself).
fn sum_of_divisors(n: i32) -> i32 {
    (1..=n).filter(|i| (n % i) == 0).sum()
}

/// Executes an operation with the given arguments.
fn eval(op: Op, args: &[i32], reg: &mut [i32]) {
    let ra = reg.get(args[1] as usize).copied().unwrap_or(i32::MAX);
    let rb = reg.get(args[2] as usize).copied().unwrap_or(i32::MAX);

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
                        .0 as i32,
                );
                entry.extend(tokens.filter_map(|v| v.parse::<i32>().ok()));
                let entry: [i32; 4] = entry.try_into().ok().ok_or("failed parse")?;
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

type Op = fn(i32, i32, i32, i32) -> i32;

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
    ("gtir", |_, va, rb, _| i32::from(va > rb)),
    ("gtri", |ra, _, _, vb| i32::from(ra > vb)),
    ("gtrr", |ra, _, rb, _| i32::from(ra > rb)),
    ("eqir", |_, va, rb, _| i32::from(va == rb)),
    ("eqri", |ra, _, _, vb| i32::from(ra == vb)),
    ("eqrr", |ra, _, rb, _| i32::from(ra == rb)),
];
