use std::collections::HashMap;

/// Find the number of new infections after ten thousand steps.
pub fn one(input: &str) -> crate::Result<usize> {
    solve(input, 10000, |c| match c {
        Cell::Clean => Cell::Weakened,
        _ => Cell::Clean,
    })
}

/// Using a more complex infection scheme, find the number of new infections after ten
/// million steps.
pub fn two(input: &str) -> crate::Result<usize> {
    solve(input, 10000000, |c| match c {
        Cell::Clean => Cell::Weakened,
        Cell::Weakened => Cell::Infected,
        Cell::Infected => Cell::Flagged,
        Cell::Flagged => Cell::Clean,
    })
}

/// Calculates the result of the puzzle, parametrized by the differences between the two
/// parts.
fn solve(input: &str, iterations: usize, infect: fn(Cell) -> Cell) -> crate::Result<usize> {
    Ok(carrier(input, infect)
        .take(iterations)
        .filter(|&c| c == Cell::Infected)
        .count())
}

/// Possible cell states. Theoretically, these could be replaced by numbers, and the
/// `infect` tables by modulo addition.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Cell {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

/// The virus carrier as described in the puzzle input. Acts as an iterator that, after every
/// call to `next`, returns the cell state it just produced.
struct Carrier {
    /// Contains all cells and their states. Cells not contained in the map are assumed to
    /// be [`clean`](Cell::Clean).
    cells: HashMap<(i32, i32), Cell>,
    /// Current position.
    position: (i32, i32),
    /// Currently-faced direction.
    delta: (i32, i32),
    /// Returns the next cell state given the previous one.
    infect: fn(Cell) -> Cell,
}

/// Constructs a carrier from the puzzle input and a cell state transform.
fn carrier(s: &str, infect: fn(Cell) -> Cell) -> Carrier {
    let x = s.lines().map(|l| l.len()).max().unwrap_or(0) as i32 / 2;
    let y = s.lines().count() as i32 / 2;
    let mut cells = HashMap::new();

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if c == '#' {
                cells.insert((x as i32, y as i32), Cell::Infected);
            }
        }
    }

    Carrier {
        cells,
        position: (x, y),
        delta: (0, -1),
        infect,
    }
}

impl Iterator for Carrier {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let cell = *self.cells.get(&self.position).unwrap_or(&Cell::Clean);
        let (x, y) = self.delta;
        self.delta = match cell {
            Cell::Clean => (y, -x),
            Cell::Weakened => (x, y),
            Cell::Infected => (-y, x),
            Cell::Flagged => (-x, -y),
        };
        let cell = (self.infect)(cell);
        self.cells.insert(self.position, cell);
        self.position = (
            self.position.0 + self.delta.0,
            self.position.1 + self.delta.1,
        );
        Some(cell)
    }
}
