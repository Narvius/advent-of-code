use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::common::{Dir, Grid, CARDINAL};

/// Find the length of the shortest path through the maze.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(analyze_maze(input, false))
}

/// Find the number of tiles involved in any equivalently-shortest path through the maze.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(analyze_maze(input, true))
}

/// A priority queue entry. The first two elements is the position + direction pair used as a key,
/// the second element is the cost we sort priority by.
#[derive(Eq, Hash, PartialEq)]
struct Entry((V2, Dir), i32);

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.1.cmp(&self.1)).then_with(|| self.0.cmp(&other.0))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Analyzes the input maze, and returns the length of the shortest path (if `used_paths` is false),
// or the amount of tiles that lie on any equivalently-best path (if `used_paths` is true).
fn analyze_maze(input: &str, used_paths: bool) -> usize {
    let (g, start, end) = parse(input);

    // `visited` is a map of (position, direction) => (cost to reach that, parents);
    // where `parents` means all previous positions that can reach it with the same speed. This is
    // necessary for detecting all possible paths.
    let mut queue = BinaryHeap::from([Entry((start, Dir::E), 0)]);
    let mut visited = HashMap::from([((start, Dir::E), (0, vec![]))]);
    let mut global_best = i32::MAX;

    // Breadth-first search, but with the twist that we use a priority queue to account for turning
    // being 1000x slower than walking foward.
    while let Some(Entry((position, dir), cost)) = queue.pop() {
        if g.at(position) == Some(b'#') {
            continue;
        }

        if position == end || cost > global_best {
            global_best = global_best.min(cost);
            continue;
        }

        let options = [
            ((position + dir, dir), cost + 1),
            ((position + dir.left(), dir.left()), cost + 1001),
            ((position + dir.right(), dir.right()), cost + 1001),
        ];

        for (key, cost) in options {
            let (best, parents) = visited.entry(key).or_insert((i32::MAX, vec![]));
            match cost.cmp(best) {
                Ordering::Less => {
                    *best = cost;
                    parents.clear();
                    parents.push((position, dir));
                }
                Ordering::Equal => {
                    if !parents.contains(&(position, dir)) {
                        parents.push((position, dir));
                    }
                }
                Ordering::Greater => continue,
            }

            queue.push(Entry(key, cost));
        }
    }

    if !used_paths {
        return global_best as usize;
    }

    // Walk backwards through the `parents` chains in `visited` (it's )
    let mut best_path_tiles = HashSet::new();
    let mut stack: Vec<_> = CARDINAL
        .into_iter()
        .filter_map(|d| visited.get(&(end, d)))
        .filter(|v| v.0 == global_best)
        .flat_map(|v| v.1.iter().copied())
        .collect();

    while let Some(key) = stack.pop() {
        best_path_tiles.insert(key.0);
        stack.extend(visited[&key].1.iter().copied());
    }

    // +1 to account for the end tile itself.
    1 + best_path_tiles.len()
}

type V2 = (i32, i32);

/// Parses the puzzle input into a graph of nodes, and a pair of indices into the graph (of the
/// start and end nodes, respectively).
fn parse(input: &str) -> (Grid<'_, u8>, V2, V2) {
    let grid = Grid::from_input(input);
    let (start, end) = (grid.find(|&e| e == b'S'), grid.find(|&e| e == b'E'));

    (grid, start.expect("a start"), end.expect("an end"))
}
