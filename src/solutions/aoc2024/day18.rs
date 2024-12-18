use std::collections::HashSet;

use crate::common::{astar, CARDINAL};

/// Find the shortest path through the maze formed by the first 1024 input obstacles.
pub fn one(input: &str) -> crate::Result<i32> {
    let obstacles = parse(input, 1024);
    astar::shortest_path_length(V2(0, 0), &obstacles).ok_or("no path".into())
}

/// Find the indices of the first obstacle that would block the exit.
pub fn two(input: &str) -> crate::Result<&str> {
    // Do a binary search; but instead of looking for a specific element, we're running it to find
    // the boundary between unblocked and blocked runs.
    let (mut l, mut r) = (0, input.lines().count());
    let mut result = None;

    while l < r {
        let m = (l + r) / 2;

        let obstacles = parse(input, m);
        result = astar::shortest_path_length(V2(0, 0), &obstacles);

        match result {
            Some(_) => l = m + 1,
            None => r = m - 1,
        }
    }

    // Pick final index depending on whether the binary search ended up on the "unblocked" or
    // "blocked" side of the boundary.
    let index = match result {
        Some(_) => l,
        None => l - 1,
    };

    // Funnily enough, we don't actually need to create an owned string here.
    Ok(input.lines().nth(index).unwrap())
}

/// 2D coordinates; implements [`astar::Node`].
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct V2(i32, i32);

impl<'a> astar::Node<'a> for V2 {
    type Cost = i32;
    type Env = HashSet<(i32, i32)>;

    fn next(&self, env: &'a Self::Env) -> Box<dyn Iterator<Item = (Self, Self::Cost)> + 'a> {
        Box::new(
            CARDINAL
                .map(|d| (self.0, self.1) + d)
                .into_iter()
                .filter(|p| (0..=70).contains(&p.0) && (0..=70).contains(&p.1) && !env.contains(p))
                .map(|(x, y)| (V2(x, y), 1)),
        )
    }

    fn heuristic(&self, _: &Self::Env) -> Self::Cost {
        (70 - self.0) + (70 - self.1)
    }

    fn done(&self, _: &Self::Env) -> bool {
        self == &V2(70, 70)
    }
}

/// Parses the first `count` lines of puzzle input into a set of obstacles.
fn parse(input: &str, count: usize) -> HashSet<(i32, i32)> {
    input
        .lines()
        .filter_map(|line| {
            let (x, y) = line.split_once(',')?;
            Some((x.parse().ok()?, y.parse().ok()?))
        })
        .take(count)
        .collect()
}
