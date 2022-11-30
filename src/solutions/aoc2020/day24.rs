use std::collections::{HashMap, HashSet};

/// Count the number of black tiles initially.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(parse(input).len())
}

/// Count the number of black tiles after 100 days of the art exhibit.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut black_tiles = parse(input);

    for _ in 0..100 {
        // Calculate all upcoming changes based on existing black tiles.
        // `white_candidates` are tiles that are adjacent to at least one black cell; `to_flip`
        // are black cells that will turn white.
        let mut white_candidates: HashMap<_, i32> = HashMap::new();
        let mut to_flip = HashSet::new();
        for &(x, y) in &black_tiles {
            // Count black neighbours, and mark this as a neighbour for adjacent white tiles.
            let mut black_count = 0;
            for neighbour in DELTAS.map(|(dx, dy)| (x + dx, y + dy)) {
                if black_tiles.contains(&neighbour) {
                    black_count += 1;
                } else {
                    *white_candidates.entry(neighbour).or_default() += 1;
                }
            }
            if black_count == 0 || black_count > 2 {
                to_flip.insert((x, y));
            }
        }

        // Execute all the changes computed earlier.
        for (p, count) in white_candidates {
            if count == 2 {
                black_tiles.insert(p);
            }
        }

        for tile in to_flip {
            if !black_tiles.remove(&tile) {
                black_tiles.insert(tile);
            }
        }
    }

    Ok(black_tiles.len())
}

/// Parses the puzzle input into a set of black tiles.
fn parse(input: &str) -> HashSet<(i32, i32)> {
    let mut black_tiles = HashSet::new();
    for mut line in input.lines() {
        let mut p = (0, 0);
        while let Some(i) = DIRS.iter().position(|&p| line.starts_with(p)) {
            line = &line[DIRS[i].len()..];
            p = (p.0 + DELTAS[i].0, p.1 + DELTAS[i].1);
        }
        if !black_tiles.remove(&p) {
            black_tiles.insert(p);
        }
    }
    black_tiles
}

const DIRS: [&str; 6] = ["w", "nw", "ne", "e", "se", "sw"];
const DELTAS: [(i32, i32); 6] = [(-1, 0), (-1, -1), (0, -1), (1, 0), (1, 1), (0, 1)];
