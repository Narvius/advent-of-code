use std::collections::{HashMap, HashSet, VecDeque};

/// Find the shortest trip possible when visiting all goal positions.
pub fn one(input: &str) -> crate::Result<String> {
    best_trip_distance(input, false)
}

/// Find the shortest trip possible when visiting all goal positions and then returning to the
/// starting position.
pub fn two(input: &str) -> crate::Result<String> {
    best_trip_distance(input, true)
}

/// Finds the shortest possible route for the cleaning robot, as per the puzzle input. If
/// `include_return` is set, this route will include a return trip to the original starting
/// position.
fn best_trip_distance(input: &str, include_return: bool) -> crate::Result<String> {
    let maze = parse(input);
    let mut best = usize::MAX;

    // Construct dijkstra maps for each relevant position.
    let start_map = dijkstra_map_for(&maze, maze.position);
    let maps: Vec<_> = maze
        .goals
        .iter()
        .copied()
        .map(|p| dijkstra_map_for(&maze, p))
        .collect();

    // Use the cached dijkstra map information to quickly find the length of each possible
    // trip, and get the shortest one.
    for order in permutations(maze.goals.len()) {
        let mut result = start_map[&maze.goals[order[0]]];
        for w in order.windows(2) {
            result += maps[w[0]][&maze.goals[w[1]]];
        }

        if include_return {
            result += maps[order[order.len() - 1]][&maze.position];
        }

        best = best.min(result);
    }

    Ok(best.to_string())
}

/// Build a "dijkstra map" for the entire `maze` from a given `start`; that is, a mapping from
/// each position to how many steps it takes to reach it from the `start`.
fn dijkstra_map_for(maze: &Maze, start: (usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut map = HashMap::from([(start, 0)]);
    let mut candidates = VecDeque::from([start]);

    while let Some((x, y)) = candidates.pop_front() {
        for p in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
            if !map.contains_key(&p) && maze.floor.contains(&p) {
                candidates.push_back(p);
                map.insert(p, map[&(x, y)] + 1);
            }
        }
    }

    map
}

/// Returns all possible permutations of the numbers in `0..k`, using Heap's algorithm.
fn permutations(k: usize) -> Vec<Vec<usize>> {
    fn inner(k: usize, values: &mut [usize]) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        if k <= 1 {
            result.push(Vec::from(values));
        } else {
            result.extend(inner(k - 1, values));
            for i in 0..(k - 1) {
                if k % 2 == 0 {
                    values.swap(i, k - 1);
                } else {
                    values.swap(0, k - 1);
                }
                result.extend(inner(k - 1, values));
            }
        }
        result
    }

    inner(k, &mut (0..k).collect::<Vec<_>>())
}

/// Contains information about walkable tiles, as well as the starting and goal positions.
struct Maze {
    floor: HashSet<(usize, usize)>,
    position: (usize, usize),
    goals: Vec<(usize, usize)>,
}

/// Parses the maze and relevant position information from the puzzle input.
fn parse(input: &str) -> Maze {
    let mut position = (0, 0);
    let mut goals = vec![];
    let mut floor = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            match c {
                '.' => {}
                '0' => position = (x, y),
                _ if c.is_numeric() => goals.push((x, y)),
                _ => continue,
            }
            floor.insert((x, y));
        }
    }

    Maze {
        floor,
        position,
        goals,
    }
}
