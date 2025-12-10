use z3::{ast::Int, Solver};

use crate::common;

/// Find the least number of buttons required to toggle on all the machines.
///
/// Solved efficiently by treating everything as bitmasks. A button is a bitmask of which bits it
/// toggles, the target configuration is a bitmask of what should be on. And then we simply go
/// through all patterns and find the shortest one that matches, each.
///
/// Because it's all bitmasks, all we need to do to press a button is XOR together the current
/// pattern and the button. That's why it's fast.
pub fn one(input: &str) -> crate::Result<u32> {
    Ok(parse(input)
        .map(|(config, buttons, _)| {
            (0..(1u32 << buttons.len()))
                .filter_map(|selection| {
                    (config == common::one_indices(selection).fold(0u32, |acc, i| acc ^ buttons[i]))
                        .then_some(selection.count_ones())
                })
                .min()
                .unwrap()
        })
        .sum())
}

/// We can "transpose" the buttons list to find a list of equations where each line is the sum that
/// produces each joltage. For example, with buttons `a(0, 2), b(1, 2), c(0, 1, 2)` and joltages
/// [10, 15, 20], we get these equations:
///
/// ```text
/// a +     c = 10
///     b + c = 15
/// a + b + c = 20
/// ````
type Equation = (Vec<i64>, i64);

/// Find the least number of buttons required to reach the correct joltage on all machines.
///
/// We just create a set of equations describing the result we want and toss it all into `z3`. I am
/// not proud of this.
pub fn two(input: &str) -> crate::Result<i64> {
    let mut result = 0;

    for (_, buttons, joltages) in parse(input) {
        let equations: Vec<Equation> = joltages
            .iter()
            .enumerate()
            .map(|(i, joltage)| {
                (
                    (buttons
                        .iter()
                        .map(|&b| i64::from(common::one_indices(b).any(|j| i == j))))
                    .collect(),
                    *joltage,
                )
            })
            .collect();

        let vars: Vec<_> = (0..buttons.len())
            .map(|n| Int::fresh_const(&format!("x{n}")))
            .collect();

        let solver = Solver::new();

        let cap = joltages.iter().copied().max().unwrap();
        for var in &vars {
            solver.assert(var.ge(0));
            solver.assert(var.le(cap));
        }

        for (coeffs, n) in equations {
            let v = vars
                .iter()
                .cloned()
                .zip(coeffs)
                .filter_map(|(v, c)| (c == 1).then_some(v))
                .reduce(|a, b| a + b)
                .unwrap();

            solver.assert(v.eq(n));
        }

        result += solver
            .solutions(vars, false)
            .map(|vs| vs.into_iter().map(|v| v.as_i64().unwrap()).sum::<i64>())
            .min()
            .unwrap();
    }

    Ok(result)
}

/// Parses the puzzle input, providing a target bitmask, list of button bitmasks, and list of
/// joltages per line.
fn parse(input: &str) -> impl Iterator<Item = (u32, Vec<u32>, Vec<i64>)> + '_ {
    (input.lines()).filter_map(|line| {
        let (pattern, rest) = line.trim_matches(&['[', '}'][..]).split_once("] (")?;
        let (buttons, joltages) = rest.split_once(") {")?;

        let pattern = pattern
            .chars()
            .rev()
            .fold(0u32, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 });
        let buttons = buttons
            .split(") (")
            .map(|button| {
                button
                    .split(',')
                    .map(|n| n.parse::<u32>().unwrap())
                    .fold(0u32, |acc, n| acc | (1 << n))
            })
            .collect();
        let joltages = joltages.split(',').map(|n| n.parse().unwrap()).collect();

        Some((pattern, buttons, joltages))
    })
}
