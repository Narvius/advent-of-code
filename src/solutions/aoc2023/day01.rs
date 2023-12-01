/// Find the total calibration score of all lines.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(input.lines().filter_map(calibration_score).sum())
}

/// Find the total calibration score of all lines, but including numbers written as text.
pub fn two(input: &str) -> crate::Result<i32> {
    Ok(input
        .lines()
        .filter_map(|line| calibration_score(&with_text_numbers(line)))
        .sum())
}

/// Calculates the calibration score for a string, as per the puzzle description.
fn calibration_score(s: &str) -> Option<i32> {
    let first = s.bytes().find(|b| b.is_ascii_digit())?;
    let last = s.bytes().rev().find(|b| b.is_ascii_digit())?;

    Some((10 * (first - b'0') + (last - b'0')) as i32)
}

/// For each number from 1 to 9 written in plain text (so `one`, `two`, `three`...), adds a
/// corresponding numerical digit into the string.
///
/// If numbers are overlapping, like for example `eightwo`, both are added.
fn with_text_numbers(line: &str) -> String {
    line.replace("one", "o1ne")
        .replace("two", "t2wo")
        .replace("three", "t3hree")
        .replace("four", "f4our")
        .replace("five", "f5ive")
        .replace("six", "s6ix")
        .replace("seven", "s7even")
        .replace("eight", "e8ight")
        .replace("nine", "n9ine")
}
