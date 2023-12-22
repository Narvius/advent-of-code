use std::collections::{HashMap, HashSet};

/// Find the number of bricks that could individually be removed without making any other bricks
/// fall further.
pub fn one(input: &str) -> crate::Result<usize> {
    let supports = support_map(input);
    Ok((0..supports.len())
        .filter(|&i| !supports[i..].iter().any(|s| s == &[i]))
        .count())
}

/// For each brick, find how many other bricks would fall further if it was removed. Sum that
/// amount across each individual brick.
pub fn two(input: &str) -> crate::Result<usize> {
    let supports = support_map(input);

    Ok((0..supports.len())
        .map(|i| {
            let mut disintegrated = HashSet::from([i]);
            for (di, s) in supports[i..].iter().enumerate() {
                if !s.is_empty() && s.iter().all(|n| disintegrated.contains(n)) {
                    disintegrated.insert(i + di);
                }
            }
            disintegrated.len() - 1
        })
        .sum())
}

/// Settles all bricks from the puzzle input, establishing which ones support which other bricks;
/// returns a list where the `i`th element indicates which bricks support the `i`th brick.
fn support_map(input: &str) -> Vec<Vec<usize>> {
    let mut bricks: Vec<_> = parse(input).collect();
    let mut supports = vec![vec![]; bricks.len()];
    let mut map = HashMap::new();

    // Sort all bricks by their lowest Z coordinate. That way we can just settle them in order,
    // they could only ever drop onto bricks earlier in the list.
    bricks.sort_unstable_by_key(|b| b.0[2]);

    for (i, &brick @ ([x1, y1, z1], [x2, y2, z2])) in bricks.iter().enumerate() {
        // Go as far down as you can.
        let mut z = brick.0[2];
        while z > 0 {
            for (x, y) in shadow(brick) {
                if let Some(&v) = map.get(&[x, y, z - 1]) {
                    if !supports[i].contains(&v) {
                        supports[i].push(v);
                    }
                }
            }

            if !supports[i].is_empty() {
                break;
            }

            z -= 1;
        }

        // Apply brick to map.
        for z in z..=(z + z2 - z1) {
            for y in y1..=y2 {
                for x in x1..=x2 {
                    map.insert([x, y, z], i);
                }
            }
        }
    }

    supports
}

/// Returns all (x, y) pairs that can make contact with something below them for the given brick.
fn shadow(([x1, y1, _], [x2, y2, _]): Brick) -> impl Iterator<Item = (i32, i32)> {
    (x1..=x2).flat_map(move |x| (y1..=y2).map(move |y| (x, y)))
}

type P = [i32; 3];
type Brick = (P, P);

/// Parses the puzzle input into a series of bricks.
fn parse(input: &str) -> impl Iterator<Item = Brick> + '_ {
    fn p(s: &str) -> Option<P> {
        let nums: Vec<_> = s.split(',').filter_map(|n| n.parse().ok()).collect();
        nums.try_into().ok()
    }

    input.lines().filter_map(|line| {
        let (p1, p2) = line.split_once('~')?;
        Some((p(p1)?, p(p2)?))
    })
}
