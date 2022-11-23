use std::collections::{HashMap, HashSet};

/// Sum the values at low points (incremented by 1).
pub fn one(input: &str) -> crate::Result<String> {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|s| s.as_bytes().iter().map(|b| b - b'0').collect())
        .collect();

    Ok(low_points(&map)
        .map(|(x, y)| map[y as usize][x as usize] as u32 + 1)
        .sum::<u32>()
        .to_string())
}

/// Find the three largest basins, and get the product of their sizes.
pub fn two(input: &str) -> crate::Result<i32> {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|s| s.as_bytes().iter().map(|b| b - b'0').collect())
        .collect();
    let mut basins = HashMap::new();
    for p in low_points(&map) {
        basins.insert(p, basin_size(&map, p));
    }

    let mut values: Vec<_> = basins.into_values().collect();
    values.sort();
    Ok(values
        .into_iter()
        .rev()
        .take(3)
        .reduce(|a, b| a * b)
        .ok_or("no values")?)
}

type Point = (i32, i32);

/// Returns the size of a basin for the given low point. May produce incorrect results for
/// arguments that are not low points.
fn basin_size(map: &[Vec<u8>], (x, y): Point) -> i32 {
    let mut basin = HashSet::new();
    let mut stack = vec![(x, y)];
    basin.insert((x, y));

    while let Some(p) = stack.pop() {
        for n in neighbours(map, p) {
            if !basin.contains(&n) && basin.contains(&flows_towards(map, n)) {
                basin.insert(n);
                stack.push(n);
            }
        }
    }

    basin.len() as i32
}

/// Returns the coordinate this coordinate "flows towards", ie. the lowest neighbour. Extreme
/// values (that is, 0 and 9) "flow towards" themselves.
fn flows_towards(map: &[Vec<u8>], (x, y): Point) -> Point {
    match map[y as usize][x as usize] {
        0 | 9 => (x, y),
        _ => neighbours(map, (x, y))
            .min_by_key(|&(x, y)| map[y as usize][x as usize])
            .unwrap_or((x, y)),
    }
}

/// Returns a list of all "low points" in the map, as defined by the problem statement.
fn low_points(map: &Vec<Vec<u8>>) -> impl Iterator<Item = Point> + '_ {
    product(0..map[0].len() as i32, 0..map.len() as i32).filter(move |&(x, y)| {
        match map.get(y as usize).and_then(|xs| xs.get(x as usize)) {
            Some(&v) => neighbours(map, (x, y)).all(|(x, y)| v < map[y as usize][x as usize]),
            _ => false,
        }
    })
}

/// Returns an iterator over the neighbours of coordinate. Works for invalid coordinates, but will
/// most likely return nothing.
fn neighbours(map: &[Vec<u8>], (x, y): Point) -> impl Iterator<Item = Point> + '_ {
    [(-1i32, 0), (0, -1i32), (1, 0), (0, 1)]
        .iter()
        .filter_map(move |&(dx, dy)| -> Option<Point> {
            let (x, y) = (x + dx, y + dy);
            map.get(y as usize).and_then(|xs| xs.get(x as usize))?;
            Some((x, y))
        })
}

/// Returns the carthesian product of two iterators.
fn product<A: Clone, B>(
    a: impl Iterator<Item = A>,
    b: impl Iterator<Item = B> + Clone,
) -> impl Iterator<Item = (A, B)> {
    a.flat_map(move |i| b.clone().map(move |j| (i.clone(), j)))
}
