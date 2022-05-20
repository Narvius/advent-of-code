use std::{
    cmp::min_by_key,
    collections::{HashMap, HashSet, VecDeque},
};

/// Find the root of the tree described by the puzzle input.
pub fn one(input: &str) -> Result<String, String> {
    let mut queue = VecDeque::from_iter(parse(input));
    let mut top = HashSet::new();

    while let Some((name, weight, next)) = queue.pop_front() {
        if !next.iter().all(|&s| top.contains(s)) {
            queue.push_back((name, weight, next));
            continue;
        }

        for s in next {
            top.remove(s);
        }

        top.insert(name);
    }

    top.into_iter()
        .next()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("no bottom disc found"))
}

/// Find the single incorrect value in the tree and find what it should be corrected to. A value
/// is incorrect if it causes the tree to become unbalanced.
pub fn two(input: &str) -> Result<String, String> {
    let tree = HashMap::from_iter(parse(input).map(|(name, weight, next)| (name, (weight, next))));
    let deepest_unbalanced = tree
        .keys()
        .filter_map(|&node| get_unbalance(node, &tree))
        .reduce(|a, b| min_by_key(a, b, |t| t.1));

    if let Some((target, current, offender)) = deepest_unbalanced {
        Ok((tree[offender].0 + target - current).to_string())
    } else {
        Err(format!("no result"))
    }
}

/// Maps a node label to the corresponding weight and children.
type Tree<'a> = HashMap<&'a str, (usize, Vec<&'a str>)>;

/// If the node is unbalanced, returns `Some((target weight, deviant weight, deviant child))`,
/// `None` otherwise.
fn get_unbalance<'a>(node: &str, tree: &'a Tree) -> Option<(usize, usize, &'a str)> {
    // Gets the total weight of a node, including its children.
    fn weight(node: &str, tree: &Tree) -> usize {
        let (mut sum, ref children) = tree[node];
        for child in children {
            sum += weight(child, tree);
        }
        sum
    }

    let mut weights: Vec<_> = tree[node].1.iter().map(|&s| (s, weight(s, tree))).collect();
    weights.sort_by_cached_key(|p| p.1);
    weights.windows(2).next().and_then(|w| {
        if w[0].1 == w[1].1 {
            let (ls, l) = *weights.last()?;
            (w[0].1 != l).then(|| (w[0].1, l, ls))
        } else {
            Some((w[1].1, w[0].1, w[0].0))
        }
    })
}

/// Parses the puzzle input into tuples containing the name, weight and children of each node
/// in the tree.
fn parse(input: &str) -> impl Iterator<Item = (&str, usize, Vec<&str>)> {
    input.lines().filter_map(|line| {
        let (head, next) = if let Some((head, next)) = line.split_once(" -> ") {
            (head, Some(next))
        } else {
            (line, None)
        };
        let (name, weight) = head.split_once(" ")?;
        Some((
            name,
            weight.trim_matches(&['(', ')'][..]).parse().ok()?,
            next.map(|s| s.split(", ").collect())
                .unwrap_or_else(|| vec![]),
        ))
    })
}
