/// Find the total caolories carried by the top elf.
pub fn one(input: &str) -> crate::Result<i32> {
    parse(input)?
        .into_iter()
        .max()
        .ok_or_else(|| "no result".into())
}

/// Find the total calories carried by the top three elves.
pub fn two(input: &str) -> crate::Result<i32> {
    let mut elves = parse(input)?;
    elves.sort_unstable_by_key(|e| i32::MAX - e);
    Ok(elves[0..3].iter().copied().sum())
}

/// Parses the input into the number of calories carried by each elf.
fn parse(input: &str) -> crate::Result<Vec<i32>> {
    let mut result = vec![0];
    for line in input.lines() {
        if line.is_empty() {
            result.push(0);
        } else if let Some(v) = result.last_mut() {
            *v += line.parse::<i32>()?;
        }
    }
    Ok(result)
}
