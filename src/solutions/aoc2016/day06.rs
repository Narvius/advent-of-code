use std::collections::HashMap;

/// Use the most common letter in each column to find to construct a word.
pub fn one(input: &str) -> Result<String, String> {
    decode(input, |m| m.into_iter().max_by_key(|p| p.1).map(|p| p.0))
}

/// Use the least common letter in each column to find to construct a word.
pub fn two(input: &str) -> Result<String, String> {
    decode(input, |m| m.into_iter().min_by_key(|p| p.1).map(|p| p.0))
}

/// Tallies occurences of characters in each column, then `pick`s one character for each column
/// in order to produce an output string.
fn decode(input: &str, pick: fn(HashMap<char, i32>) -> Option<char>) -> Result<String, String> {
    let mut maps = vec![HashMap::new(); 8];
    for line in input.lines() {
        for (i, c) in line.char_indices() {
            *maps[i].entry(c).or_insert(0) += 1;
        }
    }
    Ok(maps.into_iter().filter_map(|map| pick(map)).collect())
}
