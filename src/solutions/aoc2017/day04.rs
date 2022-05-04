use std::collections::HashSet;

/// Count the number of lines in the input that do not have a repeating word.
pub fn one(input: &str) -> Result<String, String> {
    count_non_repeats(input, |s| s.to_string())
}

/// Count the number of lines in the input that do not have any two words that are
/// anagrams of each other.
pub fn two(input: &str) -> Result<String, String> {
    count_non_repeats(input, |s| {
        let mut s = s.to_string();
        // SAFETY: All strings are ASCII, so sorting bytes will result in a valid string.
        unsafe {
            s.as_bytes_mut().sort();
        }
        s
    })
}

/// Counts the number of lines in the input that do not have a repeating `f(word)`.
fn count_non_repeats(input: &str, f: fn(&str) -> String) -> Result<String, String> {
    let mut result = 0;

    'line: for line in input.lines() {
        let mut set = HashSet::new();
        for token in line.split_whitespace() {
            if !set.insert(f(token)) {
                continue 'line;
            }
        }
        result += 1;
    }

    Ok(result.to_string())
}
