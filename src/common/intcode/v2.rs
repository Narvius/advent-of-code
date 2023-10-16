//! Expanded version of the intcode computer.
//!
//! Compared to [`v1`](super::v1), it supports relative addressing. This means new internal state
//! (the relative base), a new opcode to set the relative base, and a new addressing mode.
//!
//! Additionally, there is a new constructor that allows to set the size of the program space.

use std::collections::VecDeque;

pub type Int = i64;

/// An Intcode program, as per the puzzle description.
#[derive(Clone, Default)]
pub struct Program {
    code: Vec<Int>,
    pub input: VecDeque<Int>,
    pub output: VecDeque<Int>,
    pointer: usize,
    relative_base: Int,
}

/// An Intcode operation.
enum Op {
    Arith(fn(Int, Int) -> Int),
    Halt,
    Read,
    Write,
    JumpIf(bool),
    ChangeRelativeBase,
}

/// An addressing mode used in Intcode programs.
enum Mode {
    Position,
    Immediate,
    Relative,
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

    /// Builds a new [`Program`], and pads the code to have at least `capacity` cells.
    pub fn with_capacity(
        code: &str,
        capacity: usize,
        input: impl IntoIterator<Item = Int>,
    ) -> crate::Result<Self> {
        let mut result = Self::new(code, input)?;
        result
            .code
            .extend(std::iter::repeat(0).take(capacity.saturating_sub(result.code.len())));
        Ok(result)
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
            Op::ChangeRelativeBase => {
                self.relative_base += *self.resolve(1, m1)?;
                self.pointer += 2;
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
            Mode::Relative => self.relative_base + self.code[self.pointer + offset],
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
            9 => Op::ChangeRelativeBase,
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
            2 => Mode::Relative,
            _ => return Err(format!("unknown addressing mode {value}").into()),
        })
    }
}
