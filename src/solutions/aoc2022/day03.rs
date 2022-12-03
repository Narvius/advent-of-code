use std::collections::HashSet;

/// For each line, find the common letter between the first and last line; sum the priorities
/// of all such letters.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(input
        .lines()
        .filter_map(|s| {
            let (a, b) = s.split_at(s.len() / 2);
            shared_item_priority(&[a, b])
        })
        .sum())
}

/// For each chunk of three lines, find the letter common among all three of them; sum the
/// priorities of all such letters.
pub fn two(input: &str) -> crate::Result<i32> {
    let lines: Vec<_> = input.lines().collect();
    Ok(lines.chunks(3).filter_map(shared_item_priority).sum())
}

/// Given a slice of strings, finds the character shared between all of them; then returns
/// its priority score (defined by the puzzle).
fn shared_item_priority(groups: &[&str]) -> Option<i32> {
    let combined = groups
        .iter()
        .map(|s| s.bytes().collect::<HashSet<_>>())
        .reduce(|mut a, b| {
            a.retain(|v| b.contains(v));
            a
        })?;

    let v = combined.into_iter().next()?;
    match v {
        b'a'..=b'z' => Some((1 + v - b'a') as i32),
        b'A'..=b'Z' => Some((27 + v - b'A') as i32),
        _ => None,
    }
}
