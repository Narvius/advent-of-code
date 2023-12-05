use std::ops::Range;

/// Map all the input seeds to final locations, and find the lowest location.
pub fn one(input: &str) -> crate::Result<i64> {
    let (mut seeds, mappings) = parse(input).ok_or("parse failed")?;

    for mapping in mappings {
        for seed in &mut seeds {
            let offset = mapping
                .iter()
                .find_map(|(r, o)| r.contains(&*seed).then_some(*o))
                .unwrap_or(0);

            *seed += offset;
        }
    }

    seeds.into_iter().min().ok_or("no result".into())
}

/// Like `one`, but treat the input seeds as ranges; every two numbers are a (start of range,
/// length of range) pair.
pub fn two(input: &str) -> crate::Result<i64> {
    let (seeds, mappings) = parse(input).ok_or("parse failed")?;
    let mut seed_ranges: Vec<_> = seeds.chunks(2).map(|c| (c[0], c[0] + c[1])).collect();

    for mapping in mappings {
        let mut new_ranges = vec![];

        while let Some(range) = seed_ranges.pop() {
            let overlap = mapping
                .iter()
                .find_map(|(r, o)| interval_overlap(range, (r.start, r.end)).map(|v| (v, o)));

            if let Some(((s, e), offset)) = overlap {
                new_ranges.push((s + offset, e + offset));

                if s > range.0 {
                    seed_ranges.push((range.0, s));
                }
                if e < range.1 {
                    seed_ranges.push((e, range.1));
                }
            } else {
                new_ranges.push(range);
            }
        }

        seed_ranges = new_ranges;
    }

    seed_ranges
        .into_iter()
        .map(|r| r.0)
        .min()
        .ok_or("no result".into())
}

/// Given two half-open intervals, returns the intersection, if it exists.
fn interval_overlap((s1, e1): (i64, i64), (s2, e2): (i64, i64)) -> Option<(i64, i64)> {
    let s = s1.max(s2);
    let e = e1.min(e2);

    (s < e).then_some((s, e))
}

type Mappings = Vec<(Range<i64>, i64)>;
type Seeds = Vec<i64>;

/// Parses the puzzle input into a list of seeds and list mapping tables.
fn parse(input: &str) -> Option<(Seeds, Vec<Mappings>)> {
    let mut chunks = input.split("\r\n\r\n");
    let (_, seeds) = chunks.next()?.split_once(": ")?;

    let seeds = seeds
        .split_whitespace()
        .filter_map(|n| n.parse::<i64>().ok());
    let mappings = chunks.map(|chunk| {
        chunk
            .lines()
            .skip(1)
            .filter_map(|line| {
                let mut tokens = line
                    .split_whitespace()
                    .filter_map(|n| n.parse::<i64>().ok());
                let destination = tokens.next()?;
                let source = tokens.next()?;
                let length = tokens.next()?;

                Some((source..source + length, destination - source))
            })
            .collect()
    });

    Some((seeds.collect(), mappings.collect()))
}
