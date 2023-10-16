use std::collections::HashMap;

/// Find the total number of orbits in the system.
pub fn one(input: &str) -> crate::Result<usize> {
    let mut map = HashMap::new();

    for (pre, post) in input.lines().filter_map(|s| s.split_once(')')) {
        map.entry(pre).or_insert(vec![]).push(post);
    }

    fn score(map: &HashMap<&str, Vec<&str>>, key: &str, depth: usize) -> usize {
        match map.get(key) {
            Some(v) => depth + v.iter().map(|k| score(map, k, depth + 1)).sum::<usize>(),
            None => depth,
        }
    }

    Ok(score(&map, "COM", 0))
}

/// Find the number of jumps between the objects orbited by "YOU" and "SAN".
///
/// Does this by finding the closest common ancestor of both nodes, and summing the jumps
/// required to reach it from either side.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut map = HashMap::new();

    for (pre, post) in input.lines().filter_map(|s| s.split_once(')')) {
        map.entry(post).or_insert(pre);
    }

    fn ancestors<'a>(
        map: &'a HashMap<&'a str, &'a str>,
        node: &'a str,
    ) -> impl Iterator<Item = (&'a str, usize)> {
        std::iter::successors(Some((node, 0)), move |(n, d)| {
            map.get(n).map(|&n| (n, d + 1))
        })
    }

    for (a1, n1) in ancestors(&map, map["SAN"]) {
        if let Some((_, n2)) = ancestors(&map, map["YOU"]).find(|&(a2, _)| a1 == a2) {
            return Ok(n1 + n2);
        }
    }

    Err("no solution in data set".into())
}
