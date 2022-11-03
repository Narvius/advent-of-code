use std::collections::HashSet;

/// Find the sum of the indices of all living cells after 20 iterations.
pub fn one(input: &str) -> crate::Result<i64> {
    let (rules, (mut l, mut h), mut curr) = parse(input).ok_or("failed parse")?;

    for _ in 0..20 {
        (l, h) = (l - 2, h + 2);
        curr = (l..h).filter(|&i| lives_in_next(&curr, rules, i)).collect();
    }

    Ok(curr.into_iter().sum())
}

/// Find the sum of the indices of all living cells after fifty billion iterations.
pub fn two(input: &str) -> crate::Result<i64> {
    let (rules, (mut l, mut h), mut curr) = parse(input).ok_or("failed parse")?;

    let (mut iterations, mut diff, mut run) = (0, 0, 0);
    let mut sum = curr.iter().copied().sum::<i64>();

    // We assume that the growth of the sum will stabilize at some point. Once it does, we
    // simply extrapolate that stable growth for the remaining amount of iterations.
    while run < 5 {
        iterations += 1;
        (l, h) = (l - 2, h + 2);
        curr = (l..h).filter(|&i| lives_in_next(&curr, rules, i)).collect();

        let next_sum = curr.iter().copied().sum();
        let next_diff = next_sum - sum;
        sum = next_sum;
        if next_diff == diff {
            run += 1;
        } else {
            run = 0;
            diff = next_diff;
        }
    }

    Ok(sum + (50_000_000_000 - iterations) * diff)
}

/// Parses the mask (ruleset), initial bounds and initial state from puzzle input.
///
/// Note that there are exactly 32 rules making up the ruleset. That means we can store them
/// as one `u32`, where each digit refers to the outcome for a specific rule. Then we simply
/// read the five cells as binary digits to find the index into the ruleset number.
///
/// Bounds are are the lowest and highest number that *could* be contained in the state. This
/// expands by two cells either way each iteration, because that's the "range" of the ruleset.
fn parse(input: &str) -> Option<(u32, (i64, i64), HashSet<i64>)> {
    let mut lines = input.lines();

    let initial = lines.next()?.split_once(": ")?.1;
    lines.next();

    let mut mask = 0u32;
    for line in lines {
        let (pat, next) = line.split_once(" => ")?;
        if next == "#" {
            mask |= 1 << as_index(pat.chars().map(|c| c == '#'));
        }
    }

    let bounds = (0, initial.len() as i64);
    let initial = initial
        .char_indices()
        .filter(|(_, b)| *b == '#')
        .map(|(i, _)| i as i64)
        .collect();

    Some((mask, bounds, initial))
}

/// Checks if a cell lives in the next iteration, given a particular ruleset.
fn lives_in_next(prev: &HashSet<i64>, rules: u32, cell: i64) -> bool {
    let index = as_index((-2..=2).map(|i| prev.contains(&(cell + i))));
    (rules & (1 << index)) > 0
}

/// Converts a series of bools into the corresponding binary number.
fn as_index(bs: impl IntoIterator<Item = bool>) -> u32 {
    bs.into_iter()
        .enumerate()
        .filter(|(_, b)| *b)
        .fold(0, |acc, (i, _)| acc | (1 << i))
}
