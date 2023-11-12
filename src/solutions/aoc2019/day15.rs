use std::collections::{HashMap, VecDeque};

use crate::common::intcode::v2::*;

/// Explore the maze, find the number of steps required to reach the goal tile.
pub fn one(input: &str) -> crate::Result<usize> {
    let (map, goal) = flood_fill(Program::new(input)?)?;
    Ok(map[&goal])
}

/// Find the number of steps to reach the furthest tile from the original goal tile.
pub fn two(input: &str) -> crate::Result<usize> {
    let (mut map, goal) = flood_fill(Program::new(input)?)?;
    recenter(&mut map, goal);
    map.into_values().max().ok_or_else(|| "no result".into())
}

type Point = (i32, i32);

/// Given a [`flood_fill`] map, recalculates the distances so they're the distance from `start`.
fn recenter(map: &mut HashMap<Point, usize>, start: Point) {
    for v in map.values_mut() {
        *v = usize::MAX;
    }
    *map.entry(start).or_default() = 0;

    let mut queue = VecDeque::from([(start, 0)]);
    while let Some(((x, y), depth)) = queue.pop_front() {
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let p = (x + dx, y + dy);
            if map.contains_key(&p) && map[&p] > depth + 1 {
                *map.entry(p).or_default() = depth + 1;
                queue.push_back((p, depth + 1));
            }
        }
    }
}

/// Explores the entire map stored in the program, returning it as a map from coordinates to
/// their distance from the entrace; alongside the coordinates of the goal tile.
fn flood_fill(p: Program) -> crate::Result<(HashMap<Point, usize>, Point)> {
    let mut queue = VecDeque::from([(p, (0, 0), 0)]);
    let mut visited = HashMap::new();
    let mut goal = (0, 0);

    while let Some((p, (x, y), depth)) = queue.pop_front() {
        for (dx, dy, dir) in [(0, -1, 1), (0, 1, 2), (-1, 0, 3), (1, 0, 4)] {
            let (x, y) = (x + dx, y + dy);
            if depth + 1 < *visited.get(&(x, y)).unwrap_or(&usize::MAX) {
                let mut p = p.clone();
                p.run_with([dir])?;
                match p.output.pop_front() {
                    Some(1) => {
                        queue.push_back((p, (x, y), depth + 1));
                        *visited.entry((x, y)).or_default() = depth + 1;
                    }
                    Some(2) => {
                        goal = (x, y);
                        queue.push_back((p, (x, y), depth + 1));
                        *visited.entry((x, y)).or_default() = depth + 1;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok((visited, goal))
}
