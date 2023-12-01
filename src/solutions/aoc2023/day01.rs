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
/// numerical digit into the string.
///
/// If numbers are overlapping, like for example `eightwo`, both are added.
fn with_text_numbers(line: &str) -> String {
    const S: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    const T: [&str; 9] = [
        "o1ne", "t2wo", "th3ree", "fo4ur", "fi5ve", "s6ix", "se7ven", "ei8ght", "ni9ne",
    ];

    let mut line = line.to_string();
    for i in 0..9 {
        line = line.replace(S[i], T[i]);
    }
    line
}
