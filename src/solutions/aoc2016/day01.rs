use std::collections::HashSet;

/// Find the final position you end up at; find the taxicab distance from origin.
pub fn one(input: &str) -> Result<String, String> {
    let ((x, y), _) = parse(input).fold(((0, 0), (0, -1)), |((x, y), v), (r, d)| {
        let (vx, vy) = rotated(v, r);
        ((x + vx * d, y + vy * d), (vx, vy))
    });
    Ok((x.abs() + y.abs()).to_string())
}

/// Find the first coordinate you enter twice; find the taxicab distance from origin.
pub fn two(input: &str) -> Result<String, String> {
    let positions = parse(input)
        .scan(((0, 0), (0, -1)), |((x, y), v), (r, d)| {
            *v = rotated(*v, r);
            let mut ps = vec![];
            for _ in 0..d {
                *x += v.0;
                *y += v.1;
                ps.push((*x, *y));
            }
            Some(ps)
        })
        .flatten();

    let mut visited = HashSet::new();
    for p in positions {
        if !visited.insert(p) {
            return Ok((p.0.abs() + p.1.abs()).to_string());
        }
    }

    Err("did not find duplicate position".into())
}

/// Rotates a unit vector 90 degrees to either left or the right.
fn rotated((x, y): (i32, i32), right: bool) -> (i32, i32) {
    if right {
        (-y, x)
    } else {
        (y, -x)
    }
}

/// Parses the puzzle input into a sequence of instructions.
fn parse(input: &str) -> impl Iterator<Item = (bool, i32)> + '_ {
    input
        .split(", ")
        .filter_map(|s| Some((&s[0..1] == "R", s[1..].parse().ok()?)))
}
