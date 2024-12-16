use std::{collections::HashSet, iter};

use crate::common::{self, Grid};

/// Count the number of unique tiles visited by the guard's walk.
pub fn one(input: &str) -> crate::Result<usize> {
    let (starting_position, grid) = parse(input);
    let pd = Some((starting_position, (0, -1)));
    Ok(iter::successors(pd, |&(p, d)| step(&grid, p, d, None))
        .map(|p| p.0)
        .collect::<HashSet<_>>()
        .len())
}

/// Find the number of unique tiles a single obstacle can be placed in to lock the guard into a
/// loop.
pub fn two(input: &str) -> crate::Result<usize> {
    // Concept: Potential obstacle locations are on the path of the guard. So we do a regular walk
    // (the `outer` walk), and any time we would take a step forward, check if there would be a
    // loop if there was an obstacle there.

    let (pos, grid) = parse(input);
    let pd = Some((pos, (0, -1)));
    let outer_walk = iter::successors(pd, |&(p, d)| step(&grid, p, d, None));

    let mut visited = HashSet::new();
    let mut loops = 0;

    for ((prev_p, _), (p, d)) in outer_walk.clone().zip(outer_walk.skip(1)) {
        if !visited.insert(p) || prev_p == p {
            continue;
        }

        let pd = Some((prev_p, (-d.1, d.0)));
        let obstacle = Some(p);
        let inner_walk = iter::successors(pd, |&(p, d)| step(&grid, p, d, obstacle));

        if common::has_cycle(inner_walk) {
            loops += 1;
        }
    }

    Ok(loops)
}

/// Performs a single step on the grid; a step is either moving one coordinate forward, or turning
/// right once if there's an obstacle.
///
/// If `obstacle` is given, those coordinates are treated as obstacle regardless of what's there on
/// the `grid`.
fn step(grid: &Grid<u8>, mut pos: V2, mut dir: V2, obstacle: Option<V2>) -> Option<(V2, V2)> {
    let (x, y) = (pos.0 + dir.0, pos.1 + dir.1);
    match grid.get((x, y)) {
        c if c == Some(&b'#') || Some((x, y)) == obstacle => dir = (-dir.1, dir.0),
        Some(_) => pos = (x, y),
        None => return None,
    }

    Some((pos, dir))
}

type V2 = (i32, i32);

/// Parses the puzzle input into a starting location and a grid.
fn parse(input: &str) -> (V2, Grid<u8>) {
    let grid = Grid::from_input(input);
    let pos = grid.find(|&e| e == b'^');
    (pos.expect("a starting position"), grid)
}
