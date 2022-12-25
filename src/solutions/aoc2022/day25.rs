/// Sum the balanced quinary numbers from input, and represent the sum in
/// balanced quinary.
pub fn one(input: &str) -> crate::Result<String> {
    Ok(to_snafu(input.lines().map(from_snafu).sum()))
}

/// Freebie!
pub fn two(_input: &str) -> crate::Result<&str> {
    Ok("done!")
}

/// Converts from the puzzle's balanced quinary representation to decimal.
fn from_snafu(n: &str) -> i64 {
    n.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            5i64.pow(i as u32)
                * match c {
                    '2' => 2,
                    '1' => 1,
                    '-' => -1,
                    '=' => -2,
                    _ => 0,
                }
        })
        .sum()
}

/// Converts from decimal to the puzzle's balanced quinary representation.
fn to_snafu(mut n: i64) -> String {
    let mut result = String::new();

    while n != 0 {
        let char = "012=-".as_bytes()[n as usize % 5] as char;
        result.insert(0, char);

        n -= (n + 2) % 5 - 2;
        n /= 5;
    }

    result
}
