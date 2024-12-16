use std::mem;

use crate::common::Grid;

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
        match grid.get((x, y)) {
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

    Ok(grid
        .iter_with_position()
        .filter_map(|((x, y), c)| (*c == Cell::Box).then_some(100 * y + x))
        .sum())
}

/// Pushes the given box in the given direction. Respects wide mode. Assumes the move is possible
/// and may not halt or panic if it isn't. Ends up a no-op if (x, y) is an open space.
fn push_box(grid: &mut Grid<Cell>, (x, y): V2, (dx, dy): V2) {
    match (dx, dy) {
        // Horizontal push. Identical in either mode.
        (_, 0) => {
            let mut cell = mem::replace(&mut grid[(x, y)], Cell::Empty);
            let mut i = 1;
            while cell != Cell::Empty {
                mem::swap(&mut cell, &mut grid[(x + i * dx, y)]);
                i += 1;
            }
        }

        // Vertical push. Requires recursive handling due to wide mode.
        (0, _) => {
            let Some(((lx, ly), right)) = box_at(grid, (x, y)) else {
                return;
            };

            push_box(grid, (lx + dx, ly + dy), (dx, dy));
            grid[(lx, y + dy)] = mem::replace(&mut grid[(lx, y)], Cell::Empty);

            if let Some((rx, ry)) = right {
                push_box(grid, (rx + dx, ry + dy), (dx, dy));
                grid[(rx, y + dy)] = mem::replace(&mut grid[(rx, y)], Cell::Empty);
            }
        }

        _ => unreachable!(),
    }
}

/// Checks that a box can be pushed.
fn can_push(grid: &Grid<Cell>, p: V2, v: V2) -> bool {
    fn free_spot(grid: &Grid<Cell>, (x, y): V2, (dx, dy): V2) -> bool {
        match grid.get((x + dx, y + dy)) {
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
fn box_at(grid: &Grid<Cell>, (x, y): V2) -> Option<(V2, Option<V2>)> {
    match (grid.get((x, y))?, grid.get((x + 1, y))?) {
        (Cell::Box, Cell::BoxExt) => Some(((x, y), Some((x + 1, y)))),
        (Cell::BoxExt, _) => Some(((x - 1, y), Some((x, y)))),
        (Cell::Box, _) => Some(((x, y), None)),
        _ => None,
    }
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

/// Parses the puzzle input into the starting robot position, a grid of cells, and a list of
/// instructions for the robot to follow. If `wide` is given, interprets the grid as twice as wide,
/// with the expansion rules described in the puzzle.
fn parse(input: &str, wide: bool) -> (V2, Grid<Cell>, impl Iterator<Item = u8> + '_) {
    let (map, instructions) = input.split_once("\r\n\r\n").expect("correct input");
    let robot =
        Grid::from_input(map)
            .find(|&e| e == b'@')
            .map(|(x, y)| if wide { (2 * x, y) } else { (x, y) });

    let map = Grid::from_iters(map.lines().map(|line| {
        line.bytes().flat_map(move |c| {
            if wide {
                (match c {
                    b'#' => vec![Cell::Wall, Cell::Wall],
                    b'O' => vec![Cell::Box, Cell::BoxExt],
                    _ => vec![Cell::Empty, Cell::Empty],
                })
                .into_boxed_slice()
            } else {
                Box::from(match c {
                    b'#' => [Cell::Wall],
                    b'O' => [Cell::Box],
                    _ => [Cell::Empty],
                })
            }
        })
    }));

    (robot.expect("a robot on map"), map, instructions.bytes())
}
