use std::collections::HashMap;

use crate::common;

/// For each trailhead, find the number of reachable peaks; sum all those numbers.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(total_score_and_rating(input).0)
}

/// Find the total number of hiking trails that exist.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(total_score_and_rating(input).1)
}

/// Sums the score and rating (as described in the puzzle) across all trailheads in the input.
fn total_score_and_rating(input: &str) -> (usize, usize) {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    common::grid_coordinates(&grid)
        .map(|c| score_and_rating(&grid, c))
        .fold((0, 0), |(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
}

/// Finds the score and rating for a given trailhead.
///
/// Note that these two measures are more or less the same thing, that's why both are returned
/// together; the score is simply the amount of distinct peaks reached, whereas the rating is the
/// amount of times a peak was reached at all, both of which are tracked together anyway.
fn score_and_rating(grid: &[&[u8]], (x, y): (i32, i32)) -> (usize, usize) {
    if grid[y as usize][x as usize] != b'0' {
        return (0, 0);
    }

    let mut stack = vec![(x, y, b'0')];
    let mut goals: HashMap<(usize, usize), usize> = HashMap::new();
    while let Some((x, y, c)) = stack.pop() {
        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let (Ok(x), Ok(y)) = (usize::try_from(x + dx), usize::try_from(y + dy)) else {
                continue;
            };

            if let Some(n) = grid.get(y).and_then(|l| l.get(x)) {
                match (c, n) {
                    (b'8', b'9') => *goals.entry((x, y)).or_default() += 1,
                    _ if *n == (c + 1) => stack.push((x as i32, y as i32, *n)),
                    _ => {}
                }
            }
        }
    }

    (goals.len(), goals.values().sum())
}
