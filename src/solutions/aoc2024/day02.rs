/// Count how many reports are `safe`.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(parse(input).filter(|r| safe(r)).count())
}

/// Count how many reports are `safe` if you can remove any one element.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(parse(input)
        .filter(|r| {
            (0..r.len()).any(|i| {
                let mut r = r.clone();
                r.remove(i);
                safe(&r)
            })
        })
        .count())
}

/// Checks that a report is safe (the series is either montononically increasing or monotonically
/// decreasing; and all differences between adjacent numbers are at least 1 and at most 3).
fn safe(r: &[i32]) -> bool {
    let monotonic = r.windows(2).all(|w| w[0] < w[1]) || r.windows(2).all(|w| w[0] > w[1]);
    let steady = r.windows(2).all(|w| (1..=3).contains(&w[0].abs_diff(w[1])));

    monotonic && steady
}

/// Parses the puzzle input into a series of reports (each of which is a list of numbers).
fn parse(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input.lines().map(|line| {
        line.split_ascii_whitespace()
            .filter_map(|t| t.parse::<i32>().ok())
            .collect()
    })
}
