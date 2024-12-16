use crate::common::{self, Dir, Grid, ALL_DIRS};

/// Count the number of `XMAS`es in the grid.
pub fn one(input: &str) -> crate::Result<usize> {
    let grid = Grid::from_input(input);
    Ok(common::product(grid.coordinates(), ALL_DIRS)
        .filter(|&(p, d)| {
            std::iter::successors(Some(p), |&p| Some(p + d))
                .zip("XMAS".bytes())
                .all(|(p, c)| grid.get(p) == Some(&c))
        })
        .count())
}

/// Count the number of crossed `MAS`es in the grid.
pub fn two(input: &str) -> crate::Result<usize> {
    let grid = Grid::from_input(input);
    Ok(grid
        .iter_with_position()
        .filter(|&(p, c)| {
            *c == b'A' && {
                let diag = [Dir::NW, Dir::NE, Dir::SE, Dir::SW].map(|d| grid.get(p + d));
                matches!(
                    (diag[0], diag[2]),
                    (Some(b'M'), Some(b'S')) | (Some(b'S'), Some(b'M'))
                ) && matches!(
                    (diag[1], diag[3]),
                    (Some(b'M'), Some(b'S')) | (Some(b'S'), Some(b'M'))
                )
            }
        })
        .count())
}
