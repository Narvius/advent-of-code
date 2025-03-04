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
type Cache<'a> = HashMap<&'a str, usize>;

/// Returns the answers to both parts.
///
/// Just recursive depth-first search of the space with a cache to speed up processing of repeating
/// sub-patterns.
fn solve(input: &str) -> (usize, usize) {
    let (towels, patterns) = parse(input);
    let mut cache = HashMap::from([("", 1)]);

    patterns
        .map(|pattern| count_arrangements(&towels, pattern, &mut cache))
        .filter(|&ways| ways > 0)
        .fold((0, 0), |(count, total), ways| (count + 1, total + ways))
}

/// Counts how many ways `pattern` is formable.
fn count_arrangements<'a>(towels: &[&str], pattern: &'a str, cache: &mut Cache<'a>) -> usize {
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
fn parse(input: &str) -> (Vec<&str>, impl Iterator<Item = &str> + '_) {
    let (towels, patterns) = input.split_once("\r\n\r\n").expect("two sections in input");
    (towels.split(", ").collect(), patterns.lines())
}
