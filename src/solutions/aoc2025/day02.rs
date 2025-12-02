/// Find the sum of all numbers in the provided input ranges where the first and second half of the
/// number are the same.
pub fn one(input: &str) -> crate::Result<usize> {
    let mut sum = 0;
    for (lo, hi) in parse(input) {
        for i in lo..=hi {
            let s = i.to_string();
            let (a, b) = s.split_at(s.len() / 2);
            if a == b {
                sum += i;
            }
        }
    }
    Ok(sum)
}

/// Find the sum of all numbers in the provided input ranges which are made of the same cluster of
/// digits repeating at least twice. For example, 12121212 and 5454 both count, but 123124 doesn't.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut sum = 0;
    for (lo, hi) in parse(input) {
        for i in lo..=hi {
            let s = i.to_string();
            for d in (1..=s.len() / 2).rev() {
                if s.len() % d == 0
                    && s.as_bytes()
                        .chunks_exact(d)
                        .all(|c| c == &s.as_bytes()[0..d])
                {
                    sum += i;
                    break;
                }
            }
        }
    }
    Ok(sum)
}

/// Parses the puzzle input into a series of ranges (containing both the numerical and textual
/// representation).
fn parse(input: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    input.trim().split(',').filter_map(|line| {
        let (lo, hi) = line.split_once('-')?;
        Some((lo.parse().ok()?, hi.parse().ok()?))
    })
}
