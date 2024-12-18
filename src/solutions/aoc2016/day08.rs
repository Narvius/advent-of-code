use crate::common::{self, Grid};

/// Count the number of lit lights on the grid.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(run_instructions(input).count(|&b| b))
}

/// Display the contents of the grid after running all instructions.
pub fn two(input: &str) -> crate::Result<String> {
    let grid = run_instructions(input);
    Ok(common::pixel_display(50, 6, |x, y| {
        grid[(x as i32, y as i32)]
    }))
}

/// Runs the instructions on a 50x6 grid of pixels and returns the resulting grid.
fn run_instructions(input: &str) -> Grid<bool> {
    let mut grid = Grid::from_elem(50, 6, false);
    for line in parse(input) {
        match line {
            Line::Rect(x, y) => {
                for x in 0..x as i32 {
                    for y in 0..y as i32 {
                        grid[(x, y)] = true;
                    }
                }
            }
            Line::RotRow(y, shift) => grid.rotate_row(y, shift as i32),
            Line::RotCol(x, shift) => grid.rotate_column(x, shift as i32),
        }
    }
    grid
}

/// A single instruction from the puzzle input.
enum Line {
    Rect(usize, usize),
    RotRow(usize, usize),
    RotCol(usize, usize),
}

/// Parses the puzzle input into a series of instructions.
fn parse(input: &str) -> impl Iterator<Item = Line> + '_ {
    input.lines().filter_map(
        |line| match *line.split(' ').collect::<Vec<_>>().as_slice() {
            ["rotate", place, coord, "by", shift] => {
                let coord = coord.split_once('=')?.1.parse().ok()?;
                let shift = shift.parse().ok()?;
                Some(match place {
                    "row" => Line::RotRow(coord, shift),
                    "column" => Line::RotCol(coord, shift),
                    _ => None?,
                })
            }
            ["rect", size] => {
                let (x, y) = size.split_once('x')?;
                Some(Line::Rect(x.parse().ok()?, y.parse().ok()?))
            }
            _ => None,
        },
    )
}
