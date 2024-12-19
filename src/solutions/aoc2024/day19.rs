use std::collections::HashMap;

/// Find the number of desired arrangements that can actually be formed.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(solve(input).0)
}

/// Find the number of ways in which any desired arrangement can be formed.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(solve(input).1)
}

/// Memoization cache for `count_arrangements`.
type Cache<'a> = HashMap<&'a [u8], usize>;

/// Returns the answers to both parts.
///
/// Just recursive depth-first search of the space with a cache to speed up processing of repeating
/// sub-patterns.
fn solve(input: &str) -> (usize, usize) {
    let (towels, patterns) = parse(input);
    let mut cache: Cache = HashMap::new();
    cache.insert(&[], 1);

    patterns
        .map(|pattern| count_arrangements(&towels, pattern, &mut cache))
        .filter(|&ways| ways > 0)
        .fold((0, 0), |(count, total), ways| (count + 1, total + ways))
}

/// Counts how many ways `pattern` is formable.
fn count_arrangements<'a>(towels: &[&[u8]], pattern: &'a [u8], cache: &mut Cache<'a>) -> usize {
    if let Some(result) = cache.get(pattern) {
        return *result;
    }

    let count = towels
        .iter()
        .filter_map(|&towel| pattern.strip_prefix(towel))
        .map(|pattern| count_arrangements(towels, pattern, cache))
        .sum();

    cache.insert(pattern, count);
    count
}

/// Parses the puzzle input into a list of towels (prefixes) and sequence of arrangements.
fn parse(input: &str) -> (Vec<&[u8]>, impl Iterator<Item = &[u8]> + '_) {
    let (towels, patterns) = input.split_once("\r\n\r\n").expect("two sections in input");
    let towels: Vec<_> = towels.split(", ").map(|towel| towel.as_bytes()).collect();
    (towels, patterns.lines().map(|pattern| pattern.as_bytes()))
}
