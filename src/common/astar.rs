//! Implementation of the A* search algorithm.

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

type Cost = i32;

/// Metadata associated with a node.
struct Meta {
    is_closed: bool,
    path: Cost,
}

/// A node in the `open` list.
/// Sorted by exclusively by `cost`, making it useful in a priority queue.
struct Open<N> {
    cost: Cost,
    node: N,
}

impl<N> PartialEq for Open<N> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<N> Eq for Open<N> {}

impl<N> PartialOrd for Open<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N> Ord for Open<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

/// Returns the lowest cost to reach a `done` state, given an `initial_node`,
/// a `heuristic` and a function that produces `next` available nodes for
/// a given node. Does not produce the actual path, just the cost.
pub fn shortest_path_length<N, I, Next, Heuristic, Done>(
    initial_node: N,
    mut next: Next,
    mut heuristic: Heuristic,
    mut done: Done,
) -> Option<Cost>
where
    N: Clone + Eq + std::hash::Hash,
    Next: FnMut(&N) -> I,
    Heuristic: FnMut(&N) -> Cost,
    Done: FnMut(&N) -> bool,
    I: Iterator<Item = (N, Cost)>,
{
    let mut opens = BinaryHeap::new();
    let mut metas = HashMap::new();

    opens.push(Open {
        cost: heuristic(&initial_node),
        node: initial_node.clone(),
    });
    metas.insert(
        initial_node.clone(),
        Meta {
            is_closed: false,
            path: 0,
        },
    );

    while let Some(open) = opens.pop() {
        // SAFETY: We always insert `Meta`s for any `Open` that gets added.
        let meta = metas.get_mut(&open.node).unwrap();
        if std::mem::replace(&mut meta.is_closed, true) {
            continue;
        }

        if done(&open.node) {
            return Some(meta.path);
        }

        let cost = meta.path;
        for (node, edge_cost) in next(&open.node) {
            let cost = cost + edge_cost;
            let cost = match metas.get_mut(&node) {
                Some(target_meta) => {
                    if target_meta.is_closed || target_meta.path <= cost {
                        continue;
                    }

                    target_meta.path = cost;
                    cost
                }
                None => {
                    metas.insert(
                        node.clone(),
                        Meta {
                            is_closed: false,
                            path: cost,
                        },
                    );
                    cost + heuristic(&node)
                }
            };
            opens.push(Open { node, cost });
        }
    }

    None
}
