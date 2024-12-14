use std::collections::HashSet;

/// Find the "security score" after 100 seconds (by sorting robots into quadrants).
pub fn one(input: &str) -> crate::Result<usize> {
    let mut scores = [0, 0, 0, 0];
    parse(input).for_each(|((x, y), (dx, dy))| {
        let x = (x + 100 * dx).rem_euclid(WIDTH);
        let y = (y + 100 * dy).rem_euclid(HEIGHT);

        let quadrant = usize::from(x < WIDTH / 2) + 2 * usize::from(y < WIDTH / 2);
        scores[quadrant] += 1;
    });
    Ok(scores.into_iter().product())
}

/// Count the number of seconds until there's a christmas tree.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut robots: Vec<_> = parse(input).collect();

    for i in 1.. {
        for ((x, y), (dx, dy)) in &mut robots {
            *x = (*x + *dx).rem_euclid(WIDTH);
            *y = (*y + *dy).rem_euclid(HEIGHT);
        }

        if detect_tree(&robots) {
            return Ok(i);
        }
    }

    unreachable!()
}

/// It just so happens, that the part 2 tree happens exactly when all robots have unique positions.
fn detect_tree(robots: &[Robot]) -> bool {
    robots.iter().map(|&(p, _)| p).collect::<HashSet<_>>().len() == robots.len()
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

type Robot = ((i32, i32), (i32, i32));

/// Parses the puzzle input into a series of robot position/speed pairs.
fn parse(input: &str) -> impl Iterator<Item = Robot> + '_ {
    input.lines().filter_map(|line| {
        let (p, v) = line.split_once(' ')?;
        let (x, y) = p[2..].split_once(',')?;
        let (dx, dy) = v[2..].split_once(',')?;

        let p = (x.parse().ok()?, y.parse().ok()?);
        let v = (dx.parse().ok()?, dy.parse().ok()?);

        Some((p, v))
    })
}
