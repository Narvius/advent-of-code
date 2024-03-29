/// Count the number of lit lights on the grid.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(run_instructions(input)
        .into_iter()
        .flatten()
        .filter(|&b| b)
        .count())
}

/// Display the contents of the grid after running all instructions.
pub fn two(input: &str) -> crate::Result<String> {
    let map = run_instructions(input);
    Ok(crate::common::pixel_display(50, 6, |x, y| map[y][x]))
}

/// Runs the instructions on a 50x6 grid of pixels and returns the resulting grid.
fn run_instructions(input: &str) -> Vec<Vec<bool>> {
    let mut map = vec![vec![false; 50]; 6];
    for line in parse(input) {
        match line {
            Line::Rect(x, y) => {
                for line in map.iter_mut().take(y) {
                    for b in line.iter_mut().take(x) {
                        *b = true;
                    }
                }
            }
            Line::RotRow(y, shift) => map[y].rotate_right(shift),
            Line::RotCol(x, shift) => {
                let mut buf: Vec<_> = (0..map.len()).map(|y| map[y][x]).collect();
                buf.rotate_right(shift);
                for y in 0..map.len() {
                    map[y][x] = buf[y];
                }
            }
        }
    }
    map
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
