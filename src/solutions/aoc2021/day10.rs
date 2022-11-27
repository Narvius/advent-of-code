/// Find the sum of all syntax error scores. That is, for every line with mismatched closing
/// characters, count that mismatched closing character as an arbitrary score given by the puzzle.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(input
        .lines()
        .map(|line| match incomplete_tail(line) {
            Err(b'(') => 3,
            Err(b'[') => 57,
            Err(b'{') => 1197,
            Err(b'<') => 25137,
            _ => 0,
        })
        .sum())
}

/// Find the middle autocompletion score. That is, for every line that is missing additional
/// closing characters, find those missing closing characters, then derive a score from them, then
/// choose the median of the set (which is guaranteed to have an uneven number of entries).
pub fn two(input: &str) -> crate::Result<i64> {
    const DELIMS: [u8; 5] = [b'\0', b'(', b'[', b'{', b'<'];

    let mut scores: Vec<_> = input
        .lines()
        .filter_map(|s| incomplete_tail(s).ok())
        .map(|v| {
            v.into_iter().rev().fold(0i64, |a, b| {
                5 * a + DELIMS.iter().position(|&c| b == c).unwrap_or(0) as i64
            })
        })
        .collect();

    scores.sort();
    Ok(scores[scores.len() / 2])
}

/// Syntax checks the line. Returns `Err(mismatched delimiter)` if there was one,
/// `Ok(list of unclosed delimiters)` otherwise.
fn incomplete_tail(s: &str) -> Result<Vec<u8>, u8> {
    let mut stack = vec![];

    for &byte in s.as_bytes() {
        match byte {
            b'(' | b'[' | b'{' | b'<' => stack.push(byte),
            b')' if stack.pop() != Some(b'(') => return Err(b'('),
            b']' if stack.pop() != Some(b'[') => return Err(b'['),
            b'}' if stack.pop() != Some(b'{') => return Err(b'{'),
            b'>' if stack.pop() != Some(b'<') => return Err(b'<'),
            _ => {}
        }
    }

    Ok(stack)
}
