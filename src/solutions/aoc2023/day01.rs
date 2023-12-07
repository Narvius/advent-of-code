/// Find the total calibration score of all lines.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(input.lines().filter_map(calibration_score).sum())
}

/// Find the total calibration score of all lines, but including numbers written as text.
pub fn two(input: &str) -> crate::Result<i32> {
    Ok(input.lines().filter_map(calibration_score_with_text).sum())
}

/// Calculates the calibration score for a string, as per the puzzle description.
fn calibration_score(s: &str) -> Option<i32> {
    let first = s.bytes().find(|b| b.is_ascii_digit())?;
    let last = s.bytes().rev().find(|b| b.is_ascii_digit())?;

    Some((10 * (first - b'0') + (last - b'0')) as i32)
}

/// Calculates the calibration score for a string, as per the puzzle description; but also
/// considers numbers written in plain text while doing so.
///
/// Originally, I wrote a version that was less code (consisting of only a bunch of replaces), but
/// this version if faster, running in less than a millisecond for me (...when I compile as
/// --release, anyway).
fn calibration_score_with_text(line: &str) -> Option<i32> {
    fn scan(s: &str, range: impl Iterator<Item = usize>) -> Option<i32> {
        for i in range {
            if s.as_bytes()[i].is_ascii_digit() {
                return Some((s.as_bytes()[i] - b'0') as i32);
            } else if s[i..].starts_with("one") {
                return Some(1);
            } else if s[i..].starts_with("two") {
                return Some(2);
            } else if s[i..].starts_with("three") {
                return Some(3);
            } else if s[i..].starts_with("four") {
                return Some(4);
            } else if s[i..].starts_with("five") {
                return Some(5);
            } else if s[i..].starts_with("six") {
                return Some(6);
            } else if s[i..].starts_with("seven") {
                return Some(7);
            } else if s[i..].starts_with("eight") {
                return Some(8);
            } else if s[i..].starts_with("nine") {
                return Some(9);
            }
        }
        None
    }

    let first = scan(line, 0..line.len())?;
    let last = scan(line, (0..line.len()).rev())?;

    Some(10 * first + last)
}
