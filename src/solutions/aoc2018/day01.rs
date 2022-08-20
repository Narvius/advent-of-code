use std::collections::HashSet;

/// Sum all numbers in the input.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(input.lines().filter_map(|l| l.parse::<i32>().ok()).sum())
}

/// Find the first partial sum that repeats.
pub fn two(input: &str) -> crate::Result<i32> {
    let mut set = HashSet::new();
    let mut sum = 0;
    let numbers: Vec<_> = input
        .lines()
        .filter_map(|l| l.parse::<i32>().ok())
        .collect();

    loop {
        for &n in &numbers {
            sum += n;
            if !set.insert(sum) {
                return Ok(sum);
            }
        }
    }
}
