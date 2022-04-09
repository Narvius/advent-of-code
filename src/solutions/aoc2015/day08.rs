/// Find the difference between character counts in the representation and value of the input
/// strings.
pub fn one(input: &str) -> Result<String, String> {
    Ok(input
        .lines()
        .map(|s| s.len() - memory_chars(s))
        .sum::<usize>()
        .to_string())
}

/// Find the difference between character counts in the representation of the representation, and
/// the representation of the input strings.
pub fn two(input: &str) -> Result<String, String> {
    Ok(input
        .lines()
        .map(|s| format!("{:?}", s).len() - s.len())
        .sum::<usize>()
        .to_string())
}

/// Counts the number of characters in memory for a given string representation.
fn memory_chars(s: &str) -> usize {
    // Start at -2 to offset the surrounding quotes.
    let mut count = -2;
    let mut escaping = false;
    for b in s.bytes() {
        match (escaping, b) {
            (false, b'\\') => escaping = true,
            (true, b'x') => {
                // \xFF is a four-character escape that produces 1 character.
                // The \ is ignored, the x subtracts one, and the next two add one each, working
                // out to the expected +1 for the entire sequence.
                escaping = false;
                count -= 1;
            }
            (true, _) => {
                escaping = false;
                count += 1;
            }
            (false, _) => count += 1,
        }
    }
    count as usize
}
