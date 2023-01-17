use std::collections::HashMap;

/// Find the manhattan distance to the nearest intersection to origin.
pub fn one(input: &str) -> crate::Result<i32> {
    best_intersection_by(input, |((x, y), _)| x.abs() + y.abs())
}

/// Find the lowest number of cumulative steps taken to reach an intersection.
pub fn two(input: &str) -> crate::Result<i32> {
    best_intersection_by(input, |(_, steps)| steps)
}

/// Finds a list of all intersections of two wires from `input`, calls `key` for each
/// intersections, and returns the lowest value calculated.
fn best_intersection_by(input: &str, key: fn(((i32, i32), i32)) -> i32) -> crate::Result<i32> {
    let mut maps = input.lines().map(run_wire);

    let a = maps.next().ok_or("insufficient input")??;
    let b = maps.next().ok_or("insufficient input")??;

    a.into_iter()
        .filter_map(move |(p, steps)| b.get(&p).map(|v| (p, steps + v)))
        .map(key)
        .min()
        .ok_or_else(|| "no result".into())
}

/// Converts line instructions from the input into a map of `position reached => steps taken`
/// pairs.
fn run_wire(line: &str) -> crate::Result<HashMap<(i32, i32), i32>> {
    let mut map = HashMap::new();
    let (mut x, mut y, mut steps) = (0, 0, 0);
    for instruction in line.trim().split(',') {
        let (dx, dy) = match instruction.as_bytes()[0] {
            b'L' => (-1, 0),
            b'U' => (0, -1),
            b'R' => (1, 0),
            b'D' => (0, 1),
            c => return Err(format!("invalid direction '{c}'").into()),
        };
        let stride: i32 = instruction[1..].parse()?;

        for _ in 0..stride {
            (x, y, steps) = (x + dx, y + dy, steps + 1);
            map.entry((x, y)).or_insert(steps);
        }
    }
    Ok(map)
}
