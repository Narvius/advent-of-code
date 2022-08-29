use std::collections::HashSet;

/// Decipher the message.
pub fn one(input: &str) -> crate::Result<String> {
    let mut points = parse(input);
    advance_until_minimal(&mut points);
    let (_, (lx, ly, hx, hy)) = bounding_rect(&points);
    let points: HashSet<_> = points.into_iter().map(|p| p.0).collect();

    let mut result = String::with_capacity(((hx - lx + 1) * (hy - ly)) as usize);
    for y in ly..=hy {
        result.push('\n');
        for x in lx..=hx {
            result.push(if points.contains(&(x, y)) { '#' } else { '.' });
        }
    }
    Ok(result)
}

/// Find the amount of iterations needed to reach the message.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut points = parse(input);
    Ok(advance_until_minimal(&mut points))
}

/// Advances the position of a point cloud until they reach their smallest
/// bounding rectangle; returns the number of iterations used to reach that
/// point.
fn advance_until_minimal(points: &mut [(Point, Point)]) -> usize {
    let mut used_area = bounding_rect(points).0;
    let mut iterations = 0;

    loop {
        for ((x, y), (dx, dy)) in points.iter_mut() {
            *x += *dx;
            *y += *dy;
        }

        let new_area = bounding_rect(points).0;
        if new_area > used_area {
            for ((x, y), (dx, dy)) in points {
                *x -= *dx;
                *y -= *dy;
            }
            return iterations;
        }

        used_area = new_area;
        iterations += 1;
    }
}

/// Calculates the bounding rect of a cloud of points, returning the area and left, top, right
/// and bottom edges.
fn bounding_rect(points: &[(Point, Point)]) -> (i64, (i32, i32, i32, i32)) {
    let (mut lx, mut ly, mut hx, mut hy) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for &((x, y), _) in points {
        lx = lx.min(x);
        ly = ly.min(y);
        hx = hx.max(x);
        hy = hy.max(y);
    }
    ((hx - lx) as i64 * (hy - ly) as i64, (lx, ly, hx, hy))
}

type Point = (i32, i32);

/// Parses the puzzle input into a list of points and velocities.
fn parse(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .filter_map(|line| {
            let x = line[10..16].trim().parse().ok()?;
            let y = line[18..24].trim().parse().ok()?;
            let dx = line[36..38].trim().parse().ok()?;
            let dy = line[40..42].trim().parse().ok()?;
            Some(((x, y), (dx, dy)))
        })
        .collect()
}
