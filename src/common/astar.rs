//! Implementation of the A* search algorithm.

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

/// Metadata associated with a node, tracked once per node.
struct Info<C> {
    is_closed: bool,
    best: C,
}

/// Value used for the `open` priority queue in the A* implementation, consisting of the node
/// itself as well as the cost to reach it. The same node can be reached multiple times with
/// different costs, so there can be multiple open list entries for the same one.
///
/// Sorted exclusively by `cost` ascending, for priority queue purposes.
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
pub trait Node<'a>: Clone + Eq + std::hash::Hash {
    type Cost;
    type Env;

    /// Produces all reachable neighbours.
    fn next(&self, env: &'a Self::Env) -> Box<dyn Iterator<Item = (Self, Self::Cost)> + 'a>;

    /// A* heuristic, how much might it take to be done.
    fn heuristic(&self, env: &'a Self::Env) -> Self::Cost;

    /// Checks if this node is a finished state.
    fn done(&self, env: &'a Self::Env) -> bool;
}

/// Finds the cost of the shortest path to a finished node, given some initial node. Does not
/// compute the actual path.
///
/// `env` is some read-only data passed to every method in the [`AStarNode`] implementation for the
/// node. Usually, this would be some map data or the like.
pub fn shortest_path_length<'a, N, C>(initial_node: N, env: &'a N::Env) -> Option<C>
where
    N: Node<'a, Cost = C> + 'a,
    C: Copy + Default + std::ops::Add<Output = C> + Eq + Ord,
{
    let mut opens = BinaryHeap::new();
    let mut infos = HashMap::new();

    opens.push(Open {
        node: initial_node.clone(),
        cost: initial_node.heuristic(env),
    });
    infos.insert(
        initial_node.clone(),
        Info {
            is_closed: false,
            best: C::default(),
        },
    );

    while let Some(open) = opens.pop() {
        // SAFETY: We always insert `Info`s for any `Open` that gets added.
        let info = infos.get_mut(&open.node).unwrap();
        if std::mem::replace(&mut info.is_closed, true) {
            continue;
        }

        if open.node.done(env) {
            return Some(info.best);
        }

        let cost = info.best;
        for (node, edge_cost) in open.node.next(env) {
            let cost = cost + edge_cost;
            let cost = match infos.get_mut(&node) {
                Some(target_info) => {
                    if target_info.is_closed || target_info.best <= cost {
                        continue;
                    }

                    target_info.best = cost;
                    cost
                }
                None => {
                    infos.insert(
                        node.clone(),
                        Info {
                            is_closed: false,
                            best: cost,
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
