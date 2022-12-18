use std::collections::{HashMap, HashSet};

/// Find the total number of sides that do not touch another cube.
pub fn one(input: &str) -> crate::Result<usize> {
    // We iterate over the list once, keeping track of how many sides are exposed for each
    // droplet. When a new droplet is added, it reduces the count for all its neighbours
    // by 1, and it itself starts with a count equal to the number of missing neighbours.

    let mut droplets = HashMap::new();

    for (x, y, z) in parse(input) {
        let neighbours = DELTAS.map(|(dx, dy, dz)| (x + dx, y + dy, z + dz));
        let open_sides = neighbours
            .into_iter()
            .filter(|p| {
                let Some(v) = droplets.get_mut(p) else { return true; };
                *v -= 1;
                false
            })
            .count();
        droplets.insert((x, y, z), open_sides);
    }

    Ok(droplets.into_values().sum())
}

/// Find the total number of sides exposed to the outside.
pub fn two(input: &str) -> crate::Result<usize> {
    // We floodfill the bounding cube of the droplet constellation, counting the number of
    // sides that are hit.

    let droplets: HashSet<_> = parse(input).collect();

    // Find the bounding cube.
    let bounds = |f: fn(&Point) -> i32| {
        let (min, max) = droplets.iter().fold((i32::MAX, i32::MIN), |(min, max), p| {
            (min.min(f(p)), max.max(f(p)))
        });
        (min - 1..=max + 1, min - 1)
    };

    let (xs, min_x) = bounds(|p| p.0);
    let (ys, min_y) = bounds(|p| p.1);
    let (zs, min_z) = bounds(|p| p.2);

    // Perform the floodfill.
    let mut touched_sides = 0;
    let mut stack = vec![(min_x, min_y, min_z)];
    let mut visited = HashSet::from([(min_x, min_y, min_z)]);

    while let Some((x, y, z)) = stack.pop() {
        let neighbours = DELTAS.map(|(dx, dy, dz)| (x + dx, y + dy, z + dz));
        for p in neighbours {
            if !xs.contains(&p.0) || !ys.contains(&p.1) || !zs.contains(&p.2) {
                continue;
            }

            if droplets.contains(&p) {
                touched_sides += 1;
            } else if visited.insert(p) {
                stack.push(p);
            }
        }
    }

    Ok(touched_sides)
}

// A point on a 3D grid.
type Point = (i32, i32, i32);

/// Offsets to orthogonal neighbours of a point on a 3D grid.
const DELTAS: [Point; 6] = [
    (-1, 0, 0),
    (0, -1, 0),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, 0),
    (1, 0, 0),
];

/// Parses the puzzle input into a series of 3D points.
fn parse(input: &str) -> impl Iterator<Item = Point> + '_ {
    input.lines().filter_map(|line| {
        let mut cs = line.split(',').filter_map(|c| c.parse().ok());
        Some((cs.next()?, cs.next()?, cs.next()?))
    })
}
