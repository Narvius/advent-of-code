use std::collections::HashSet;

use crate::common;

/// Count the number of unique tiles visited by the guard's walk.
pub fn one(input: &str) -> crate::Result<usize> {
    let (mut pos, grid) = parse(input);
    let mut dir = (0, -1);
    let mut visited = HashSet::from([pos]);

    while let Some(after) = step(&grid, pos, dir, None) {
        (pos, dir) = after;
        visited.insert(pos);
    }

    Ok(visited.len())
}

/// Find the number of unique tiles a single obstacle can be placed in to lock the guard into a
/// loop.
pub fn two(input: &str) -> crate::Result<usize> {
    // Concept: Potential obstacle locations are on the path of the guard. So we do a regular walk
    // (the `outer` walk), and any time we would take a step forward, start a new, `inner` walk
    // pretending there's an obstacle in that spot. If the inner walk detects the same
    // position+orientation twice, it's a loop.

    let (mut pos, grid) = parse(input);
    let mut dir = (0, -1);
    let mut loops = 0;

    // `visited` is to keep track of paradoxical paths (see larger comment couple lines down);
    // `visited_with_dir` is a performance optimization that makes it faster to detect loops.
    let mut visited = HashSet::from([pos]);
    let mut visited_with_dir = HashSet::from([(pos, dir)]);

    // Outer walk.
    while let Some(after) = step(&grid, pos, dir, None) {
        if pos != after.0 && !visited.contains(&after.0) {
            // Do the inner walk with one added obstacle if the tile ahead is open and hasn't been
            // visited before in the outer walk. The latter condition is because if we placed an
            // obstacle there, we wouldn't have reached the position we're in right now anyway.
            let (mut pos, mut dir) = (pos, dir);
            let mut visited = visited_with_dir.clone();

            while let Some(after) = step(&grid, pos, dir, Some(after.0)) {
                (pos, dir) = after;
                if !visited.insert(after) {
                    loops += 1;
                    break;
                }
            }
        }

        (pos, dir) = after;
        visited.insert(pos);
        visited_with_dir.insert((pos, dir));
    }

    Ok(loops)
}

/// Performs a single step on the grid; a step is either moving one coordinate forward, or turning
/// right once if there's an obstacle.
///
/// If `obstacle` is given, those coordinates are treated as obstacle regardless of what's there on
/// the `grid`.
fn step(
    grid: &[&[u8]],
    mut pos: (i32, i32),
    mut dir: (i32, i32),
    obstacle: Option<(i32, i32)>,
) -> Option<((i32, i32), (i32, i32))> {
    let (Ok(x), Ok(y)) = (
        usize::try_from(pos.0 + dir.0),
        usize::try_from(pos.1 + dir.1),
    ) else {
        return None;
    };

    match grid.get(y).and_then(|line| line.get(x)) {
        c if c == Some(&b'#') || Some((x as i32, y as i32)) == obstacle => dir = (-dir.1, dir.0),
        Some(_) => pos = (x as i32, y as i32),
        None => return None,
    }

    Some((pos, dir))
}

/// Parses the puzzle input into a starting location and a grid.
fn parse(input: &str) -> ((i32, i32), Vec<&[u8]>) {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    let pos = common::grid_coordinates(&grid).find(|&(x, y)| grid[y as usize][x as usize] == b'^');
    (pos.expect("no starting position"), grid)
}
