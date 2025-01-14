use std::collections::{HashMap, HashSet};

/// Count the number of positions with antinodes in them.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(count_antinodes(input, false))
}

/// Count the number of positions with antinodes in them, considering harmonic resonances.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(count_antinodes(input, true))
}

/// Counts the number of positions with antinodes in them. That is, for any given pair of antennas
/// with the same frequency, that's the positions exactly one distance away in either direction;
/// unless we consider harmonic resonances, in which case it's all positions _any_ multiple of
/// distance away from either antenna.
fn count_antinodes(input: &str, harmonics: bool) -> usize {
    let mut antinodes = HashSet::new();
    let ((w, h), frequencies) = parse(input);
    let in_bounds = |(x, y): &(i32, i32)| (0..w).contains(x) && (0..h).contains(y);

    for points in frequencies.into_values() {
        for (i, &(x1, y1)) in points.iter().enumerate() {
            for &(x2, y2) in points[(i + 1)..].iter() {
                let (dx, dy) = (x2 - x1, y2 - y1);
                for (start, dir) in [((x1, y1), (-dx, -dy)), ((x2, y2), (dx, dy))] {
                    antinodes.extend(resonances(start, dir, harmonics).take_while(in_bounds));
                }
            }
        }
    }
    antinodes.len()
}

/// Returns all coordinates resonant with `(x, y)` in direction `(dx, dy)`. That is exactly the
/// point `(x + dx, y + dy)`, unless we also consider `harmonics`; in which case it is the
/// half-line `(x + i * dx, y + i * dy)` starting at `i` = 0 and counting up.
fn resonances(
    (x, y): (i32, i32),
    (dx, dy): (i32, i32),
    harmonics: bool,
) -> impl Iterator<Item = (i32, i32)> {
    if harmonics { 0..=i32::MAX } else { 1..=1 }.map(move |i| (x + i * dx, y + i * dy))
}

type Frequencies = HashMap<u8, Vec<(i32, i32)>>;

/// Parses the puzzle input, returning the grid width and height, and the positions of all antennas
/// grouped by their frequency.
fn parse(input: &str) -> ((i32, i32), Frequencies) {
    let (mut map, mut max_w) = (Frequencies::new(), 0);
    for (y, line) in input.lines().enumerate() {
        max_w = line.len().max(max_w);
        for (x, c) in line.bytes().enumerate() {
            if c != b'.' {
                map.entry(c).or_default().push((x as i32, y as i32));
            }
        }
    }
    ((max_w as i32, input.lines().count() as i32), map)
}
