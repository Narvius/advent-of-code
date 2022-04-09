/// Find the difference between character counts in the representation and value of the input
/// strings.
pub fn one(input: &str) -> Result<String, String> {
    Ok(input
        .lines()
        .map(extra_representation_chars)
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

/// Counts how many extra characters there are in the representation than in memory.
fn extra_representation_chars(s: &str) -> usize {
    let mut result = 2;
    let mut escaping = false;

    for c in s.chars() {
        if escaping {
            result += if c == 'x' { 3 } else { 1 };
            escaping = false;
        } else if c == '\\' {
            escaping = true;
        }
    }

    result
}
