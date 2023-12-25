use std::collections::{HashMap, HashSet};

/// Split the graph into two sub-graphs connected by three connections. Multiply their sizes.
pub fn one(input: &str) -> crate::Result<usize> {
    // Concept: Start from any arbitrary node. Keep track of the nodes we absorbed, and the border
    // of that blob. The "border" are all nodes *not* in our blob, directly reachable from
    // somewhere in the blob.
    //
    // At each step expand the blob by 1 node, such that the border grows by the least amount
    // possible.
    //
    // Once we reach 3 nodes in the border, we have our solution.

    let g = parse(input).ok_or("no parse")?;
    let mut blob = HashSet::from([0]);
    let mut border: HashSet<usize> = HashSet::from_iter(g[0].iter().copied());

    while border.len() > 3 {
        let item = *border
            .iter()
            .min_by_key(|&&n| g[n].iter().filter(|conn| !blob.contains(conn)).count())
            .ok_or("unreachable")?;

        border.remove(&item);
        blob.insert(item);
        for &conn in g[item].iter() {
            if !blob.contains(&conn) {
                border.insert(conn);
            }
        }
    }

    Ok((g.len() - blob.len()) * blob.len())
}

/// Freebie!
pub fn two(_input: &str) -> crate::Result<&str> {
    Ok("done!")
}

/// Parses the puzzle input into a graph represented as a list of lists, where `graph[n]` is the
/// list of nodes reachable from node `n`.
fn parse(input: &str) -> Option<Vec<Vec<usize>>> {
    let mut nodes: HashMap<&str, usize> = HashMap::new();
    let mut graph: Vec<Vec<usize>> = vec![];

    for line in input.lines() {
        let (src, tgts) = line.split_once(": ")?;
        let from = *nodes.entry(src).or_insert_with(|| {
            graph.push(vec![]);
            graph.len() - 1
        });
        for tgt in tgts.split_whitespace() {
            let to = *nodes.entry(tgt).or_insert_with(|| {
                graph.push(vec![]);
                graph.len() - 1
            });
            graph[from].push(to);
            graph[to].push(from);
        }
    }

    Some(graph)
}
