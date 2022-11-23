use std::collections::HashMap;

/// Find all points where at least two horizontal/vertical lines overlap.
pub fn one(input: &str) -> crate::Result<usize> {
    part(input, false)
}

/// Find all points where at least two lines overlap.
pub fn two(input: &str) -> crate::Result<usize> {
    part(input, true)
}

/// Shared solution code for parts 1 and 2. `include_all` decides whether all lines (`true`) or only
/// horizontal/vertical lines (`false`) from the input should be used.
fn part(input: &str, include_all: bool) -> crate::Result<usize> {
    let lines = input
        .lines()
        .filter_map(parse_line)
        .filter(|&((x1, y1), (x2, y2))| include_all || x1 == x2 || y1 == y2);

    let mut map = HashMap::new();
    for p in lines.flat_map(points) {
        *map.entry(p).or_insert(0) += 1;
    }

    Ok(map.into_values().filter(|&v| v >= 2).count())
}

/// Given a line from the input, produces a pair of the line end points describes by it.
fn parse_line(line: &str) -> Option<((i32, i32), (i32, i32))> {
    let (from, to) = line.split_once(" -> ")?;
    let ((x1, y1), (x2, y2)) = (from.split_once(',')?, to.split_once(',')?);
    Some((
        (x1.parse().ok()?, y1.parse().ok()?),
        (x2.parse().ok()?, y2.parse().ok()?),
    ))
}

/// Given line endpoints, produces all points on that line. Only produces correct output for lines
/// with a slope that is divisible by 45 degrees.
fn points(((x1, y1), (x2, y2)): ((i32, i32), (i32, i32))) -> impl Iterator<Item = (i32, i32)> {
    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();
    let len = (x2 - x1).abs().max((y2 - y1).abs());
    (0..=len).map(move |n| (x1 + n * dx, y1 + n * dy))
}
