use std::collections::HashMap;

use crate::common::{diamond_deltas, Grid, CARDINAL};

/// Count the amount of distinct range-2 cheats that save at least 100 steps.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(count_cheats(input, 2))
}

/// Count the amount of distinct range-20 cheats that save at least 100 steps.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(count_cheats(input, 20))
}

/// Counts the amount of distinct cheats that save at least 100 steps. A cheat is a pair of
/// starting and ending position, where their taxicab distance is less or equal `cheat_duration`.
fn count_cheats(input: &str, cheat_duration: usize) -> usize {
    let main_path = parse(input);
    let lookup: HashMap<V2, usize> = (main_path.iter().enumerate())
        .map(|(i, &p)| (p, i))
        .collect();

    (main_path.iter().enumerate())
        .flat_map(|(step, &p)| diamond_deltas(cheat_duration).map(move |d| (step, p, d)))
        .filter_map(|(step, p, d)| {
            let cheated_steps = *lookup.get(&(p.0 + d.0, p.1 + d.1))?;
            let extra_steps = (d.0.abs() + d.1.abs()) as usize;
            (cheated_steps >= step + extra_steps + 100).then_some(1)
        })
        .sum()
}

type V2 = (i32, i32);

/// Parses the puzzle input into a list of positions of the main path cells, in order.
fn parse(input: &str) -> Vec<V2> {
    let grid = Grid::from_input(input);
    let end = grid.find(|&e| e == b'E').expect("an end");

    let mut cell = grid.find(|&e| e == b'S').expect("a start");
    let mut main_path = vec![];
    let mut dir = *(CARDINAL.iter())
        .find(|&d| grid[cell + d] == b'.')
        .expect("a starting direction");

    while cell != end {
        main_path.push(cell);

        let next_dir = [dir, dir.left(), dir.right()]
            .into_iter()
            .find(|&d| grid[cell + d] != b'#')
            .expect("possible next step");

        (cell, dir) = (cell + next_dir, next_dir);
    }

    main_path.push(cell);
    main_path
}
