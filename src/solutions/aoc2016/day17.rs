use std::collections::VecDeque;

/// Find the shortest path to the vault.
pub fn one(input: &str) -> crate::Result<String> {
    let mut d = md5::Context::new();
    d.consume(input.as_bytes());
    let mut candidates = VecDeque::from([(0, 0, String::new())]);
    let mut shortest: Option<String> = None;
    let mut shortest_len = usize::MAX;

    while let Some((x, y, p)) = candidates.pop_front() {
        if shortest_len < p.len() {
            continue;
        }

        if (x, y) == (3, 3) && p.len() < shortest_len {
            shortest_len = p.len();
            shortest = Some(p);
            continue;
        }

        let hash = {
            let mut d = d.clone();
            d.consume(p.as_bytes());
            format!("{:x}", d.compute())
        };

        for ((dx, dy, c), o) in DELTAS.into_iter().zip(hash.bytes()) {
            let (x, y) = (x + dx, y + dy);
            if (0..=4).contains(&x) && (0..=4).contains(&y) {
                match o {
                    (b'b'..=b'f') => {
                        let mut s = p.clone();
                        s.push(c as char);
                        candidates.push_back((x, y, s));
                    }
                    _ => continue,
                }
            }
        }
    }

    shortest.ok_or_else(|| "no path found".into())
}

/// Find the length of the longest possible path to the vault.
///
/// Same structure as [`one`] (a kind of floodfill), but differs in several important ways:
/// * no candidates are discarded early based on path length
/// * the exact paths are not kept track of, only the length and partial md5 hashes
pub fn two(input: &str) -> crate::Result<usize> {
    let d = {
        let mut d = md5::Context::new();
        d.consume(input);
        d
    };
    let mut candidates = VecDeque::from([(0, 0, 0, d)]);
    let mut longest = 0usize;

    while let Some((x, y, l, d)) = candidates.pop_front() {
        if (x, y) == (3, 3) {
            longest = longest.max(l);
            continue;
        }

        let hash = format!("{:x}", d.clone().compute());

        for ((dx, dy, c), o) in DELTAS.into_iter().zip(hash.bytes()) {
            let (x, y) = (x + dx, y + dy);
            if (0..4).contains(&x) && (0..4).contains(&y) {
                match o {
                    (b'b'..=b'f') => {
                        let mut d = d.clone();
                        d.consume([c]);
                        candidates.push_back((x, y, l + 1, d));
                    }
                    _ => continue,
                }
            }
        }
    }

    Ok(longest)
}

/// Directions alongside the characters they add to the hash.
const DELTAS: [(i32, i32, u8); 4] = [(0, -1, b'U'), (0, 1, b'D'), (-1, 0, b'L'), (1, 0, b'R')];
