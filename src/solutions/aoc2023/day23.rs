use std::collections::{HashMap, HashSet};

/// Find the length o the longest path, without repeating any tiles.
pub fn one(input: &str) -> crate::Result<usize> {
    longest_to_end(input, false).ok_or("no result".into())
}

/// Find the length of the longest path, without repeating any tiles, ignoring slopes.
pub fn two(input: &str) -> crate::Result<usize> {
    longest_to_end(input, true).ok_or("no result".into())
}

const DELTAS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
type Graph = Vec<Vec<(usize, usize)>>;

/// Returns the longest path to the end. If `backwards` is set, slopes are ignored.
fn longest_to_end(input: &str, backwards: bool) -> Option<usize> {
    fn work(g: &Graph, from: usize, used: u64) -> Option<usize> {
        if from == g.len() {
            return Some(0);
        }

        let mut best = None;
        for &(next, steps) in &g[from] {
            if next == usize::MAX {
                return Some(steps);
            }
            if used & (1 << next) > 0 {
                continue;
            }

            best = best.max(work(g, next, used | (1 << next)).map(|n| n + steps));
        }

        best
    }

    let g = build_graph(input, backwards);
    work(&g, 0, 1)
}

/// Constructs a graph from the forest map, represented as a list of lists. `graph[i]` is the entry
/// for the `i`th node; the entry contains a list of destination nodes, alongside the amount of
/// steps required to reach them.
///
/// `graph[0]` is the starting node (top left), and the final node is index `usize::MAX`.
fn build_graph(input: &str, backwards: bool) -> Graph {
    let map: Vec<_> = input.lines().map(str::as_bytes).collect();

    let (node, steps) = corridor_length(&map, (1, 0));
    let mut nodes = HashMap::from([((1, 0), 0), (node, 1)]);
    let mut graph = vec![vec![(1, steps)], vec![]];
    let (mut open, mut seen) = (vec![node], HashSet::from([(1, 0)]));

    if backwards {
        graph[1].push((0, steps));
    }

    while let Some((x, y)) = open.pop() {
        if !seen.insert((x, y)) || y as usize == map.len() - 1 {
            continue;
        }

        #[allow(clippy::map_entry)]
        if !nodes.contains_key(&(x, y)) {
            nodes.insert((x, y), nodes.len());
            graph.push(vec![]);
        }

        for (dx, dy) in DELTAS {
            let (Ok(nx), Ok(ny)) = (usize::try_from(x + dx), usize::try_from(y + dy)) else {
                continue;
            };
            let go_from = match (map[ny][nx], dx, dy) {
                (b'<', -1, 0) => Some((x - 2, y)),
                (b'^', 0, -1) => Some((x, y - 2)),
                (b'>', 1, 0) => Some((x + 2, y)),
                (b'v', 0, 1) => Some((x, y + 2)),
                _ => None,
            };

            if let Some(go_from) = go_from {
                let (new_pos, steps) = corridor_length(&map, go_from);

                if new_pos.1 as usize == map.len() - 1 {
                    graph[nodes[&(x, y)]].push((usize::MAX, steps + 2));
                    continue;
                }

                #[allow(clippy::map_entry)]
                if !nodes.contains_key(&new_pos) {
                    nodes.insert(new_pos, nodes.len());
                    graph.push(vec![]);
                }

                open.push(new_pos);
                graph[nodes[&(x, y)]].push((nodes[&new_pos], steps + 2));

                if backwards {
                    graph[nodes[&new_pos]].push((nodes[&(x, y)], steps + 2));
                }
            }
        }
    }

    graph
}

/// Starting at the first tile of a corridor, finds the total length of that corridor, and the
/// coordinate of the node it ends on.
fn corridor_length(map: &[&[u8]], (mut x, mut y): (i32, i32)) -> ((i32, i32), usize) {
    let mut len = 0;
    let mut forbidden = None;

    loop {
        for (dx, dy) in DELTAS {
            let (Ok(nx), Ok(ny)) = (usize::try_from(x + dx), usize::try_from(y + dy)) else {
                continue;
            };
            if ny >= map.len() || Some((dx, dy)) == forbidden {
                continue;
            }

            if ny == map.len() - 1 {
                return ((nx as i32, ny as i32), len + 1);
            }

            if len > 0 {
                match map[ny][nx] {
                    b'<' => return ((nx as i32 - 1, ny as i32), len + 2),
                    b'^' => return ((nx as i32, ny as i32 - 1), len + 2),
                    b'>' => return ((nx as i32 + 1, ny as i32), len + 2),
                    b'v' => return ((nx as i32, ny as i32 + 1), len + 2),
                    _ => {}
                }
            }

            if map[ny][nx] == b'.' {
                (x, y) = (nx as i32, ny as i32);
                forbidden = Some((-dx, -dy));
                len += 1;
                break;
            }
        }
    }
}
