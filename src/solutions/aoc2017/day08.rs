use std::collections::HashMap;

/// Find the highest value contained in any register after executing all instructions.
pub fn one(input: &str) -> crate::Result<i32> {
    let mut registers = HashMap::new();
    for line in parse(input) {
        line.apply(&mut registers);
    }
    registers.into_values().max().ok_or_else(|| "no result".into())
}

/// Find the highest value ever reached by any register during execution.
pub fn two(input: &str) -> crate::Result<i32> {
    let mut registers = HashMap::new();
    let mut highest = None;
    for line in parse(input) {
        line.apply(&mut registers);
        highest = highest.max(registers.values().max().copied());
    }
    highest.ok_or_else(|| "no result".into())
}

/// A single instruction, comprised of an action and a precondition.
struct Line<'a> {
    target: &'a str,
    increment: bool,
    amount: i32,
    check: (&'a str, &'a str, i32),
}

impl<'a> Line<'a> {
    /// Checks whether this line should execute, given a set of register values.
    fn check(&self, registers: &mut HashMap<&'a str, i32>) -> bool {
        let (lhs, op, rhs) = self.check;
        let lhs = *registers.entry(lhs).or_default();
        match op {
            "<" => lhs < rhs,
            "<=" => lhs <= rhs,
            ">" => lhs > rhs,
            ">=" => lhs >= rhs,
            "==" => lhs == rhs,
            "!=" => lhs != rhs,
            _ => false,
        }
    }

    /// Applies the action of this line to the given set of register values.
    fn apply(&self, registers: &mut HashMap<&'a str, i32>) {
        if self.check(registers) {
            let reg = registers.entry(self.target).or_default();
            if self.increment {
                *reg += self.amount;
            } else {
                *reg -= self.amount;
            }
        }
    }
}

/// Parses the puzzle input into a series of instructions.
fn parse(input: &str) -> impl Iterator<Item = Line> + '_ {
    input.lines().filter_map(|line| {
        let data: Vec<_> = line.split_whitespace().collect();
        if let &[target, inc, amount, _if, lhs, op, rhs] = &data[..] {
            Some(Line {
                target,
                increment: inc == "inc",
                amount: amount.parse().ok()?,
                check: (lhs, op, rhs.parse().ok()?),
            })
        } else {
            None
        }
    })
}
