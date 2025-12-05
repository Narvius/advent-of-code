/// Count the number of fresh ingredients by comparing their IDs to the fresh ingredient ranges.
pub fn one(input: &str) -> crate::Result<usize> {
    let (ranges, numbers) = parse(input);

    Ok(numbers
        .filter(|n| ranges.iter().any(|(lo, hi)| lo <= n && n <= hi))
        .count())
}

pub fn two(input: &str) -> crate::Result<u64> {
    let (mut ranges, _) = parse(input);

    // Eliminate overlaps by fusing overlapping ranges. Since ranges get removed during this, we're
    // using indices instead of iterators; only ever positions larger than `i` get removed though,
    // so technically it would be fine, we just can't prove it to the compiler.
    let mut i = 0;
    while i < ranges.len() {
        while let Some(j) = (i + 1..ranges.len()).find(|&j| overlap(ranges[i], ranges[j]).is_some())
        {
            let (lo, hi) = ranges.swap_remove(j);
            ranges[i] = (lo.min(ranges[i].0), hi.max(ranges[i].1));
        }

        i += 1;
    }

    Ok(ranges.into_iter().map(|(lo, hi)| hi - lo + 1).sum())
}

/// Finds the overlap between two ranges, if any.
fn overlap((alo, ahi): (u64, u64), (blo, bhi): (u64, u64)) -> Option<(u64, u64)> {
    let lo = u64::max(alo, blo);
    let hi = u64::min(ahi, bhi);
    (lo <= hi).then_some((lo, hi))
}

/// Parses the puzzle input into a list of fresh ingredient ID ranges, and a list of actual
/// ingredient IDs.
fn parse(input: &str) -> (Vec<(u64, u64)>, impl Iterator<Item = u64> + '_) {
    let (ranges, numbers) = input.split_once("\r\n\r\n").unwrap();

    (
        ranges
            .lines()
            .filter_map(|line| {
                let (lo, hi) = line.split_once('-')?;
                Some((lo.parse::<u64>().ok()?, hi.parse::<u64>().ok()?))
            })
            .collect(),
        numbers.lines().filter_map(|line| line.parse::<u64>().ok()),
    )
}
