use std::collections::HashMap;

/// Running the redistribution logic, find the number of cycles at which the first repeated
/// configuration appears.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(run_distribution_until_repeat(input)?.0)
}

/// Running the redistribution logic, find the number of cycles after which the configurations
/// loop.
pub fn two(input: &str) -> crate::Result<i32> {
    Ok(run_distribution_until_repeat(input)?.1)
}

/// Runs the redistribution logic until a repeat is found, and returns both the number of total
/// cycles ran, and how long the detected cycle is.
fn run_distribution_until_repeat(input: &str) -> Result<(i32, i32), String> {
    let mut cells = input
        .split_whitespace()
        .map(|v| v.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "failed to parse input".to_owned())?;
    let mut seen = HashMap::new();
    let mut i = 0;
    let len = cells.len();

    loop {
        i += 1;
        let (index, v) =
            cells
                .iter()
                .copied()
                .enumerate()
                .fold((0, 0), |old, new| if new.1 > old.1 { new } else { old });

        cells[index] = 0;
        for n in 1..=v {
            cells[(index + n) % len] += 1;
        }

        if let Some(prev_i) = seen.insert(cells.clone(), i) {
            return Ok((i, i - prev_i));
        }
    }
}
