use std::collections::HashSet;

/// Find the number of pairs of hailstones whose trajectories (ignoring the Z axis) intersect
/// within a specific range of values.
pub fn one(input: &str) -> crate::Result<usize> {
    const RANGE: std::ops::RangeInclusive<f64> = 200000000000000.0..=400000000000000.0;
    let lines: Vec<_> = parse(input).collect();

    let mut intersects = 0;
    for (i, &([x1, y1, _], [vx1, vy1, _])) in lines.iter().enumerate() {
        // Adapted from https://paulbourke.net/geometry/pointlineplane/javascript.txt.
        let (ax1, ay1) = (x1, y1);
        let (ax2, ay2) = (x1 + vx1, y1 + vy1);

        for &([x2, y2, _], [vx2, vy2, _]) in lines[(i + 1)..].iter() {
            let (bx1, by1) = (x2, y2);
            let (bx2, by2) = (x2 + vx2, y2 + vy2);

            let denom = (by2 - by1) * (ax2 - ax1) - (bx2 - bx1) * (ay2 - ay1);
            if denom.abs() <= f64::EPSILON {
                continue;
            }

            let n1 = ((bx2 - bx1) * (ay1 - by1) - (by2 - by1) * (ax1 - bx1)) / denom;
            let n2 = ((ax2 - ax1) * (ay1 - by1) - (ay2 - ay1) * (ax1 - bx1)) / denom;

            if n1 < 0.0 || n2 < 0.0 {
                continue;
            }

            let (x, y) = (ax1 + n1 * vx1, ay1 + n1 * vy1);

            if RANGE.contains(&x) && RANGE.contains(&y) {
                intersects += 1;
            }
        }
    }

    Ok(intersects)
}

/// Find the start position from which you can throw a rock that, going in a straight line, will
/// hit every hailstone.
pub fn two(input: &str) -> crate::Result<f64> {
    let lines: Vec<_> = parse(input).collect();
    // Find the velocity our magic rock has to have.
    let [rvx, rvy, rvz] = [
        velocity_for_axis(&lines, 0).ok_or("no X velocity")?,
        velocity_for_axis(&lines, 1).ok_or("no Y velocity")?,
        velocity_for_axis(&lines, 2).ok_or("no Z velocity")?,
    ];

    // Now, we know the velocity. Combining it and the velocity of any single hailstone we can
    // produce a line that contains all possible start positions for our rock. Using any *two*
    // hailstones we can produce two such lines; and intersect them to find the only possible start
    // position.

    let ([x1, y1, z1], [vx1, vy1, vz1]) = lines[0];
    let ([x2, y2, _], [vx2, vy2, _]) = lines[1];

    let a1 = (vy1 - rvy) / (vx1 - rvx);
    let b1 = y1 - a1 * x1;
    let a2 = (vy2 - rvy) / (vx2 - rvx);
    let b2 = y2 - a2 * x2;

    let x = (b2 - b1) / (a1 - a2);
    let y = a1 * x + b1;
    let z = z1 + (vz1 - rvz) * ((x - x1) / (vx1 - rvx));

    Ok(x + y + z)
}

/// Finds the only possible velocity along a given axis that allows you to hit all hailstones.
///
/// This is constrained, because some hailstones have the same velocity along an axis. So for
/// example, if two hailstones have the same X velocity, that means the X distance between them
/// remains constant forever. So if we want to hit both of them, your chosen velocity MUST cleanly
/// divide that distance in order to hit both. There are enough such pairs that each axis is
/// constrained to exactly one possible velocity.
fn velocity_for_axis(lines: &[(P, P)], axis: usize) -> Option<f64> {
    let mut candidates = HashSet::new();
    for (i, &(p1, v1)) in lines.iter().enumerate() {
        for &(p2, v2) in lines[(i + 1)..].iter() {
            // Originally, I wanted to save a list of constraints and calculate the final result
            // from that, but this is fast enough.
            if v1[axis] == v2[axis] && v1[axis].abs() > 100.0 {
                let mut set = HashSet::new();
                for candidate in -400..=400 {
                    let candidate = candidate as f64;
                    if candidate == v1[axis] {
                        continue;
                    }
                    if (p2[axis] - p1[axis]) % (candidate - v1[axis]).abs() < f64::EPSILON {
                        set.insert(candidate as i64);
                    }
                }

                if candidates.is_empty() {
                    candidates.extend(set);
                } else {
                    candidates.retain(|v| set.contains(v));
                }
            }
        }
    }
    (candidates.len() == 1).then(|| candidates.into_iter().next().unwrap() as f64)
}

type P = [f64; 3];

/// Parses the puzzle input into a list of (position, velocity) pairs for each hailstone.
fn parse(input: &str) -> impl Iterator<Item = (P, P)> + '_ {
    fn p3(s: &str) -> Option<P> {
        let p: Vec<_> = s
            .split(", ")
            .filter_map(|n| n.trim().parse().ok())
            .collect();
        p.try_into().ok()
    }

    input.lines().filter_map(|line| {
        let (p, v) = line.split_once(" @ ")?;
        Some((p3(p)?, p3(v)?))
    })
}
