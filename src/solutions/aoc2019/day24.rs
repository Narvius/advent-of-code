use std::collections::HashSet;

/// Run the cellular automaton, compute a checksum from the first position
/// that repeats.
pub fn one(input: &str) -> crate::Result<i32> {
    let mut grid = parse(input);
    let mut buffer = [false; 25];
    let mut seen = HashSet::from([grid]);

    loop {
        for n in 0..25 {
            buffer[n] = rule(grid[n], neighbours(&grid, n));
        }
        std::mem::swap(&mut grid, &mut buffer);

        if !seen.insert(grid) {
            let bits = grid.into_iter().enumerate().filter(|(_, b)| *b);
            return Ok(bits.fold(0, |acc, (i, _)| acc | 1 << i));
        }
    }
}

/// Run the cellular automaton on a recursive grid for 200 steps, count
/// live cells.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut grid = vec![parse(input)];
    let mut buffer = vec![Grid::default()];

    for _ in 0..200 {
        // Add empty sentinel layers if the first or last one aren't empty.
        // This is because we *might* break into a new layer.
        if grid[0].iter().any(|&b| b) {
            grid.insert(0, Grid::default());
            buffer.insert(0, Grid::default());
        }
        if grid.last().unwrap().iter().any(|&b| b) {
            grid.push(Grid::default());
            buffer.push(Grid::default());
        }

        // Run the rule.
        for layer in 0..grid.len() {
            for n in 0..25 {
                buffer[layer][n] = rule(grid[layer][n], neighbours_recursive(&grid, layer, n));
            }
        }
        std::mem::swap(&mut grid, &mut buffer);
    }

    Ok(grid.into_iter().map(count).sum())
}

/// The rule powering the cellular automaton.
fn rule(alive: bool, neighbours: usize) -> bool {
    matches!((alive, neighbours), (true, 1) | (false, 1 | 2))
}

type Grid = [bool; 25];
type RecursiveGrid = Vec<[bool; 25]>;

/// Counts live neighbours for a cell in the grid.
fn neighbours(grid: &Grid, n: usize) -> usize {
    let (x, y) = (n % 5, n / 5);
    count([
        x != 0 && grid[n - 1],
        y != 0 && grid[n - 5],
        x != 4 && grid[n + 1],
        y != 4 && grid[n + 5],
    ])
}

/// Counts live neighbours for a cell in the recursive grid.
fn neighbours_recursive(grid: &RecursiveGrid, layer: usize, n: usize) -> usize {
    let (x, y) = (n % 5, n / 5);

    // The middle isn't a real square, it doesn't have neighbours.
    if (x, y) == (2, 2) {
        return 0;
    }

    let get = |layer_offset, n| {
        if n == 12 {
            return false;
        }
        let layer = usize::try_from(layer as i32 + layer_offset).ok();
        layer
            .and_then(|layer| grid.get(layer).map(|grid| grid[n]))
            .unwrap_or(false)
    };

    // Outer recursion.
    let outer = count([
        x == 0 && get(-1, 11),
        y == 0 && get(-1, 7),
        x == 4 && get(-1, 13),
        y == 4 && get(-1, 17),
    ]);

    // Inner recursion.
    let inner = match (x, y) {
        (1, 2) => count((0..5).map(|y| get(1, 5 * y))),
        (2, 1) => count((0..5).map(|x| get(1, x))),
        (3, 2) => count((0..5).map(|y| get(1, 4 + 5 * y))),
        (2, 3) => count((0..5).map(|x| get(1, 20 + x))),
        _ => 0,
    };

    neighbours(&grid[layer], n) + outer + inner
}

/// Counts the number of `true`s in an iterator of bools.
fn count(bools: impl IntoIterator<Item = bool>) -> usize {
    bools.into_iter().filter(|&b| b).count()
}

/// Parses the puzzle input into a [`Grid`].
fn parse(input: &str) -> Grid {
    let mut result = Grid::default();
    for (i, c) in input.lines().flat_map(|line| line.chars()).enumerate() {
        result[i] = c == '#';
    }
    result
}
