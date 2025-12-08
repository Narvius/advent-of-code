use std::collections::{BTreeMap, HashSet};

/// Do the 1000 shortest connections, and return the product of the sizes of the three largest
/// circuits as a checksum.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(connect_circuits(input, false))
}

/// Connects as many junction boxes as necessary to combine them all into one circuit; return the
/// product of the X coordinates of the last two connected boxes as a checksum.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(connect_circuits(input, true))
}

/// Performs the puzzle logic, connecting enough junction boxes together to complete part `one`
/// (`fully_connect` = false) or part `two` (`fully_connect` = true), and computing the relevant
/// checksum to go with it.
fn connect_circuits(input: &str, fully_connect: bool) -> usize {
    let boxes: Vec<_> = parse(input).collect();
    let mut distances = BTreeMap::new();
    let mut cliques: Vec<HashSet<usize>> = vec![];

    // Calculate the distances for each pair of boxes; storing the pairs sorted by said distance in
    // the `distances` map.
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let [x1, y1, z1] = boxes[i];
            let [x2, y2, z2] = boxes[j];
            // Since we're not using the distance in calculations, but merely as a comparison, we
            // don't need to square root it.
            let d = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);
            distances.insert(d, (i, j));
        }
    }

    // Part `one`.
    for _ in 0..1000 {
        connect_two_shortest(&mut distances, &mut cliques);
    }

    if !fully_connect {
        cliques.sort_unstable_by_key(|c| c.len());
        return cliques[cliques.len() - 3..]
            .iter()
            .map(|c| c.len())
            .product();
    }

    // Part `two`.
    let (mut i, mut j) = (0, 0);

    while cliques.len() > 1 || cliques[0].len() != boxes.len() {
        (i, j) = connect_two_shortest(&mut distances, &mut cliques);
    }

    (boxes[i][0] * boxes[j][0]) as usize
}

/// Connects the two closest-together boxes, and returns their indices.
///
/// Most of the work is already done before this function ever gets called; `distances` contains
/// all distances between boxes, and allows efficient retrieval of the shortest one, alongside the
/// pair it is between.
///
/// Each `clique` in `cliques` is one "circuit" as described in the puzzle; this function modifies
/// it by adding the closest connection from `distances`, merging cliques as necessary.
fn connect_two_shortest(
    distances: &mut BTreeMap<i64, (usize, usize)>,
    cliques: &mut Vec<HashSet<usize>>,
) -> (usize, usize) {
    let (_, (i, j)) = distances.pop_first().unwrap();

    let ci = (0..cliques.len()).find(|&ci| cliques[ci].contains(&i));
    let cj = (0..cliques.len()).find(|&cj| cliques[cj].contains(&j));

    match (ci, cj) {
        (Some(ci), Some(cj)) if ci == cj => {}
        (Some(ci), Some(cj)) => {
            let clique = cliques.remove(ci.max(cj));
            cliques[ci.min(cj)].extend(clique);
        }
        (Some(ci), None) => {
            cliques[ci].insert(j);
        }
        (None, Some(cj)) => {
            cliques[cj].insert(i);
        }
        (None, None) => cliques.push(HashSet::from([i, j])),
    }

    (i, j)
}

/// Parses the puzzle input into a series of coordinate triplets.
fn parse(input: &str) -> impl Iterator<Item = [i64; 3]> + '_ {
    input.lines().filter_map(|line| {
        let (x, yz) = line.split_once(',')?;
        let (y, z) = yz.split_once(',')?;
        Some([x, y, z].map(|v| v.parse().unwrap()))
    })
}
