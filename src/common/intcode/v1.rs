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

        let (op, m1, m2, m3) = decompose(op)?;
        match op {
            Op::Arith(f) => {
                let a = *self.resolve(1, m1)?;
                let b = *self.resolve(2, m2)?;
                let target = self.resolve(3, m3)?;

                *target = f(a, b);
                self.pointer += 4;
            }
            Op::Halt => return Ok(Outcome::Halted),
            Op::Read => match self.input.pop_front() {
                Some(v) => {
                    let target = self.resolve(1, m1)?;
                    *target = v;
                    self.pointer += 2;
                }
                None => return Ok(Outcome::WaitingForInput),
            },
            Op::Write => {
                let v = *self.resolve(1, m1)?;
                self.output.push_back(v);
                self.pointer += 2;
            }
            Op::JumpIf(cond) => {
                let v = *self.resolve(1, m1)?;
                self.pointer = if (v != 0) == cond {
                    usize::try_from(*self.resolve(2, m2)?)?
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
        let index = match mode {
            Mode::Position => self.code[self.pointer + offset],
            Mode::Immediate => (self.pointer + offset) as Int,
        };
        Ok(&mut self.code[usize::try_from(index)?])
    }
}

fn decompose(code: Int) -> crate::Result<(Op, Mode, Mode, Mode)> {
    Ok((
        (code % 100).try_into()?,
        ((code / 100) % 10).try_into()?,
        ((code / 1000) % 10).try_into()?,
        ((code / 10000) % 10).try_into()?,
    ))
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
            7 => Op::Arith(|a, b| Int::from(a < b)),
            8 => Op::Arith(|a, b| Int::from(a == b)),
            99 => Op::Halt,
            _ => return Err(format!("unknown opcode {value}").into()),
        })
    }
}

impl TryFrom<Int> for Mode {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Int) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => return Err(format!("unknown addressing mode {value}").into()),
        })
    }
}
