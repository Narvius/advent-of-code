use std::collections::VecDeque;

/// Find the first allowed IP address.
pub fn one(input: &str) -> Result<String, String> {
    let mut ranges = parse(input);
    find_next_allowed(&mut ranges)
        .map(|s| s.to_string())
        .ok_or_else(|| "no result".to_owned())
}

/// Counts the number of allowed IP addresses.
pub fn two(input: &str) -> Result<String, String> {
    let (mut ranges, mut result) = (parse(input), 0);
    while let Some(allowed_lo) = find_next_allowed(&mut ranges) {
        let allowed_hi = ranges.front().map(|i| i.0 - 1).unwrap_or(u32::MAX);
        result += allowed_hi.saturating_sub(allowed_lo);
    }
    Ok(result.to_string())
}

/// Given a a sorted queue of intervals, consumes all that form one combined interval, and returns
/// the first number past that combined interval.
fn find_next_allowed(ranges: &mut VecDeque<(u32, u32)>) -> Option<u32> {
    let mut max = ranges.front()?.0;
    while let Some((lo, hi)) = ranges.pop_front() {
        if max.checked_add(1)? < lo {
            ranges.push_front((lo, hi));
            return Some(max);
        }
        max = hi.max(max);
    }
    None
}

/// Parses the puzzle input into a sorted queue of intervals.
fn parse(input: &str) -> VecDeque<(u32, u32)> {
    let mut items: Vec<(u32, u32)> = input
        .lines()
        .filter_map(|line| {
            let (lo, hi) = line.split_once('-')?;
            Some((lo.parse().ok()?, hi.parse().ok()?))
        })
        .collect();
    items.sort_by_key(|(lo, _)| *lo);
    items.into_iter().collect()
}
