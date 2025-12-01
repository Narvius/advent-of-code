use std::collections::{HashMap, HashSet};

/// Find the number of triangles in the graph that contain a node starting with 't'.
pub fn one(input: &str) -> crate::Result<usize> {
    let mut sets = HashSet::new();
    let net = parse(input);
    for &k1 in net.keys().filter(|k| k.starts_with('t')) {
        for &k2 in &net[k1] {
            for &k3 in &net[k2] {
                if net[k3].contains(k1) {
                    let mut set = [k1, k2, k3];
                    set.sort();
                    sets.insert(set);
                }
            }
        }
    }
    Ok(sets.len())
}

/// Find the largest connected subgraph, and list its nodes in sorted order.
pub fn two(input: &str) -> crate::Result<String> {
    let net = parse(input);
    let conns: HashSet<(&str, &str)> = input
        .lines()
        .filter_map(|line| line.split_once('-'))
        .flat_map(|(a, b)| [(a, b), (b, a)])
        .collect();
    let mut cliques: Vec<HashSet<&str>> = vec![];

    for &key in net.keys() {
        // Add the node to every clique it is fully connected to.
        let mut new_cliques = vec![];
        for clique in &mut cliques {
            if clique.iter().all(|&node| conns.contains(&(node, key))) {
                new_cliques.push(clique.clone());
                new_cliques.last_mut().unwrap().insert(key);
            }
        }
        cliques.extend(new_cliques);

        // Also consider the node its own clique, as it may be the start of a clique that doesn't
        // contain any of the previous nodes.
        cliques.push(HashSet::from([key]));

        // Originally, I had an algorithm that, instead of adding clones of each clique to the
        // list, would simply add the node to any existing clique in-place. This *happened* to work
        // for my input; but I'm pretty sure there's constellation where it wouldn't have. Adding a
        // node to a clique locks out a whole bunch of other nodes, so adding any given node may
        // lock a clique out of becoming its largest version.
        //
        // So even though I already had a solution, I rewrote it to this, where I always consider
        // versions of every clique with and without the new node. Unfortunately, it is way
        // slower--but I'd feel bad ending on a solution that I know only worked accidentally.
        //
        // There's something called the "Bron-Kerbosch algorithm" that solves the problem of
        // finding the largest clique (fully-connected subgraph--that's where I got the term
        // "clique" from for this). Implementing it instead would presumably result in faster code,
        // compared to my jury-rigged algo; but I'm done for now.
    }

    // Adapt the largest clique to the desired output format.
    let clique = cliques
        .into_iter()
        .max_by_key(|c| c.len())
        .ok_or("no result")?;

    let mut clique: Vec<_> = clique.into_iter().collect();
    clique.sort();
    Ok(clique.join(","))
}

type Network<'a> = HashMap<&'a str, HashSet<&'a str>>;

/// Parses the puzzle input into a graph in the form of a hash map of node to hash set of nodes it
/// is connected to.
fn parse(input: &str) -> Network<'_> {
    (input.lines())
        .filter_map(|s| s.split_once('-'))
        .fold(Network::new(), |mut net, (a, b)| {
            net.entry(a).or_default().insert(b);
            net.entry(b).or_default().insert(a);
            net
        })
}
