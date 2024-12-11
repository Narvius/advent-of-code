use std::collections::HashMap;

/// Count the number of rocks after blinking 25 times.
pub fn one(input: &str) -> crate::Result<u128> {
    Ok(blink(input, 25))
}

/// Count the number of rocks after blinking 75 times.
pub fn two(input: &str) -> crate::Result<u128> {
    Ok(blink(input, 75))
}

/// Runs the blinking logic described in the puzzle `times` times on the `input`, and returns the
/// total resulting number of rocks.
///
/// Note that while the puzzle specifies that "order is preserved", it never matters; so we can do
/// a simple bucketing strategy.
fn blink(input: &str, times: u128) -> u128 {
    let mut rocks: HashMap<i64, u128> = input
        .split_ascii_whitespace()
        .filter_map(|n| Some((n.parse().ok()?, 1)))
        .collect();

    for _ in 0..times {
        let mut new = HashMap::new();
        for (key, value) in rocks {
            if key == 0 {
                *new.entry(1).or_default() += value;
            } else if let Some((a, b)) = split_number(key) {
                *new.entry(a).or_default() += value;
                *new.entry(b).or_default() += value;
            } else {
                *new.entry(2024 * key).or_default() += value;
            }
        }
        rocks = new;
    }

    rocks.into_values().sum()
}

/// If a number has an even amount of digits, returns a pair of numbers representing the left half
/// and right half of digits, respectively.
fn split_number(n: i64) -> Option<(i64, i64)> {
    let len = (f64::log10(n as f64) + 1.0) as u32;
    (len % 2 == 0).then_some((n / 10i64.pow(len / 2), n % 10i64.pow(len / 2)))
}
