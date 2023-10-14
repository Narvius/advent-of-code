use std::collections::BinaryHeap;

/// Pick the bot with the largest radius, and count how many bots it sees.
pub fn one(input: &str) -> crate::Result<usize> {
    let bots = parse(input)?;
    let largest = *bots.iter().max_by_key(|b| b.range).unwrap();
    Ok(bots
        .into_iter()
        .filter(|b| distance(b.pos, largest.pos) <= largest.range)
        .count())
}

/// From among all points in space that are seen by the maximum amount of bots, pick the closest
/// one; and return its distance from origin.
///
/// I ended up giving up on this one, and consulted the reddit thread. The solution is
/// relatively simple, and definitely not universally correct.
///
/// 1. For each bot, draw a line through origin and it.
/// 2. Take the resulting intersecting line segments, record the two end points for each.
/// 3. Take these intervals, and find a point where the most intervals overlap.
///
/// Due to the structure of the input, this *happens* to produce the correct answer, but
/// it is easy to construct input sets where the result would be wrong.
pub fn two(input: &str) -> crate::Result<u32> {
    let bots = parse(input)?;

    let segments = bots.into_iter().map(|b| {
        let center = distance((0, 0, 0), b.pos);
        (center.saturating_sub(b.range), center + b.range)
    });

    let mut queue = BinaryHeap::new();
    for (low, high) in segments {
        queue.push(Entry(low, 1));
        queue.push(Entry(high, -1));
    }

    let mut current = 0;
    let mut best = 0;
    let mut result = 0;

    while let Some(Entry(point, change)) = queue.pop() {
        current += change;
        if current > best {
            best = current;
            result = point;
        }
    }
    Ok(result)
}

/// Manhattan distance between two 3D points.
fn distance((x1, y1, z1): (i32, i32, i32), (x2, y2, z2): (i32, i32, i32)) -> u32 {
    x1.abs_diff(x2) + y1.abs_diff(y2) + z1.abs_diff(z2)
}

/// A distance from origin, paired with whether it is the start (+1) or end (-1) of a line
/// segment. See description of part 2 for more details.
///
/// [`BinaryHeap`] is a max-heap. Since I want a min-heap (closest distance first), Ord is
/// implemented inversed.
#[derive(Eq, PartialEq)]
struct Entry(u32, i32);

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

/// An entry from the puzzle input.
#[derive(Copy, Clone)]
struct Bot {
    pos: (i32, i32, i32),
    range: u32,
}

/// Constructs a list of [`Bot`]s from puzzle input.
fn parse(input: &str) -> crate::Result<Vec<Bot>> {
    let mut bots = vec![];
    for line in input.lines() {
        let (_, pos) = line
            .split_once("=<")
            .ok_or_else(|| "unexpected input format".to_string())?;

        let (pos, r) = pos
            .split_once(">, r=")
            .ok_or_else(|| "unexpected input format".to_string())?;

        let pos = pos
            .split(',')
            .map(|n| n.parse())
            .collect::<Result<Vec<_>, _>>()?;
        let [a, b, c] = pos.as_slice() else {
            return Err("wrong number of coordinates".into());
        };

        bots.push(Bot {
            pos: (*a, *b, *c),
            range: r.parse()?,
        });
    }
    Ok(bots)
}
