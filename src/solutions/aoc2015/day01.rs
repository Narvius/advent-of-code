/// Find the final floor the elevator stops at.
pub fn one(input: &str) -> crate::Result<String> {
    Ok(parse(input).sum::<i32>().to_string())
}

/// Find the number of steps after which the elevator first goes into the basement.
pub fn two(input: &str) -> crate::Result<String> {
    let (mut floor, mut count) = (0, 0);
    for c in parse(input) {
        (floor, count) = (floor + c, count + 1);
        if floor < 0 {
            return Ok(count.to_string());
        }
    }
    Err("never reached the basement".into())
}

/// Turns an input character into a floor delta.
fn parse(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    })
}
