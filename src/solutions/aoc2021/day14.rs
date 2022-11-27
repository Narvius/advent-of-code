use std::collections::HashMap;

/// Expand the polymer 10 times, get the difference in occurences of the most common and the least
/// common byte.
pub fn one(input: &str) -> crate::Result<i64> {
    let (polymer, pairs) = parse(input)?;
    score_after_steps(polymer, &pairs, 10).ok_or_else(|| "no result".into())
}

/// Expand the polymer 40 times, get the difference in occurences of the most common and the least
/// common byte.
pub fn two(input: &str) -> crate::Result<i64> {
    let (polymer, pairs) = parse(input)?;
    score_after_steps(polymer, &pairs, 40).ok_or_else(|| "no result".into())
}

type Rules = HashMap<(u8, u8), u8>;

/// Expands the `polymer` by the given amount of `steps`, using `rules`; then returns the difference
/// in quantity of the most common byte and the least common byte in the output.
fn score_after_steps(polymer: &[u8], rules: &Rules, steps: usize) -> Option<i64> {
    let mut counts = HashMap::new();
    let mut pairs = HashMap::new();

    // Count the actual bytes in the input polymer.
    for &c in polymer {
        *counts.entry(c).or_insert(0) += 1;
    }

    // Find pairs that are in the input polymer.
    for w in polymer.windows(2) {
        *pairs.entry((w[0], w[1])).or_insert(0) += 1;
    }

    // Iteratively construct new pair lists until all steps are done.
    for _ in 0..steps {
        pairs = {
            let mut temp = HashMap::new();
            // There are `v` of this pair. That means `v` of a new letter will spawn, and `v`
            // of two new pairs will be formed. Count all of those.
            for ((c1, c2), v) in pairs {
                let new = *rules.get(&(c1, c2))?;
                *counts.entry(new).or_insert(0) += v;
                *temp.entry((c1, new)).or_insert(0) += v;
                *temp.entry((new, c2)).or_insert(0) += v;
            }
            temp
        }
    }

    Some(counts.values().max()? - counts.values().min()?)
}

/// Parses the puzzle input into an input polymer and an expansion rule map.
fn parse(input: &str) -> crate::Result<(&[u8], Rules)> {
    let mut lines = input.lines();
    let polymer = lines.next().ok_or("insufficient input")?.as_bytes();
    let mut pairs = HashMap::new();

    for line in lines {
        if line.contains(" -> ") {
            let (pair, new) = line.split_once(" -> ").ok_or("unexpected input")?;
            pairs.insert((pair.as_bytes()[0], pair.as_bytes()[1]), new.as_bytes()[0]);
        }
    }

    Ok((polymer, pairs))
}
