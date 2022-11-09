use std::collections::{HashMap, HashSet, VecDeque};

/// Find the distance to furthest-away room.
pub fn one(input: &str) -> crate::Result<i32> {
    parse(input.trim_matches(&['^', '$'][..]).as_bytes())
        .ok_or("parse failed")?
        .compile()
        .into_distances()
        .into_values()
        .max()
        .ok_or_else(|| "no result".into())
}

/// Find the amount of rooms at least a distance of 1000 away.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(parse(input.trim_matches(&['^', '$'][..]).as_bytes())
        .ok_or("parse failed")?
        .compile()
        .into_distances()
        .into_values()
        .filter(|&v| v >= 1000)
        .count())
}

type Doors = HashSet<((i32, i32), (i32, i32))>;

/// Contains a hash set that says that if the pair (p, q) is contained in it, there is
/// a door between those points.
struct Map(Doors);

impl Map {
    /// Converts this map into a mapping from coordinates to distances to reach it.
    fn into_distances(self) -> HashMap<(i32, i32), i32> {
        let mut queue = VecDeque::from([((0, 0), -1)]);
        let mut scores = HashMap::new();

        while let Some(((x, y), s)) = queue.pop_front() {
            if scores.contains_key(&(x, y)) {
                continue;
            }

            scores.insert((x, y), s + 1);

            for p in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)] {
                if self.0.contains(&((x, y), p)) {
                    queue.push_back((p, s + 1));
                }
            }
        }

        scores
    }
}

/// The input parsed into a tree.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Pattern<'a> {
    Lit(&'a [u8]),
    Branch(Vec<Pattern<'a>>),
    Seq(Vec<Pattern<'a>>),
}

impl<'a> Pattern<'a> {
    /// Turns the input tree into a [`Map`].
    fn compile(self) -> Map {
        // Adds a door to the map.
        fn mark(map: &mut Doors, pos: &mut (i32, i32), dir: u8) {
            let start = *pos;
            match dir {
                b'W' => pos.0 -= 1,
                b'N' => pos.1 -= 1,
                b'E' => pos.0 += 1,
                b'S' => pos.1 += 1,
                _ => {}
            }
            map.insert((start, *pos));
            map.insert((*pos, start));
        }

        // The main recursive function. Returns a list of positions that a person could have
        // ended up at by following the pattern; it's a list, because all branches are
        // evaluated simultaneously.
        fn work(pat: Pattern<'_>, map: &mut Doors, mut pos: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
            match pat {
                Pattern::Lit(steps) => {
                    for pos in &mut pos {
                        for &step in steps {
                            mark(map, pos, step);
                        }
                    }
                    pos
                }
                Pattern::Branch(branches) => {
                    let mut result = vec![];
                    for branch in branches {
                        result.append(&mut work(branch, map, pos.clone()));
                    }
                    result
                }
                Pattern::Seq(blocks) => {
                    for block in blocks {
                        pos = work(block, map, pos);
                    }
                    pos
                }
            }
        }

        let mut map = HashSet::new();
        work(self, &mut map, vec![(0, 0)]);
        Map(map)
    }
}

/// Parses the puzzle input into an AST, for easier processing down the line.
fn parse(data: &[u8]) -> Option<Pattern<'_>> {
    let mut fragments = vec![];
    if data.is_empty() {
        return None;
    }

    if let Some((pos, _)) = data.iter().enumerate().find(|(_, &c)| c == b'(') {
        if pos > 0 {
            fragments.push(Pattern::Lit(&data[0..pos]));
        }

        // Find the *matching* closing bracket.
        let mut depth = 1;
        let mut dividers = vec![pos];
        for i in (pos + 1).. {
            match data[i] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                b'|' if depth == 1 => dividers.push(i),
                _ => continue,
            }

            if depth == 0 {
                dividers.push(i);
                fragments.push(Pattern::Branch(
                    dividers
                        .windows(2)
                        .filter_map(|w| parse(&data[(w[0] + 1)..w[1]]))
                        .collect(),
                ));
                if i < data.len() {
                    match parse(&data[i..]) {
                        Some(Pattern::Seq(mut ps)) => fragments.append(&mut ps),
                        Some(pat) => fragments.push(pat),
                        _ => {}
                    }
                }
                break;
            }
        }
    } else {
        return Some(Pattern::Lit(data));
    }

    if fragments.len() == 1 {
        fragments.pop()
    } else {
        Some(Pattern::Seq(fragments))
    }
}
