/// Find the sum of six specific signal strenghts.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(Program::from_input(input)
        .skip(19)
        .step_by(40)
        .zip((20..).step_by(40))
        .take(6)
        .fold(0, |acc, (a, b)| acc + a * b))
}

/// Use the program to draw a string containing 8 capital letters.
pub fn two(input: &str) -> crate::Result<String> {
    let mut crt = String::with_capacity(41 * 6);
    let mut program = Program::from_input(input);
    for _ in 0..6 {
        crt.push('\n');
        for x in 0..40 {
            let val = program.next().unwrap_or(0);
            crt.push(if (x - val).abs() <= 1 { '#' } else { '.' });
        }
    }
    Ok(crt)
}

/// A program from puzzle input. An iterator that returns the values after the `n`th cycle,
/// starting at n = 1 on the element.
struct Program {
    ops: Vec<(i32, usize)>,
    reg: i32,
    wait: usize,
}

impl Program {
    /// Parses the puzzle input into a [`Program`].
    fn from_input(input: &str) -> Self {
        let mut code: Vec<(i32, usize)> = input
            .lines()
            .filter_map(|line| {
                Some(match line.split_once(' ') {
                    Some((_, val)) => (val.parse().ok()?, 2),
                    None => (0, 1),
                })
            })
            .collect();
        code.reverse();
        let wait = code[0].1;
        Self {
            ops: code,
            reg: 1,
            wait,
        }
    }
}

impl Iterator for Program {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let (reg_change, new_wait) = match self.wait - 1 {
            0 => {
                let (change, _) = self.ops.pop()?;
                let wait = self.ops.last()?.1;
                (change, wait)
            }
            wait => (0, wait),
        };

        let old_reg = self.reg;
        self.reg += reg_change;
        self.wait = new_wait;

        Some(old_reg)
    }
}
