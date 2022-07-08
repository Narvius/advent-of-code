use std::{cmp::Ordering, collections::HashMap};

/// Find the correct Sue using exact matches for all values.
pub fn one(input: &str) -> crate::Result<usize> {
    let data = sue_data();
    for (n, ps) in parse(input) {
        if ps.into_iter().all(|(k, v)| data[k].0 == v) {
            return Ok(n);
        }
    }
    Err("failed to find a matching Sue".into())
}

/// Find the correct Sue using ranges for the appropriate values.
pub fn two(input: &str) -> crate::Result<usize> {
    let data = sue_data();
    for (n, ps) in parse(input) {
        if ps.into_iter().all(|(k, v)| v.cmp(&data[k].0) == data[k].1) {
            return Ok(n);
        }
    }
    Err("failed to find a matching Sue".into())
}

type Sue<'a> = (usize, [(&'a str, usize); 3]);

/// Parses the input into a sequence of Sues.
fn parse<'a>(input: &'a str) -> impl Iterator<Item = Sue> + 'a {
    input.lines().filter_map(|line| {
        let mut tokens = line.split(": ").flat_map(|s| s.split(", "));
        Some((
            tokens.next()?[4..].parse().ok()?,
            [
                (tokens.next()?, tokens.next()?.parse().ok()?),
                (tokens.next()?, tokens.next()?.parse().ok()?),
                (tokens.next()?, tokens.next()?.parse().ok()?),
            ],
        ))
    })
}

/// All the data we know about the correct Sue from the puzzle description.
fn sue_data() -> HashMap<&'static str, (usize, Ordering)> {
    HashMap::from([
        ("children", (3, Ordering::Equal)),
        ("cats", (7, Ordering::Greater)),
        ("samoyeds", (2, Ordering::Equal)),
        ("pomeranians", (3, Ordering::Less)),
        ("akitas", (0, Ordering::Equal)),
        ("vizslas", (0, Ordering::Equal)),
        ("goldfish", (5, Ordering::Less)),
        ("trees", (3, Ordering::Greater)),
        ("cars", (2, Ordering::Equal)),
        ("perfumes", (1, Ordering::Equal)),
    ])
}
