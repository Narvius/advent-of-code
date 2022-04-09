/// Find the final floor the elevator stops at.
pub fn one(input: &str) -> Result<String, String> {
    Ok(input.chars().map(as_delta).sum::<i32>().to_string())
}

/// Find the number of steps after which the elevator first goes into the basement.
pub fn two(input: &str) -> Result<String, String> {
    Ok((1 + input
        .chars()
        .map(as_delta)
        .scan(0, |s, x| {
            *s += x;
            Some(*s)
        })
        .take_while(|&s| s >= 0)
        .count())
    .to_string())
}

/// Turns an input character into a floor delta.
fn as_delta(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}
