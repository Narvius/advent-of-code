use crate::common::{Grid, CARDINAL};

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
fn count_cheats(input: &str, cheat_duration: i32) -> usize {
    let main_path = parse(input);

    let mut count = 0;
    for (i, &(x1, y1)) in main_path.iter().enumerate() {
        for (normal_steps, &(x2, y2)) in main_path[i..].iter().skip(100).enumerate() {
            let distance = (x1.abs_diff(x2) + y1.abs_diff(y2)) as i32;
            if distance <= cheat_duration && (normal_steps as i32 - distance) >= 0 {
                count += 1;
            }
        }
    }
    count
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
