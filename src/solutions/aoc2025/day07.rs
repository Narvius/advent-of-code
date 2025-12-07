use std::collections::{BTreeMap, HashSet};

/// Find how many times the beam splits.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(run_beams(input).0)
}

/// Find the number of possible timelines for the beam.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(run_beams(input).1)
}

/// Runs the tachyon beam through the manifold and returns the number of splits and timelines for
/// the beam.
fn run_beams(input: &str) -> (usize, usize) {
    let g = crate::common::Grid::from_input(input);

    // A map of "beam heads". A beam head is the position of where a beam spawns; ie. one initial
    // beam and then to the left and right of every hit splitter.
    //
    // Positions are stored in `(y, x)` order, so the `BTreeMap` spits them out in reading order.
    // This simplifies the logic, since for every beam we know that everything before it is already
    // processed.
    //
    // The value corresponds to the number of timelines that beam appears in. This lets us collate
    // calculations for the exponentially many timelines in a more efficient fashion.
    let mut heads = BTreeMap::from([((1, g.width() as i32 / 2), 1)]);

    let mut visited_splitters = HashSet::new();
    let mut timelines = 0;

    'outer: while let Some(((mut y, x), times)) = heads.pop_first() {
        while let Some(c) = g.at((x, y)) {
            y += 1;
            if c == b'^' {
                visited_splitters.insert((x, y));
                *heads.entry((y, x - 1)).or_default() += times;
                *heads.entry((y, x + 1)).or_default() += times;

                continue 'outer;
            }
        }

        // Beams that leave the map through the bottom correspond to a timeline each.
        timelines += times;
    }

    (visited_splitters.len(), timelines)
}
