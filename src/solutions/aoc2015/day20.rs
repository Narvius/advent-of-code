/// Find the earliest house that gets enough presents. This is equivalent to finding the lowest
/// number that has a high enough sum of proper factors.
pub fn one(input: &str) -> crate::Result<String> {
    // I tried to do this a clever way via prime factorization or something, but this is fast
    // enough for how trivial it is to implement.

    let mut cache = vec![0; 1_000_000];
    let target = parse(input)?;

    for n in 1..cache.len() {
        for d in 1.. {
            if let Some(p) = cache.get_mut(n * d) {
                *p += 10 * n as i64;
            } else {
                break;
            }
        }
    }

    Ok((1..).find(|&i| cache[i] >= target).unwrap_or(0).to_string())
}

/// Find the earliest house that gets enough presents; but additional limitations no longer allow
/// it to be abstracted to a nice mathematical solution.
pub fn two(input: &str) -> crate::Result<String> {
    let mut cache = vec![0; 1_000_000];
    let target = parse(input)?;

    for n in 1..cache.len() {
        for d in 1..50 {
            if let Some(p) = cache.get_mut(n * d) {
                *p += 11 * n as i64;
            } else {
                break;
            }
        }
    }

    Ok((1..).find(|&i| cache[i] >= target).unwrap_or(0).to_string())
}

/// Parses the puzzle input into a target number.
fn parse(input: &str) -> Result<i64, String> {
    input.parse().map_err(|_| "failed to parse input".into())
}
