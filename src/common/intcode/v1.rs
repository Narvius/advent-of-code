//! First formalized implementation.
//!
//! There is a proto-implementation used in Day 2, but it is not worth extracting into
//! a shared module.

use std::collections::VecDeque;

pub type Int = i32;

/// An Intcode program, as per the puzzle description.
#[derive(Clone, Default)]
pub struct Program {
    code: Vec<Int>,
    pub input: VecDeque<Int>,
    pub output: VecDeque<Int>,
    pointer: usize,
}

/// Converts an integer into addressing [`Mode`]s.
struct Modes(Int);

/// An Intcode operation.
enum Op {
    Arith(fn(Int, Int) -> Int),
    Halt,
    Read,
    Write,
    JumpIf(bool),
}

/// An addressing mode used in Intcode programs.
enum Mode {
    Position,
    Immediate,
}

/// Describes the outcome for stepping a [`Program`].
pub enum Outcome {
    Halted,
    WaitingForInput,
    Ok,
}

impl Program {
    /// Builds a new [`Program`], parsing the code from a puzzle input, and populating 'stdin'
    /// for the program with `input`.
    pub fn new(code: &str, input: impl IntoIterator<Item = Int>) -> crate::Result<Self> {
        fn parse(input: &str) -> Result<Vec<Int>, impl std::error::Error> {
            input.trim().split(',').map(|n| n.parse::<Int>()).collect()
        }

        Ok(Self {
            code: parse(code)?,
            input: input.into_iter().collect(),
            ..Default::default()
        })
    }

    /// Executes one instruction of the program.
    pub fn step(&mut self) -> crate::Result<Outcome> {
        let Some(op) = self.code.get(self.pointer).copied() else {
            return Ok(Outcome::Halted);
        };
        let (op, mut modes) = ((op % 100).try_into()?, Modes(op / 100));

        match op {
            Op::Arith(f) => {
                let a = *self.resolve(1, modes.take()?)?;
                let b = *self.resolve(2, modes.take()?)?;
                let target = self.resolve(3, modes.take()?)?;

                *target = f(a, b);
                self.pointer += 4;
            }
            Op::Halt => return Ok(Outcome::Halted),
            Op::Read => match self.input.pop_front() {
                Some(v) => {
                    let target = self.resolve(1, modes.take()?)?;
                    *target = v;
                    self.pointer += 2;
                }
                None => return Ok(Outcome::WaitingForInput),
            },
            Op::Write => {
                let v = *self.resolve(1, modes.take()?)?;
                self.output.push_back(v);
                self.pointer += 2;
            }
            Op::JumpIf(cond) => {
                let v = *self.resolve(1, modes.take()?)?;
                self.pointer = if (v != 0) == cond {
                    usize::try_from(*self.resolve(2, modes.take()?)?)?
                } else {
                    self.pointer + 3
                };
            }
        }

        Ok(Outcome::Ok)
    }

    /// Returns a mutable reference to the actual value referred to by the cell
    /// `pointer + offset`, taking the addressing `mode` into account.
    fn resolve(&mut self, offset: usize, mode: Mode) -> crate::Result<&mut Int> {
        let code = &mut self.code;
        match mode {
            Mode::Position => {
                let index = *code
                    .get(self.pointer + offset)
                    .ok_or("not enough operands")?;
                Ok(code
                    .get_mut(usize::try_from(index)?)
                    .ok_or(format!("invalid indirect pointer {index}"))?)
            }
            Mode::Immediate => code
                .get_mut(self.pointer + offset)
                .ok_or_else(|| "not enough operands".into()),
        }
    }
}

impl Modes {
    /// Gets the next addressing mode, consuming it.
    fn take(&mut self) -> crate::Result<Mode> {
        let result = match self.0 % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => return Err(format!("unknown mode {}", self.0 % 10).into()),
        };
        self.0 /= 10;
        Ok(result)
    }
}

impl TryFrom<Int> for Op {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Int) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Op::Arith(|a, b| a + b),
            2 => Op::Arith(|a, b| a * b),
            3 => Op::Read,
            4 => Op::Write,
            5 => Op::JumpIf(true),
            6 => Op::JumpIf(false),
            7 => Op::Arith(|a, b| i32::from(a < b)),
            8 => Op::Arith(|a, b| i32::from(a == b)),
            99 => Op::Halt,
            _ => return Err(format!("unknown opcode {value}").into()),
        })
    }
}
