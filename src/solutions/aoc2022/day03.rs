use std::collections::HashSet;

/// For each line, find the common letter between the first and last line; sum the priorities
/// of all such letters.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(input
        .lines()
        .filter_map(|s| {
            let (a, b) = s.split_at(s.len() / 2);
            Some(priority(*letters(a).intersection(&letters(b)).next()?))
        })
        .sum())
}

/// For each chunk of three lines, find the letter common among all three of them; sum the
/// priorities of all such letters.
pub fn two(input: &str) -> crate::Result<i32> {
    let lines: Vec<_> = input.lines().collect();
    Ok(lines
        .chunks(3)
        .filter_map(|chunk| {
            let combined = chunk.iter().map(|line| letters(line)).reduce(|mut a, b| {
                a.retain(|v| b.contains(v));
                a
            })?;

            Some(priority(combined.into_iter().next()?))
        })
        .sum())
}

/// Make a hashmap out of the bytes of a string.
fn letters(s: &str) -> HashSet<u8> {
    s.bytes().collect()
}

/// Converts an item (represented by an ASCII character) to a priority score.
fn priority(v: u8) -> i32 {
    match v {
        b'a'..=b'z' => (1 + v - b'a') as i32,
        _ => (27 + v - b'A') as i32,
    }
}
