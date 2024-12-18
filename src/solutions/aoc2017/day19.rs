use crate::common::{Dir, Grid};

/// Return the string seen after walking the routing diagram.
pub fn one(input: &str) -> crate::Result<String> {
    Ok(walk(input)?.0)
}

/// Return the number of steps needed to walk the routing diagram.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(walk(input)?.1)
}

/// Walks the routing diagram and returns the seen string as well as the number of steps
/// taken.
fn walk(diagram: &str) -> crate::Result<(String, usize)> {
    let grid = parse(diagram);
    let initial = grid.find(|cell| cell.is_some()).map(|p| (p, Dir::S, None));
    let mut steps = 1; // walking ONTO the diagram counts as a step

    let seen = std::iter::successors(initial, |&(pos, dir, _)| {
        next_step(&grid, pos, dir).map(|(d, cell)| (pos + d, d, cell))
    })
    .inspect(|_| steps += 1)
    .filter_map(|(_, _, found)| found)
    .collect();

    Ok((seen, steps))
}

/// Returns the direction and tile for the next step.
fn next_step(grid: &Grid<Cell>, p: V2, d: Dir) -> Option<(Dir, Option<char>)> {
    [d, d.left(), d.right()]
        .into_iter()
        .find(|d| grid[p + d].is_some())
        .map(|d| Some((d, grid[p + d]?)))?
}

type V2 = (i32, i32);

/// Type used for map cells.
///
/// - outer option: `Some` if walkable, `None` if not
/// - inner option: contained letter
type Cell = Option<Option<char>>;

/// Parses the puzzle input into a two-dimensional map.
fn parse(input: &str) -> Grid<Cell> {
    Grid::from_iters(input.lines().map(|line| {
        line.bytes().map(|b| match b {
            b'|' | b'-' | b'+' => Some(None),
            b' ' => None,
            _ => Some(Some(b as char)),
        })
    }))
}
