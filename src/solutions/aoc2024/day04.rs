use crate::common;

/// Count the number of `XMAS`es in the grid.
pub fn one(input: &str) -> crate::Result<usize> {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    let lines = common::product3(0..grid.len() as i32, 0..grid[0].len() as i32, DELTAS);
    Ok(lines
        .filter(|(x, y, (dx, dy))| {
            (0..NEEDLE.len())
                .all(|i| Some(NEEDLE[i]) == char_at(&grid, (x + dx * i as i32, y + dy * i as i32)))
        })
        .count())
}

/// Count the number of crossed `MAS`es in the grid.
pub fn two(input: &str) -> crate::Result<usize> {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    let tiles = common::product(0..grid.len() as i32, 0..grid[0].len() as i32);
    Ok(tiles
        .filter(|(x, y)| {
            let d1 = DIAG1.map(|(dx, dy)| char_at(&grid, (x + dx, y + dy)));
            let d2 = DIAG2.map(|(dx, dy)| char_at(&grid, (x + dx, y + dy)));

            (d1 == MAS || d1 == SAM) && (d2 == MAS || d2 == SAM)
        })
        .count())
}

/// Retrieves the character at the given position from the grid.
fn char_at(grid: &[&[u8]], (x, y): (i32, i32)) -> Option<u8> {
    let (x, y) = (usize::try_from(x).ok()?, usize::try_from(y).ok()?);
    grid.get(y).and_then(|line| line.get(x)).copied()
}

// Word searched for in part 1.
const NEEDLE: &[u8; 4] = b"XMAS";
// Directions to search in in part 1.
#[rustfmt::skip]
const DELTAS: [(i32, i32); 8] = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];

// Deltas for both diagonals for part 2.
const DIAG1: [(i32, i32); 3] = [(-1, -1), (0, 0), (1, 1)];
const DIAG2: [(i32, i32); 3] = [(-1, 1), (0, 0), (1, -1)];
// For ease of comparing diagonal values in part 2.
const MAS: [Option<u8>; 3] = [Some(b'M'), Some(b'A'), Some(b'S')];
const SAM: [Option<u8>; 3] = [Some(b'S'), Some(b'A'), Some(b'M')];
