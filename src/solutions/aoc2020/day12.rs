/// Get taxicab distance to (0, 0) after running the course.
pub fn one(input: &str) -> crate::Result<i32> {
    let (p, _) = parse(input).fold(((0, 0), (1, 0)), |((x, y), d), (cmd, arg)| match cmd {
        'N' => ((x, y - arg), d),
        'S' => ((x, y + arg), d),
        'E' => ((x + arg, y), d),
        'W' => ((x - arg, y), d),
        'L' => ((x, y), rotated(d, 360 - arg)),
        'R' => ((x, y), rotated(d, arg)),
        'F' => ((x + arg * d.0, y + arg * d.1), d),
        _ => ((x, y), d),
    });
    Ok(p.0.abs() + p.1.abs())
}

/// Get taxicab distance to (0, 0) after running the waypoint-based course.
pub fn two(input: &str) -> crate::Result<i32> {
    let (p, _) = parse(input).fold(((0, 0), (10, -1)), |(p, (dx, dy)), (cmd, arg)| match cmd {
        'N' => (p, (dx, dy - arg)),
        'S' => (p, (dx, dy + arg)),
        'E' => (p, (dx + arg, dy)),
        'W' => (p, (dx - arg, dy)),
        'L' => (p, rotated((dx, dy), 360 - arg)),
        'R' => (p, rotated((dx, dy), arg)),
        'F' => ((p.0 + arg * dx, p.1 + arg * dy), (dx, dy)),
        _ => (p, (dx, dy)),
    });
    Ok(p.0.abs() + p.1.abs())
}

/// Returns the same point rotated around (0, 0) by the given amount of degrees. Only works
/// for multiples of 90, will (incorrectly) return the original point otherwise.
fn rotated((x, y): (i32, i32), angle: i32) -> (i32, i32) {
    match angle {
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        _ => (x, y),
    }
}

/// Parses the puzzle input into a series of (command, argument) pairs.
fn parse(input: &str) -> impl Iterator<Item = (char, i32)> + '_ {
    input
        .lines()
        .filter_map(|line| Some((line.chars().next()?, line[1..].parse().ok()?)))
}
