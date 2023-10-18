use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
};

/// Find the highest amount of asteroids visible from any one of them.
pub fn one(input: &str) -> crate::Result<usize> {
    locate_base(input).map(|v| v.1).ok_or("no base".into())
}

/// Find the 200th asteroid destroyed by a spinning laser located on the asteroid from which
/// the most other asteroids are visible.
pub fn two(input: &str) -> crate::Result<i32> {
    let base = locate_base(input).ok_or("no base")?.0;
    let mut asteroids = parse(input);
    asteroids.retain(|&p| p != base);

    let laser_order = {
        // Collect asteroids into buckets corresponding to the line they are on, to sort out
        // on which rotation they are destroyed.
        let mut map = HashMap::new();
        for &p in &asteroids {
            let (gradient, gcd) = gradient(base, p);
            map.entry(gradient).or_insert(vec![]).push((p, gcd));
        }
        // A list of asteroid positions alongside the angle of the laser they will be
        // vaporised at.
        let mut with_hit_angle: Vec<_> = map
            .into_values()
            .flat_map(|mut asteroids| {
                asteroids.sort_unstable_by_key(|v| v.1);
                asteroids
                    .into_iter()
                    .enumerate()
                    .map(|(obscured_by, (position, _))| {
                        (position, laser_angle(base, position, obscured_by))
                    })
            })
            .collect();
        with_hit_angle.sort_unstable_by(|a, b| a.1.total_cmp(&b.1));
        with_hit_angle
    };

    let (x, y) = laser_order[199].0;
    Ok(100 * x + y)
}

/// Locates the base in the puzzle input, alongside the number of asteroids seen from it.
fn locate_base(input: &str) -> Option<((i32, i32), usize)> {
    let spots = parse(input);
    spots
        .iter()
        .map(|&s| {
            let mut visible = HashSet::new();
            visible.extend(spots.iter().filter(|&&p| p != s).map(|&p| gradient(s, p).0));
            (s, visible.len())
        })
        .max_by_key(|v| v.1)
}

/// Given the position of the base, an asteroid, and how many other asteroids it is obscured by,
/// returns the angle of the laser at which it will be vaporized, in radians.
fn laser_angle(base: (i32, i32), asteroid: (i32, i32), obscured_by: usize) -> f32 {
    const TWO_PI: f32 = 2.0 * PI;

    let (x, y) = ((asteroid.0 - base.0) as f32, (asteroid.1 - base.1) as f32);
    // atan2 starts at positive x and goes counterclockwise; this formula transforms it into
    // starting at negative y (up) and going clockwise.
    (2.5 * PI + y.atan2(x)) % TWO_PI + obscured_by as f32 * TWO_PI
}

/// Breaks up an asteroid position into the gradient of the line it is on, and how far
/// along the line it is.
fn gradient((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> ((i32, i32), i32) {
    let gcd = crate::common::gcd(x1.abs_diff(x2) as usize, y1.abs_diff(y2) as usize) as i32;
    (((x2 - x1) / gcd, (y2 - y1) / gcd), gcd)
}

/// Parses the puzzle input into a list of asteroid positions.
fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect()
}
