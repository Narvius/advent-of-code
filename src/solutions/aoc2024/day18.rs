use std::{cmp::Ordering, collections::HashSet};

use crate::common::{astar, binary_search, CARDINAL};

/// Find the shortest path through the maze formed by the first 1024 input obstacles.
pub fn one(input: &str) -> crate::Result<i32> {
    let obstacles = parse(input, 1024);
    astar::shortest_path_length(V2(0, 0), &obstacles).ok_or("no path".into())
}

/// Find the coordinates of the first obstacle that would block the exit.
pub fn two(input: &str) -> crate::Result<&str> {
    // Use a binary search to find the exact amount of input obstacles required to first block.
    let obstacle_count = binary_search((0, input.lines().count()), |m| {
        match astar::shortest_path_length(V2(0, 0), &parse(input, m)) {
            Some(_) => Ordering::Less, // There's a path, so we need to add more obstacles.
            None => Ordering::Greater, // There's no path, so we need to remove obstacles.
        }
    });

    Ok(input.lines().nth(obstacle_count.unwrap_err() - 1).unwrap())
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
