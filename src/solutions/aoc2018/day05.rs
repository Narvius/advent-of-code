/// Collapse the polymer as given, and return the length.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(collapsed_length(input, None))
}

/// Find the shortest possible polymer length when ignoring one type of unit.
pub fn two(input: &str) -> crate::Result<usize> {
    (b'a'..=b'z')
        .map(|c| collapsed_length(input, Some(c)))
        .min()
        .ok_or_else(|| "no shortest sequence found".into())
}

/// Calculates the length of the polymer after collapsing it and removing the provided unit.
fn collapsed_length(polymer: &str, ignore: Option<u8>) -> usize {
    let mut stack = Vec::new();

    const CASE_DIFFERENCE: u8 = b'a' - b'A';

    for c in polymer.bytes() {
        if stack.last().map(|&s| c.abs_diff(s)) == Some(CASE_DIFFERENCE) {
            stack.pop();
        } else if Some(c.to_ascii_lowercase()) != ignore {
            stack.push(c);
        }
    }

    stack.len()
}
