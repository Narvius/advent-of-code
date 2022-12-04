/// Count number of pairs that fully overlap.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(parse(input)
        .filter(|&(r1, r2)| {
            // An overlap is full if it exists and matches r1 or r2.
            overlap((r1, r2))
                .map(|r| [r1, r2].contains(&r))
                .unwrap_or(false)
        })
        .count())
}

/// Count number of pairs that overlap at all.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(parse(input).filter_map(overlap).count())
}

/// Finds the overlap between two ranges, if any.
fn overlap(((alo, ahi), (blo, bhi)): ((i32, i32), (i32, i32))) -> Option<(i32, i32)> {
    let lo = i32::max(alo, blo);
    let hi = i32::min(ahi, bhi);
    (lo <= hi).then_some((lo, hi))
}

/// Parses the puzzle input into pairs of ranges.
fn parse(input: &str) -> impl Iterator<Item = ((i32, i32), (i32, i32))> + '_ {
    input.lines().filter_map(|line| {
        let mut v = line
            .split(&[',', '-'])
            .filter_map(|token| token.parse::<i32>().ok());

        Some(((v.next()?, v.next()?), (v.next()?, v.next()?)))
    })
}
