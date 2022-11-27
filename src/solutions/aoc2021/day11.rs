use std::collections::HashSet;

/// Simulate the octopi for 100 steps and count the total number of flashes.
pub fn one(input: &str) -> crate::Result<usize> {
    let mut map = parse(input);
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut map);
    }
    Ok(flashes)
}

/// Find the number of steps needed until all octopi flash simultaneously.
pub fn two(input: &str) -> crate::Result<i32> {
    let mut map = parse(input);
    for steps in 1.. {
        if step(&mut map) == map.len() * map[0].len() {
            return Ok(steps);
        }
    }

    unreachable!()
}

/// Parses the puzzle input into a 2D grid of octopus levels.
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|s| s.bytes().map(|b| b - b'0').collect())
        .collect()
}

/// Steps the octopus simulation once and returns the number of flashes that occurred.
fn step(map: &mut Vec<Vec<u8>>) -> usize {
    let mut found = HashSet::new();
    let mut queue = vec![];

    for (y, x) in product(0..map.len() as i32, 0..map[0].len() as i32) {
        map[y as usize][x as usize] += 1;
        if map[y as usize][x as usize] > 9 {
            found.insert((x, y));
            queue.push((x, y));
        }
    }

    while let Some(p) = queue.pop() {
        for (x, y) in neighbours(map[0].len(), map.len(), p) {
            map[y as usize][x as usize] += 1;
            if map[y as usize][x as usize] > 9 && !found.contains(&(x, y)) {
                found.insert((x, y));
                queue.push((x, y));
            }
        }
    }

    let flashes = found.len();

    for (x, y) in found {
        map[y as usize][x as usize] = 0;
    }

    flashes
}

/// Returns an iterator over the neighbours of coordinate. Works for invalid coordinates, but will
/// most likely return nothing.
fn neighbours(width: usize, height: usize, (x, y): (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    product(-1..=1, -1..=1).filter_map(move |(dx, dy)| {
        let (x, y) = (x + dx, y + dy);
        ((dx, dy) != (0, 0) && (0..width as i32).contains(&x) && (0..height as i32).contains(&y))
            .then_some((x, y))
    })
}

/// Returns the carthesian product of two iterators.
fn product<A: Clone, B>(
    a: impl Iterator<Item = A>,
    b: impl Iterator<Item = B> + Clone,
) -> impl Iterator<Item = (A, B)> {
    a.flat_map(move |i| b.clone().map(move |j| (i.clone(), j)))
}
