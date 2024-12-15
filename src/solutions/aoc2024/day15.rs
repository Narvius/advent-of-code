use std::mem;

/// Find the box checksum after performing all steps.
pub fn one(input: &str) -> crate::Result<i32> {
    walk(input, false)
}

/// Find the box checksum after performing all steps in wide mode.
pub fn two(input: &str) -> crate::Result<i32> {
    walk(input, true)
}

/// Performs the robot walk and returns the final box checksum after all instructions are finished.
fn walk(input: &str, wide: bool) -> crate::Result<i32> {
    let (mut robot, mut grid, instructions) = parse(input, wide);

    for instruction in instructions {
        let delta = match instruction {
            b'<' => (-1, 0),
            b'^' => (0, -1),
            b'>' => (1, 0),
            b'v' => (0, 1),
            _ => continue,
        };

        let (x, y) = (robot.0 + delta.0, robot.1 + delta.1);
        match get(&grid, (x, y)) {
            Some(Cell::Wall) | None => { /* Blocks, nothing happens */ }
            Some(Cell::Empty) => robot = (x, y),
            Some(Cell::Box) | Some(Cell::BoxExt) => {
                if can_push(&grid, (x, y), delta) {
                    push_box(&mut grid, (x, y), delta);
                    robot = (x, y);
                }
            }
        }
    }
    Ok((0..grid.len())
        .flat_map(|y| (0..grid[y].len()).map(move |x| (x as i32, y as i32)))
        .filter_map(|(x, y)| (get(&grid, (x, y)) == Some(Cell::Box)).then_some(100 * y + x))
        .sum())
}

/// Pushes the given box in the given direction. Respects wide mode. Assumes the move is possible
/// and may not halt or panic if it isn't. Ends up a no-op if (x, y) is an open space.
fn push_box(grid: &mut Grid, (x, y): V2, (dx, dy): V2) {
    match (dx, dy) {
        // Horizontal push. Identical in either mode.
        (_, 0) => {
            let mut cell = mem::replace(&mut grid[y as usize][x as usize], Cell::Empty);
            let mut i = 1;
            while cell != Cell::Empty {
                mem::swap(&mut cell, &mut grid[y as usize][(x + i * dx) as usize]);
                i += 1;
            }
        }

        // Vertical push. Requires recursive handling due to wide mode.
        (0, _) => {
            let Some(((lx, ly), right)) = box_at(grid, (x, y)) else {
                return;
            };

            push_box(grid, (lx + dx, ly + dy), (dx, dy));
            grid[(y + dy) as usize][lx as usize] =
                mem::replace(&mut grid[y as usize][lx as usize], Cell::Empty);

            if let Some((rx, ry)) = right {
                push_box(grid, (rx + dx, ry + dy), (dx, dy));
                grid[(y + dy) as usize][rx as usize] =
                    mem::replace(&mut grid[y as usize][rx as usize], Cell::Empty);
            }
        }

        _ => unreachable!(),
    }
}

/// Checks that a box can be pushed.
fn can_push(grid: &Grid, p: V2, v: V2) -> bool {
    fn free_spot(grid: &Grid, (x, y): V2, (dx, dy): V2) -> bool {
        match get(grid, (x + dx, y + dy)) {
            Some(Cell::Wall) | None => false,
            Some(Cell::Empty) => true,
            Some(Cell::Box | Cell::BoxExt) if dx != 0 => {
                free_spot(grid, (x + dx, y + dy), (dx, dy))
            }
            _ => can_push(grid, (x + dx, y + dy), (dx, dy)),
        }
    }

    let Some((left, right)) = box_at(grid, p) else {
        return true;
    };

    free_spot(grid, left, v) && right.map(|p| free_spot(grid, p, v)).unwrap_or(true)
}

/// Returns both parts of the box at the given position. Effectively a no-op outside of wide mode.
fn box_at(grid: &Grid, (x, y): V2) -> Option<(V2, Option<V2>)> {
    match (get(grid, (x, y))?, get(grid, (x + 1, y))?) {
        (Cell::Box, Cell::BoxExt) => Some(((x, y), Some((x + 1, y)))),
        (Cell::BoxExt, _) => Some(((x - 1, y), Some((x, y)))),
        (Cell::Box, _) => Some(((x, y), None)),
        _ => None,
    }
}

/// Returns the grid cell at the given coordinates.
fn get(grid: &Grid, p: (i32, i32)) -> Option<Cell> {
    let (x, y) = (usize::try_from(p.0).ok()?, usize::try_from(p.1).ok()?);
    grid.get(y).and_then(|line| line.get(x)).cloned()
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// A single cell of the map.
enum Cell {
    /// An empty space.
    Empty,
    /// A wall.
    Wall,
    /// Either a standalone box, or the left side of a box in wide mode. Must always be moved
    /// together with its matching [`Cell::BoxExt`].
    Box,
    /// The right side of a box. Must always be moved together with its matching [`Cell::Wall`].
    BoxExt,
}

type V2 = (i32, i32);
type Grid = Vec<Vec<Cell>>;

/// Parses the puzzle input into the starting robot position, a grid of cells, and a list of
/// instructions for the robot to follow. If `wide` is given, interprets the grid as twice as wide,
/// with the expansion rules described in the puzzle.
fn parse(input: &str, wide: bool) -> (V2, Grid, impl Iterator<Item = u8> + '_) {
    let mut robot = None;
    let (map, instructions) = input.split_once("\r\n\r\n").expect("correct input");
    let map = map
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .flat_map(|(x, c)| {
                    if wide {
                        (match c {
                            b'#' => vec![Cell::Wall, Cell::Wall],
                            b'O' => vec![Cell::Box, Cell::BoxExt],
                            b'@' => {
                                robot = Some((2 * x as i32, y as i32));
                                vec![Cell::Empty, Cell::Empty]
                            }
                            _ => vec![Cell::Empty, Cell::Empty],
                        })
                        .into_boxed_slice()
                    } else {
                        Box::from(match c {
                            b'#' => [Cell::Wall],
                            b'O' => [Cell::Box],
                            b'@' => {
                                robot = Some((x as i32, y as i32));
                                [Cell::Empty]
                            }
                            _ => [Cell::Empty],
                        })
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (robot.expect("a robot on map"), map, instructions.bytes())
}
