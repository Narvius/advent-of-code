use std::collections::HashMap;

/// Find the shortest possible route that visits all cities.
pub fn one(input: &str) -> crate::Result<usize> {
    let (locations, distances) = parse(input)?;
    all_route_lengths(locations, distances)
        .min()
        .ok_or_else(|| "no input locations".into())
}

/// Find the longest possible route that visits all cities.
pub fn two(input: &str) -> crate::Result<usize> {
    let (locations, distances) = parse(input)?;
    all_route_lengths(locations, distances)
        .max()
        .ok_or_else(|| "no input locations".into())
}

type Distances = HashMap<(usize, usize), usize>;

/// Returns the lengths of all possible routes.
fn all_route_lengths(locations: usize, distances: Distances) -> impl Iterator<Item = usize> {
    // Calculates the length of a route.
    fn route_length(order: &[usize], distances: &Distances) -> usize {
        order
            .windows(2)
            .map(|w| distances.get(&(w[0], w[1])).unwrap_or(&999_999))
            .sum()
    }

    crate::common::permutations(locations)
        .into_iter()
        .map(move |v| route_length(&v[..], &distances))
}

/// Parses the puzzle input into an amount of locations and a map of the distances between them.
fn parse(input: &str) -> Result<(usize, Distances), String> {
    let mut locations = HashMap::new();
    let mut distances = HashMap::new();

    for line in input.lines() {
        let line: Vec<_> = line.split(' ').collect();
        for location in [line[0], line[2]] {
            if !locations.contains_key(location) {
                locations.insert(location, locations.len());
            }
        }

        let (l1, l2, v) = (
            locations[line[0]],
            locations[line[2]],
            line[4]
                .parse()
                .map_err(|_| "failed to parse input".to_owned())?,
        );
        distances.insert((l1, l2), v);
        distances.insert((l2, l1), v);
    }

    Ok((locations.len(), distances))
}
