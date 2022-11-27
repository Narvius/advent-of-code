use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Find the lowest risk achievable when crossing the grid.
pub fn one(input: &str) -> crate::Result<i32> {
    find_risk(parse(input)).ok_or_else(|| "no result".into())
}

/// Find the lowest risk achievable when crossing the fivefold expanded grid.
pub fn two(input: &str) -> crate::Result<i32> {
    find_risk(parse_large(input)).ok_or_else(|| "no result".into())
}

/// Calculates the lowest possible risk as per the puzzle rules.
fn find_risk(mut grid: Vec<Vec<i32>>) -> Option<i32> {
    let mut seekers = BinaryHeap::new();
    seekers.push(Seeker {
        cost: 0,
        position: (0, 0),
    });

    let (width, height) = (grid[0].len() as i32, grid.len() as i32);
    // Take the lowest-cost seeker so far.
    while let Some(seeker) = seekers.pop() {
        // If a seeker is on the endpoint, it has the correct answer. See comment below.
        if seeker.position == (width - 1, height - 1) {
            return Some(seeker.cost);
        }

        // Spawn new seekers on all adjacent unconsumed squares, consuming them.
        // This is valid, because that way a seeker on a square always represents the lowest-cost
        // possible seeker that it *could* have. By maintaining this property across the entire
        // grid, we end up with the lowest possible cost on the `endpoint`.
        for (x, y) in neighbours(seeker.position, width, height) {
            let v = &mut grid[y as usize][x as usize];
            if *v > 0 {
                seekers.push(Seeker {
                    position: (x, y),
                    cost: seeker.cost + *v,
                });
                *v = 0;
            }
        }
    }

    None
}

/// Gets orthogonally-adjacent coordinates that fit into `usize`.
fn neighbours((x, y): (i32, i32), width: i32, height: i32) -> impl Iterator<Item = (i32, i32)> {
    [(-1i32, 0i32), (0, -1), (1, 0), (0, 1)]
        .into_iter()
        .filter_map(move |(dx, dy)| {
            let (x, y) = (x + dx, y + dy);
            ((0..width).contains(&x) && (0..height).contains(&y)).then_some((x, y))
        })
}

/// A path search head that stores the accumulated cost so far and its position.
#[derive(Copy, Clone, Eq, PartialEq)]
struct Seeker {
    cost: i32,
    position: (i32, i32),
}

// Seekers get sorted by cost descending, so that the lowest cost gets popped off a max-heap
// priority queue.
impl Ord for Seeker {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
    }
}

impl PartialOrd for Seeker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Parses the puzzle input into a grid.
fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| (b - b'0') as i32).collect())
        .collect()
}

/// Parses the puzzle input into a fivefold expanded grid with incremented sectors.
fn parse_large(input: &str) -> Vec<Vec<i32>> {
    (0..5)
        .flat_map(|y_offset| {
            input.lines().map(move |line| {
                (0..5)
                    .flat_map(|x_offset| {
                        line.bytes()
                            .map(move |b| (1 + (b - b'0' - 1 + x_offset + y_offset) % 9) as i32)
                    })
                    .collect()
            })
        })
        .collect()
}
