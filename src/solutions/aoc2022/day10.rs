/// Find the sum of six specific signal strengths.
pub fn one(input: &str) -> crate::Result<i32> {
    let values = RegisterValues::from_input(input);
    let chosen_six = values.zip(1..).skip(19).step_by(40).take(6);
    Ok(chosen_six.map(|(register, cycle)| register * cycle).sum())
}

/// Draw a 40x6 image containing a text string, using the register values to decide which
/// pixels are lit.
pub fn two(input: &str) -> crate::Result<String> {
    let mut crt = String::with_capacity(41 * 6);
    let mut values = RegisterValues::from_input(input);
    for _ in 0..6 {
        crt.push('\n');
        for x in 0..40 {
            let val = values.next().unwrap_or(0);
            crt.push(if (x - val).abs() <= 1 { '#' } else { '.' });
        }
    }
    Ok(crt)
}

/// An iterator over the register values produced during execution of a program like the one
/// provided in the input.
struct RegisterValues {
    /// List of (register change, time until completion) pairs.
    ops: Vec<(i32, usize)>,
    /// The current register value.
    reg: i32,
    /// CUrrent amount of steps to wait before the next op begins.
    wait: usize,
}

impl RegisterValues {
    /// Parses the puzzle input into a [`RegisterValues`] iterator.
    fn from_input(input: &str) -> Self {
        let code: Vec<(i32, usize)> = input
            .lines()
            .rev()
            .filter_map(|line| {
                Some(match line.split_once(' ') {
                    Some((_, val)) => (val.parse().ok()?, 2),
                    None => (0, 1),
                })
            })
            .collect();
        let wait = code[0].1;
        Self {
            ops: code,
            reg: 1,
            wait,
        }
    }
}

impl Iterator for RegisterValues {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let (reg_change, new_wait) = match self.wait - 1 {
            0 => {
                let (change, _) = self.ops.pop()?;
                (change, self.ops.last()?.1)
            }
            wait => (0, wait),
        };

        let old_reg = self.reg;
        self.reg += reg_change;
        self.wait = new_wait;

        Some(old_reg)
    }
}
