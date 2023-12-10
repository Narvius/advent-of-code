use std::collections::HashSet;

/// Find half the length of the loop.
pub fn one(input: &str) -> crate::Result<usize> {
    let (map, (x, y), (dx, dy)) = parse(input).ok_or("parse failed")?;

    let (mut state, mut steps) = (step(&map, (x, y), (dx, dy)), 1);
    while state.0 != (x, y) {
        (state, steps) = (step(&map, state.0, state.1), steps + 1);
    }

    Ok(steps / 2)
}

/// Count the number of tiles inside of the loop.
pub fn two(input: &str) -> crate::Result<usize> {
    let (map, mut pos, mut dir) = parse(input).ok_or("parse failed")?;

    // Find the whole shape, for collision detection.
    let mut shape = HashSet::new();
    loop {
        if !shape.insert(pos) {
            break;
        }

        (pos, dir) = step(&map, pos, dir);
    }

    // Find the smallest rectangle containing the loop.
    let (mut l, mut t, mut r, mut b) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for &(x, y) in &shape {
        l = l.min(x);
        t = t.min(y);
        r = r.max(x);
        b = b.max(y);
    }

    // Within that rectangle, count the number of tiles that are inside.
    Ok((l..=r)
        .flat_map(|x| (t..=b).map(move |y| (x, y)))
        .filter(|&p| is_inside(&map, &shape, p, (r, b)))
        .count())
}

/// Transforms a position and direction by taking a step along the loop.
fn step(map: &[&[u8]], (x, y): Pos, (dx, dy): Dir) -> (Pos, Dir) {
    let (x, y) = (x + dx, y + dy);
    let (dx, dy) = match (map[y as usize][x as usize], dx, dy) {
        (b'L', 0, 1) => (1, 0),
        (b'L', -1, 0) => (0, -1),
        (b'F', -1, 0) => (0, 1),
        (b'F', 0, -1) => (1, 0),
        (b'7', 0, -1) => (-1, 0),
        (b'7', 1, 0) => (0, 1),
        (b'J', 1, 0) => (0, -1),
        (b'J', 0, 1) => (-1, 0),
        _ => (dx, dy),
    };
    ((x, y), (dx, dy))
}

/// Checks if a given position is inside the loop. It does so by casting a ray at a 45 degree angle
/// downwards to the right, and counting the intersections with the loop. If there's an odd number
/// of intersections, the point is inside.
///
/// Due to the angle chosen, L and 7 bends count as two intersections, while all other loop tiles
/// count as one each.
///
/// `max_x` and `max_y` are used as a stop point for the ray cast.
fn is_inside(map: &[&[u8]], shape: &HashSet<Pos>, (x, y): Pos, (max_x, max_y): Pos) -> bool {
    // If we're on the shape, we can't be inside.
    if shape.contains(&(x, y)) {
        return false;
    }

    // Count intersections of shape with a line going down-right at a 45 degree angle.
    let steps = i32::min(max_x - x, max_y - y);
    let mut intersects = 0;
    for n in 1..=steps {
        if shape.contains(&(x + n, y + n)) {
            intersects += match map[(y + n) as usize][(x + n) as usize] {
                b'L' | b'7' => 2,
                b'-' | b'|' | b'F' | b'J' => 1,
                b'S' => {
                    // We unfortunately have to check which kind of bend the 'S' tile is.
                    let (x, y) = ((x + n) as usize, (y + n) as usize);
                    let left = b"-FL".contains(&map[y][x - 1]);
                    let top = b"|F7".contains(&map[y - 1][x]);
                    let right = b"-7J".contains(&map[y][x + 1]);
                    let bottom = b"|JL".contains(&map[y - 1][x + 1]);

                    1 + usize::from((top && right) || (left && bottom))
                }
                _ => 0,
            };
        }
    }
    intersects % 2 == 1
}

type Pos = (i32, i32);
type Dir = (i32, i32);

/// Parses the puzzle input into a map, starting position, and starting direction.
fn parse(input: &str) -> Option<(Vec<&[u8]>, Pos, Dir)> {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();

    let (x, y) = (0..map.len())
        .flat_map(|y| (0..map[y].len()).map(move |x| (x as i32, y as i32)))
        .find(|&(x, y)| map[y as usize][x as usize] == b'S')?;

    let (_, dx, dy) = [
        (b"-FL", -1, 0),
        (b"|F7", 0, -1),
        (b"-7J", 1, 0),
        (b"|JL", 0, 1),
    ]
    .into_iter()
    .find(|(s, dx, dy)| s.contains(&map[(y + dy) as usize][(x + dx) as usize]))?;

    Some((map, (x, y), (dx, dy)))
}
