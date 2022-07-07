/// Find the total score of all groups; the score of a group is its depth.
pub fn one(input: &str) -> crate::Result<String> {
    let (mut score, mut depth) = (0, 0);
    process(input, |p| match p {
        ('{', false) => {
            depth += 1;
            score += depth;
        }
        ('}', false) => depth -= 1,
        _ => {}
    });
    Ok(score.to_string())
}

/// Count the number of unescaped garbage characters.
pub fn two(input: &str) -> crate::Result<String> {
    let mut count = 0;
    process(input, |(_, garbage)| {
        if garbage {
            count += 1;
        }
    });
    Ok(count.to_string())
}

/// Processes the input stream according to puzzle rules. Calls the provided closure with every
/// character that isn't a control character for the skip/garbage tracking.
fn process(input: &str, mut f: impl FnMut((char, bool))) {
    let (mut garbage, mut skip) = (false, false);

    for c in input.chars() {
        if skip {
            skip = false;
            continue;
        }

        match (c, garbage) {
            ('<', false) => garbage = true,
            ('!', true) => skip = true,
            ('>', true) => garbage = false,
            _ => f((c, garbage)),
        }
    }
}
