use std::collections::{HashMap, HashSet};

/// Run 10 rounds of elf movements and find a checksum from their position.
pub fn one(input: &str) -> crate::Result<i32> {
    let (mut elves, mut checks) = (parse(input), CHECKS);

    for _ in 0..10 {
        step_elves(&mut elves, &mut checks);
    }

    // Find the bounding box to calculate the number of empty tiles.
    let (l, t, r, b) = elves.iter().fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        |(l, t, r, b), &(x, y)| (l.min(x), t.min(y), r.max(x), b.max(y)),
    );

    Ok((1 + r - l) * (1 + b - t) - elves.len() as i32)
}

/// Find the number of the first round on which no elves moved.
pub fn two(input: &str) -> crate::Result<usize> {
    let (mut elves, mut checks) = (parse(input), CHECKS);

    Ok(1 + (0..)
        .take_while(|_| step_elves(&mut elves, &mut checks))
        .count())
}

/// Executes one round of elf movement; returns whether any elf moved.
fn step_elves(elves: &mut HashSet<Point>, checks: &mut [Check; 4]) -> bool {
    let mut proposed: HashMap<Point, Vec<Point>> = HashMap::new();

    // Calculate proposed moves.
    for &(x, y) in elves.iter() {
        let any_neighbour = (-1..=1)
            .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
            .any(|(dx, dy)| (dx, dy) != (0, 0) && elves.contains(&(x + dx, y + dy)));

        if !any_neighbour {
            continue;
        }

        let dir = (*checks)
            .into_iter()
            .find(|(dx, dy)| {
                (-1..=1).all(|i| !elves.contains(&(x + dx.unwrap_or(i), y + dy.unwrap_or(i))))
            })
            .map(|(dx, dy)| (dx.unwrap_or(0), dy.unwrap_or(0)));

        if let Some((dx, dy)) = dir {
            proposed.entry((x + dx, y + dy)).or_default().push((x, y));
        }
    }

    // Execute proposed moves that are possible.
    let mut any_moved = false;
    for (target, sources) in proposed {
        if sources.len() == 1 {
            elves.remove(&sources[0]);
            elves.insert(target);
            any_moved = true;
        }
    }

    // Move the first check to the end.
    checks.rotate_left(1);

    any_moved
}

/// A directional elf check. `(None, Some(-1))` for example is the north check;
/// using clever unwraps, this turns into (-1, -1), (0, -1) (1, -1) to check
/// all three northern spots, and into (0, -1) when executing the actual move.
type Check = (Option<i32>, Option<i32>);
/// A point in 2D space.
type Point = (i32, i32);

/// The initial set of directional elf checks. See [`Check`] for more information.
const CHECKS: [Check; 4] = [
    (None, Some(-1)), // North
    (None, Some(1)),  // South
    (Some(-1), None), // West
    (Some(1), None),  // East
];

/// Parses the puzzle input into a set of elf positions.
fn parse(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter_map(move |(x, c)| (c == '#').then_some((x as i32, y as i32)))
        })
        .collect()
}
