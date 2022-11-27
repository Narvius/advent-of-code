/// Count passwords that match the letter count policy.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(input
        .lines()
        .filter_map(|line| {
            let (p1, p2, c, code) = parse(line)?;
            let count = code.chars().filter(|&cc| cc == c).count();
            (p1..=p2).contains(&count).then_some(1)
        })
        .sum())
}

/// Count passwords that match the exclusive letter position policy.
pub fn two(input: &str) -> crate::Result<i32> {
    Ok(input
        .lines()
        .filter_map(|line| {
            let (p1, p2, c, code) = parse(line)?;
            ((code.chars().nth(p1 - 1) == Some(c)) ^ (code.chars().nth(p2 - 1) == Some(c)))
                .then_some(1)
        })
        .sum())
}

/// Parses the content of a "policy: password" line.
fn parse(line: &str) -> Option<(usize, usize, char, &str)> {
    let (nums, data) = line.split_once(' ')?;
    let (a, b) = nums.split_once('-')?;
    let (c, code) = data.split_once(": ")?;

    Some((a.parse().ok()?, b.parse().ok()?, c.chars().next()?, code))
}
