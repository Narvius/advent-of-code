/// Find the amount of wrapping paper needed.
pub fn one(input: &str) -> crate::Result<String> {
    Ok(solve(input, |(w, h, d)| {
        2 * (w * h + h * d + d * w) + (w * h).min(h * d).min(d * w)
    })
    .to_string())
}

/// Find the amount of ribbon needed.
pub fn two(input: &str) -> crate::Result<String> {
    Ok(solve(input, |(w, h, d)| {
        2 * (w + h + d - w.max(h).max(d)) + w * h * d
    })
    .to_string())
}

/// Runs the given formula for each line of input and sums up the results.
fn solve(input: &str, formula: fn((i32, i32, i32)) -> i32) -> i32 {
    input.lines().filter_map(dimensions).map(formula).sum()
}

/// Parses a line of puzzle input.
fn dimensions(line: &str) -> Option<(i32, i32, i32)> {
    let mut xs = line.split('x').map(|t| t.parse().ok());
    Some((xs.next()??, xs.next()??, xs.next()??))
}
