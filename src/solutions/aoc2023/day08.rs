use std::collections::HashMap;

/// Find the number of steps required to reach "ZZZ" from "AAA".
pub fn one(input: &str) -> crate::Result<usize> {
    let (steps, map) = parse(input).ok_or("failed parse")?;
    Ok(steps_required(steps, &map, "AAA", "ZZZ"))
}

/// Find the number of steps required such that starting at all points ending with "A", you
/// simultaneously arrive at points ending with "Z".
pub fn two(input: &str) -> crate::Result<usize> {
    let (steps, map) = parse(input).ok_or("failed parse")?;

    map.keys()
        .filter(|k| k.ends_with('A'))
        .map(|location| steps_required(steps, &map, location, "Z"))
        .reduce(crate::common::lcm)
        .ok_or("no result".into())
}

/// Returns the number of steps required to reach a node ending with `end` from the node named
/// `start`; using `steps` as instructions on the provided `map`.
fn steps_required(steps: &str, map: &Map, start: &str, end: &str) -> usize {
    let mut location = start;
    for (i, d) in std::iter::repeat(steps.bytes()).flatten().enumerate() {
        location = map[location][usize::from(d == b'R')];
        if location.ends_with(end) {
            return i + 1;
        }
    }
    unreachable!()
}

type Map<'a> = HashMap<&'a str, [&'a str; 2]>;

/// Parses the puzzle input into the step instructions and a map.
pub fn parse(input: &str) -> Option<(&str, Map)> {
    let (steps, map) = input.split_once("\r\n")?;
    let map = map.lines().filter_map(|line| {
        let (key, rest) = line.split_once(" = (")?;
        let val: [&str; 2] = rest.trim_end_matches(')').split_once(", ")?.into();
        Some((key, val))
    });

    Some((steps, map.collect()))
}
