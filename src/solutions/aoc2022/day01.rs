/// Find the total caolories carried by the top elf.
pub fn one(input: &str) -> crate::Result<i32> {
    parse(input).max().ok_or_else(|| "no result".into())
}

/// Find the total calories carried by the top three elves.
pub fn two(input: &str) -> crate::Result<i32> {
    let mut elves: Vec<i32> = parse(input).collect();
    elves.sort_unstable_by_key(|e| i32::MAX - e);
    Ok(elves[0..3].iter().copied().sum())
}

/// Parses the puzzle input into a list of the calories carried by each elf.
fn parse(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .split("\n\n")
        .map(|chunk| chunk.lines().filter_map(|n| n.parse::<i32>().ok()).sum())
}
