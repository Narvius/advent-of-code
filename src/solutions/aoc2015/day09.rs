use std::collections::HashMap;

/// Find the shortest possible route that visits all cities.
pub fn one(input: &str) -> Result<String, String> {
    let (locations, distances) = parse(input).ok_or(format!("failed to parse puzzle input"))?;
    Ok(all_route_lengths(locations, distances)
        .min()
        .ok_or(format!("no input locations"))?
        .to_string())
}

/// Find the longest possible route that visits all cities.
pub fn two(input: &str) -> Result<String, String> {
    let (locations, distances) = parse(input).ok_or(format!("failed to parse puzzle input"))?;
    Ok(all_route_lengths(locations, distances)
        .max()
        .ok_or(format!("no input locations"))?
        .to_string())
}

type Distances = HashMap<(usize, usize), usize>;

/// Returns the lengths of all possible routes.
fn all_route_lengths(locations: usize, distances: Distances) -> impl Iterator<Item = usize> {
    // Returns all possible permutations of the numbers in `0..k`, using Heap's algorithm.
    fn permutations(k: usize) -> Vec<Vec<usize>> {
        fn inner(k: usize, values: &mut [usize]) -> Vec<Vec<usize>> {
            let mut result = Vec::new();
            if k <= 1 {
                result.push(Vec::from(values));
            } else {
                result.extend(inner(k - 1, values));
                for i in 0..(k - 1) {
                    if k % 2 == 0 {
                        values.swap(i, k - 1);
                    } else {
                        values.swap(0, k - 1);
                    }
                    result.extend(inner(k - 1, values));
                }
            }
            result
        }

        inner(k, &mut (0..k).collect::<Vec<_>>())
    }

    // Calculates the length of a route.
    fn route_length(order: &[usize], distances: &Distances) -> usize {
        order
            .windows(2)
            .map(|w| distances.get(&(w[0], w[1])).unwrap_or(&999_999))
            .sum()
    }

    permutations(locations)
        .into_iter()
        .map(move |v| route_length(&v[..], &distances))
}

/// Parses the puzzle input into an amount of locations and a map of the distances between them.
fn parse(input: &str) -> Option<(usize, Distances)> {
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
            line[4].parse().ok()?,
        );
        distances.insert((l1, l2), v);
        distances.insert((l2, l1), v);
    }

    Some((locations.len(), distances))
}