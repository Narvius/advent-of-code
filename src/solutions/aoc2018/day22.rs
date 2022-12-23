use std::collections::{BinaryHeap, HashMap, VecDeque};

/// Find the total risk level for the smallest rectangle including the start
/// and target tiles.
pub fn one(input: &str) -> crate::Result<usize> {
    let mut map = parse(input).ok_or("parse failed")?;
    map.extend_to(map.target);
    Ok(map.data.into_iter().flatten().map(|v| v % 3).sum())
}

/// Find the length of the shortest path to the target tile.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut map = parse(input).ok_or("failed parse")?;
    // Used to limit the search space, no point going too far past the target along either axis.
    let max_search = (map.target.0 * 2, map.target.1 * 2);
    // A queue of points to floodfill from. Stored in a heap that prioritizes the point
    // closest to the target at all times.
    let mut fill_queue = BinaryHeap::from([QueuedPoint((0, 0), map.target, 1, 0)]);
    // One HashMap per tool that keeps track of which tiles were visited with that tool;
    // the value for an entry is how many steps it took to get there optimally.
    let mut visited: [HashMap<(usize, usize), usize>; 3] = std::array::from_fn(|_| HashMap::new());

    // The length of the shortest path found so far; used to prune searches that aren't going anywhere.
    let mut best = usize::MAX;

    while let Some(QueuedPoint(p, target, tool, steps)) = fill_queue.pop() {
        // If we're so far we couldn't possibly reach the target anymore in time, don't bother.
        if steps + p.0.abs_diff(target.0) + p.1.abs_diff(target.1) >= best {
            continue;
        }

        // Floodfill the area this point is a part of with the given tool.
        // Queue new fills for the edges of the area; mark the area as visited.
        let mut queue = VecDeque::from([(p, steps)]);
        while let Some((p, steps)) = queue.pop_front() {
            // If we've taken too long or this tile was already visited more efficiently, skip.
            if best <= steps || *visited[tool].get(&p).unwrap_or(&usize::MAX) <= steps {
                continue;
            }

            // If we're at the target, we're done!
            if p == map.target {
                best = steps;
                continue;
            }

            // Mark as visited.
            visited[tool].insert(p, steps);

            // Queue up neighbours.
            for n in neighbours(p, max_search) {
                if map.passable(n, tool) {
                    queue.push_back((n, steps + 1));
                } else {
                    // Edge of the floodfill. We know we have to switch tools; so we enqueue
                    // new floodfill areas for the one tool we can switch to (all pairs of
                    // terrains have one overlapping allowed tool).
                    let shared_tool = 3 - map.data[p.1][p.0] % 3 - map.data[n.1][n.0] % 3;
                    fill_queue.push(QueuedPoint(n, target, shared_tool, steps + 8));
                }
            }
        }
    }

    Ok(best)
}

/// Returns the orthogonal neighbours of the given point. Does not return neighbours
/// with negative coordinates, or with coordinates beyond the provided maximum.
fn neighbours((x, y): Point, (max_x, max_y): Point) -> impl Iterator<Item = Point> {
    [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .into_iter()
        .filter(move |&(dx, dy)| {
            (x, dx) != (0, -1)
                && (x, dx) != (max_x, 1)
                && (y, dy) != (0, -1)
                && (y, dy) != (max_y, 1)
        })
        .map(move |(dx, dy)| ((x as i32 + dx) as usize, (y as i32 + dy) as usize))
}

type Point = (usize, usize);

/// A queued-up point for processing. The four fields are the current position,
/// target position, current tool and steps taken so far.
///
/// Has a custom ordering where points further away from the target are less,
/// so that a [`BinaryHeap`] prioritizes points closer to the target.
#[derive(Eq, PartialEq)]
struct QueuedPoint(Point, Point, usize, usize);

impl Ord for QueuedPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Closer to target = higher score.
        fn score((x, y): Point, (tx, ty): Point) -> usize {
            usize::MAX - (x.abs_diff(tx) + y.abs_diff(ty))
        }

        score(self.0, self.1).cmp(&score(other.0, other.1))
    }
}

impl PartialOrd for QueuedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// A map of the underground level.
struct Map {
    target: Point,
    depth: usize,
    data: Vec<Vec<usize>>,
}

impl Map {
    /// Extends the map just enough that the provided point becomes a valid coordinate.
    fn extend_to(&mut self, (x, y): Point) {
        for y in 0..=y {
            if self.data.len() == y {
                self.data.push(vec![]);
            }
            for x in self.data[y].len()..=x {
                let geologic_index = match (x, y) {
                    _ if (x, y) == self.target => 0,
                    (0, 0) => 0,
                    (0, y) => y * 48271,
                    (x, 0) => x * 16807,
                    (x, y) => self.data[y - 1][x] * self.data[y][x - 1],
                };
                self.data[y].push((geologic_index + self.depth) % 20183);
            }
        }
    }

    /// Checks if a given tile is passable with the given `tool`. If necessary, extends
    /// the map.
    fn passable(&mut self, (x, y): Point, tool: usize) -> bool {
        self.extend_to((x, y));
        (self.data[y][x] % 3) != tool
    }
}

/// Constructs a [`Map`] from the puzzle input.
fn parse(input: &str) -> Option<Map> {
    let (depth, target) = input.split_once('\n')?;
    let (x, y) = target.strip_prefix("target: ")?.trim().split_once(',')?;

    Some(Map {
        target: (x.parse().ok()?, y.parse().ok()?),
        depth: depth.strip_prefix("depth: ")?.parse().ok()?,
        data: vec![vec![]],
    })
}
