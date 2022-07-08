use std::collections::HashMap;

/// Find the number of nodes connected (directly or indirectly) to node 0.
pub fn one(input: &str) -> crate::Result<usize> {
    let mut nodes = parse(input);
    Ok(consume_group(0, &mut nodes))
}

/// Count the number of disjoint subgraphs in the input graph.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut nodes = parse(input);
    let mut groups = 0;

    while let Some(&key) = nodes.keys().next() {
        groups += 1;
        consume_group(key, &mut nodes);
    }

    Ok(groups)
}

/// Given a node ID, removes that node and all nodes connected to it (directly or indirectly)
/// from the graph, then returns the number of nodes removed.
fn consume_group(node: usize, nodes: &mut HashMap<usize, Vec<usize>>) -> usize {
    let mut count = 0;
    if let Some(mut targets) = nodes.remove(&node) {
        count = 1;
        while let Some(target) = targets.pop() {
            if let Some(new_targets) = nodes.remove(&target) {
                targets.extend(new_targets);
                count += 1;
            }
        }
    }
    count
}

/// Parses the puzzle input into a map representing the graph.
fn parse(input: &str) -> HashMap<usize, Vec<usize>> {
    HashMap::from_iter(input.lines().filter_map(|line| {
        let (source, targets) = line.split_once(" <-> ")?;
        Some((
            source.parse().ok()?,
            targets
                .split(", ")
                .map(|t| t.parse().ok())
                .collect::<Option<_>>()?,
        ))
    }))
}
