use crate::common::Grid;

/// Count how many rolls of of paper are initially accessible.
pub fn one(input: &str) -> crate::Result<usize> {
    let g = Grid::from_input(input);
    Ok(g.iter_with_position()
        .filter(|((x, y), _)| accessible(&g, *x, *y))
        .count())
}

/// Count how many rolls of paper can be removed if accessible rolls get removed repeatedly.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut g = Grid::from_input(input);
    let mut changed = true;
    let mut removed = 0;

    while changed {
        changed = false;

        for y in 0..g.height() as i32 {
            for x in 0..g.width() as i32 {
                if accessible(&g, x, y) {
                    *g.get_mut((x, y)).unwrap() = b'.';
                    changed = true;
                    removed += 1;
                }
            }
        }
    }

    Ok(removed)
}

/// Checks whether a cell contains an accessible (fewer than 4 '@' neighbours) roll of paper (is
/// '@' itself).
fn accessible(grid: &Grid<'_, u8>, x: i32, y: i32) -> bool {
    #[rustfmt::skip]
    const DELTAS: [(i32, i32); 8] = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];

    grid.at((x, y)) == Some(b'@')
        && 4 > DELTAS
            .into_iter()
            .filter(|(dx, dy)| grid.at((x + dx, y + dy)) == Some(b'@'))
            .count()
}
