use std::collections::{HashMap, HashSet};

/// Find the largest finite area after floodfilling from all the input cells.
pub fn one(input: &str) -> crate::Result<i32> {
    const SIZE: i32 = 400;

    let mut cells: Vec<Option<u8>> = vec![None; (SIZE * SIZE) as usize];
    let mut counts: HashMap<usize, i32> = HashMap::new();
    let mut heads: Vec<_> = parse(input).iter().copied().enumerate().collect();

    while !heads.is_empty() {
        let mut new_heads = vec![];
        let mut fresh = HashSet::new();
        for (id, (x, y)) in heads {
            if !(0..SIZE).contains(&x) || !(0..SIZE).contains(&y) {
                // Went beyond the map area we're interested in. We assume that any
                // areas that reach the edge are infinite, so we make the associated
                // value arbitrarily low as an "infinite" marker.
                *counts.entry(id).or_default() -= SIZE * SIZE;
            } else {
                let cell = &mut cells[(y * SIZE + x) as usize];
                match *cell {
                    None => {
                        // Cell is free. Claim it, and mark it as "fresh", meaning it can
                        // still be lost if another area lies claim to it during this
                        // run of the loop.
                        *cell = Some(id as u8);
                        *counts.entry(id).or_default() += 1;
                        fresh.insert((x, y));

                        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                            new_heads.push((id, (x + dx, y + dy)));
                        }
                    }
                    Some(other_id) => {
                        // If a cell was claimed during this loop, and this is a
                        // conflicting claim, then mark the cell with an arbitrary large
                        // value.
                        if id as u8 != other_id && fresh.contains(&(x, y)) {
                            *cell = Some(u8::MAX);
                            *counts.entry(other_id as usize).or_default() -= 1;
                        }
                    }
                }
            }
        }
        heads = new_heads;
    }

    counts
        .into_values()
        .max()
        .ok_or_else(|| "no finite area".into())
}

/// Count the number of cells that have a total Manhattan distance to all input cells of
/// less than 10000.
pub fn two(input: &str) -> crate::Result<usize> {
    const SIZE: usize = 700;
    const OFFSET: i32 = -100;

    // Precalculate distances along each axis independently.
    let points = parse(input);
    let (x_distances, y_distances) = {
        let (mut xs, mut ys) = (vec![0; SIZE], vec![0; SIZE]);
        for i in 0..SIZE {
            for &(px, py) in &points {
                xs[i] += px.abs_diff(i as i32 + OFFSET);
                ys[i] += py.abs_diff(i as i32 + OFFSET);
            }
        }
        (xs, ys)
    };

    // Check all combinations of precalculated values.
    Ok((0..SIZE * SIZE)
        .filter(|&i| x_distances[i % SIZE] + y_distances[i / SIZE] < 10000)
        .count())
}

/// Parses the puzzle input.
fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .filter_map(|line| {
            let (x, y) = line.split_once(", ")?;
            Some((x.parse().ok()?, y.parse().ok()?))
        })
        .collect()
}
