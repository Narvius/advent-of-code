use std::collections::HashMap;

/// Entering the map from the top left, horizontally, find the energize count.
pub fn one(input: &str) -> crate::Result<usize> {
    let map: Vec<_> = input.lines().map(str::as_bytes).collect();
    Ok(energize(&map, ((0, 0), (1, 0))))
}

/// Try all possible ways to enter the map, and find the best energize count.
pub fn two(input: &str) -> crate::Result<usize> {
    let map: Vec<_> = input.lines().map(str::as_bytes).collect();
    let (width, height) = (map[0].len(), map.len());
    let mut best = 0;

    for y in 0..height {
        best = best.max(energize(&map, ((0, y as i32), (1, 0))));
        best = best.max(energize(&map, ((width as i32 - 1, y as i32), (-1, 0))));
    }

    for x in 0..width {
        best = best.max(energize(&map, ((x as i32, 0), (0, 1))));
        best = best.max(energize(&map, ((x as i32, height as i32 - 1), (0, -1))));
    }

    Ok(best)
}

/// Find how many tiles on the map would be energized for the given start position and velocity.
fn energize(map: &[&[u8]], start: ((i32, i32), (i32, i32))) -> usize {
    let mut seen: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    let mut rays = vec![start];

    while let Some(((mut x, mut y), (mut dx, mut dy))) = rays.pop() {
        while (0..map[0].len() as i32).contains(&x) && (0..map.len() as i32).contains(&y) {
            let entry = seen.entry((x, y)).or_default();
            if entry.contains(&(dx, dy)) {
                break;
            } else {
                entry.push((dx, dy));
            }

            match (map[y as usize][x as usize], dx, dy) {
                (b'\\', _, _) => (dx, dy) = (dy, dx),
                (b'/', _, _) => (dx, dy) = (-dy, -dx),
                (b'|', _, 0) | (b'-', 0, _) => {
                    rays.push(((x - dy, y - dx), (-dy, -dx)));
                    (dx, dy) = (dy, dx);
                }
                _ => {}
            }

            (x, y) = (x + dx, y + dy)
        }
    }

    seen.len()
}
