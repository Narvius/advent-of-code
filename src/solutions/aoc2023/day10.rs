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

/// Count the number of points inside of the loop.
///
/// Uses the [shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula) to calculate the
/// "real" area of the polygon making up the shape, then [Pick's
/// theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem) to convert that "real" area to the
/// number of integer points inside that shape.
pub fn two(input: &str) -> crate::Result<i32> {
    let (map, mut pos, mut dir) = parse(input).ok_or("parse failed")?;

    // Compute all points on the shape.
    let mut shape = vec![pos];
    loop {
        (pos, dir) = step(&map, pos, dir);
        shape.push(pos);
        if shape.first() == shape.last() {
            break;
        }
    }

    // Use shoelace formula to find the actual area of the shape.
    let area = shape
        .windows(2)
        .map(|w| w[0].0 * w[1].1 - w[0].1 * w[1].0)
        .sum::<i32>()
        .abs();

    // Use Pick's theorem to find the number of integer coordinate points inside the shape, based
    // on the actual area.
    Ok(area / 2 - shape.len() as i32 / 2 + 1)
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
