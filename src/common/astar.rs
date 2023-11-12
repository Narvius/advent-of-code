//! Implementation of the A* search algorithm.

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

/// Metadata associated with a node.
struct Meta<C> {
    is_closed: bool,
    path: C,
}

/// A node in the `open` list.
/// Sorted by exclusively by `cost`, making it useful in a priority queue.
struct Open<N, C: Eq + Ord> {
    node: N,
    cost: C,
}

impl<N, C: Eq + Ord> PartialEq for Open<N, C> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<N, C: Eq + Ord> Eq for Open<N, C> {}

impl<N, C: Ord> PartialOrd for Open<N, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N, C: Ord> Ord for Open<N, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

/// A value that can be used as a node in an A* search.
pub trait AStarNode: Clone + Eq + std::hash::Hash {
    type Cost;
    type Env;

    /// Produces all reachable neighbours.
    fn next<'a>(&'a self, env: &'a Self::Env) -> Box<dyn Iterator<Item = (Self, Self::Cost)> + 'a>;

    /// A* heuristic, how much might it take to be done.
    fn heuristic(&self, env: &Self::Env) -> Self::Cost;

    /// Checks if this node is a finished state.
    fn done(&self, env: &Self::Env) -> bool;
}

/// Finds the shortest
pub fn shortest_path_length<N, C>(initial_node: N, env: &N::Env) -> Option<C>
where
    N: AStarNode<Cost = C>,
    C: Copy + Default + std::ops::Add<Output = C> + Eq + Ord,
{
    let mut opens = BinaryHeap::new();
    let mut metas = HashMap::new();

    opens.push(Open {
        node: initial_node.clone(),
        cost: initial_node.heuristic(env),
    });
    metas.insert(
        initial_node.clone(),
        Meta {
            is_closed: false,
            path: C::default(),
        },
    );

    while let Some(open) = opens.pop() {
        // SAFETY: We always insert `Meta`s for any `Open` that gets added.
        let meta = metas.get_mut(&open.node).unwrap();
        if std::mem::replace(&mut meta.is_closed, true) {
            continue;
        }

        if open.node.done(env) {
            return Some(meta.path);
        }

        let cost = meta.path;
        for (node, edge_cost) in open.node.next(env) {
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
                    cost + node.heuristic(env)
                }
            };
            opens.push(Open { node, cost });
        }
    }

    None
}
