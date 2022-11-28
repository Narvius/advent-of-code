/// Find the number of seats taken in equilibrium using the basic ruleset.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(Map::from_input(input, false)
        .ok_or("failed parse")?
        .run_until_equilibrium())
}

/// Find the number of seats taken in equilibrium using the extended ruleset.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(Map::from_input(input, true)
        .ok_or("failed parse")?
        .run_until_equilibrium())
}

/// A cellular automaton following the rules outlined in the problem statement.
struct Map {
    map: Vec<Cell>,
    buffer: Vec<Cell>,
    target_cache: Vec<Vec<usize>>,
    leave_threshold: usize,
}

type Point = (i32, i32);

impl Map {
    /// Parses the map from input, and prepares all additional data (like the target cache).
    /// `extended_scan` decides if immediate neighbours (false) or first seats in line of
    /// sight (true) are counted.
    fn from_input(input: &str, extended_scan: bool) -> Option<Self> {
        let map: Vec<_> = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| if c == 'L' { Cell::Seat } else { Cell::Empty })
            .collect();
        let size = (
            input.lines().next()?.len() as i32,
            input.lines().count() as i32,
        );

        let target_cache: Vec<Vec<usize>> = match extended_scan {
            // Basic scan: Count immediately-adjacent seats.
            false => product(0..size.1, 0..size.0)
                .map(|(y, x)| {
                    product(-1..=1, -1..=1)
                        .filter(|&dir| dir != (0, 0))
                        .filter_map(|(dx, dy)| to_index((x + dx, y + dy), size))
                        .filter(|&i| map[i] == Cell::Seat)
                        .collect()
                })
                .collect(),

            // Extended scan: Count the first seat in a straight line in each direction.
            true => product(0..size.1, 0..size.0)
                .map(|(y, x)| {
                    product(-1..=1, -1..=1)
                        .filter(|&dir| dir != (0, 0))
                        .filter_map(|dir| in_direction(&map, (x, y), dir, size))
                        .collect()
                })
                .collect(),
        };

        Some(Self {
            map,
            buffer: vec![Cell::Empty; (size.0 * size.1) as usize],
            target_cache,
            leave_threshold: if extended_scan { 5 } else { 4 },
        })
    }

    /// Runs the automaton until it reaches equilibrium (no changes after a step), and returns
    /// the amount of taken seats afterwards.
    fn run_until_equilibrium(mut self) -> usize {
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..self.map.len() {
                self.buffer[i] = match (self.map[i], self.seen_count(i)) {
                    (Cell::Seat, 0) => Cell::Taken,
                    (Cell::Taken, n) if n >= self.leave_threshold => Cell::Seat,
                    (cell, _) => cell,
                };

                changed |= self.map[i] != self.buffer[i];
            }
            std::mem::swap(&mut self.map, &mut self.buffer);
        }
        self.map.into_iter().filter(|&c| c == Cell::Taken).count()
    }

    /// Returns the number of seen [taken](Cell::Taken) seats.
    fn seen_count(&self, index: usize) -> usize {
        self.target_cache[index]
            .iter()
            .filter(|&&i| self.map[i] == Cell::Taken)
            .count()
    }
}

/// Converts a coordinate pair into a 1D index, given the dimensions.
fn to_index((x, y): Point, (w, h): Point) -> Option<usize> {
    let valid = (0..w).contains(&x) && (0..h).contains(&y);
    valid.then_some((y * w + x) as usize)
}

/// Finds the first point starting at `(x, y)`, going in the direction `(dx, dy)` that is
/// a [seat](Cell::Seat).
fn in_direction(map: &[Cell], (x, y): Point, (dx, dy): Point, size: Point) -> Option<usize> {
    for n in 1.. {
        let index = to_index((x + dx * n, y + dy * n), size)?;
        if map[index] == Cell::Seat {
            return Some(index);
        }
    }
    unreachable!()
}

/// A single spot on the map.
#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Seat,
    Taken,
}

/// Returns the carthesian product of two iterators.
fn product<A: Clone, B>(
    a: impl Iterator<Item = A>,
    b: impl Iterator<Item = B> + Clone,
) -> impl Iterator<Item = (A, B)> {
    a.flat_map(move |i| b.clone().map(move |j| (i.clone(), j)))
}
