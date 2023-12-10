/// Find half the length of the loop.
pub fn one(input: &str) -> crate::Result<usize> {
    let (map, start_position, dir) = parse(input).ok_or("parse failed")?;

    let (mut state, mut steps) = ((start_position, dir), 0);
    loop {
        (state, steps) = (step(&map, state.0, state.1), steps + 1);
        if state.0 == start_position {
            return Ok(steps / 2);
        }
    }
}

/// Count the number of tiles inside of the loop.
pub fn two(input: &str) -> crate::Result<usize> {
    let (map, mut pos, mut dir) = parse(input).ok_or("parse failed")?;
    let (w, h) = (map[0].len(), map.len());

    // Find the whole shape, alongside the smallest rectangle it is contained in.
    let mut shape = vec![vec![false; w]; h];
    loop {
        if shape[pos.1 as usize][pos.0 as usize] {
            break;
        }

        shape[pos.1 as usize][pos.0 as usize] = true;
        (pos, dir) = step(&map, pos, dir);
    }

    // Conceptually, we are casting a ray at a 45 degree angle to the top left and counting the
    // intersections with the loop. If there's an odd number of intersections, the point is inside.
    // Due to the angle chosen, L and 7 bends count as two intersections, while all other loop
    // tiles count as one each.
    //
    // For the implementation, we are walking from the *end* of the ray backwards; this allows us
    // to reuse calculations for all points whose ray is on the same parallel line.

    // We consider every diagonal as starting on y = 0 in the rectangle. Some diagonals are cut off
    // due to the rectangle shape; those start at negative x values, but skip the first couple
    // points until they're in the rectangle.
    let mut inside_points = 0;
    for x in -(h as i32)..w as i32 {
        // Calculate the x range the diagonal intersects the rectangle in.
        let min_step = 0.max(-x);
        let max_step = i32::min(w as i32 - x, h as i32);

        // Run along the diagonal.
        let mut next_is_inside = false;
        for n in min_step..max_step {
            let (x, y) = ((x + n) as usize, n as usize);
            if shape[y][x] {
                next_is_inside ^= match map[y][x] {
                    b'-' | b'|' | b'F' | b'J' => true,
                    b'S' => {
                        // We unfortunately have to check which kind of bend the 'S' tile is.
                        let left = b"-FL".contains(&map[y][x - 1]);
                        let top = b"|F7".contains(&map[y - 1][x]);
                        let right = b"-7J".contains(&map[y][x + 1]);
                        let bottom = b"|JL".contains(&map[y + 1][x]);

                        !((top && right) || (left && bottom))
                    }
                    _ => false,
                };
            } else if next_is_inside {
                inside_points += 1;
            }
        }
    }

    Ok(inside_points)
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
