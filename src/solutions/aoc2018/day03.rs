/// Find the total number of cells that have conflicting claims.
pub fn one(input: &str) -> crate::Result<usize> {
    let mut tiles = vec![0u8; 1_000_000];
    for (_, (x, y), (w, h)) in parse(input) {
        for y in y..y + h {
            for x in x..x + w {
                tiles[y * 1000 + x] += 1;
            }
        }
    }
    Ok(tiles.into_iter().filter(|&c| c >= 2).count())
}

/// Find the ID of the only claim that doesn't overlap with any other.
pub fn two(input: &str) -> crate::Result<usize> {
    let claims: Vec<_> = parse(input).collect();

    for &claim in &claims {
        let overlapping = claims.iter().filter(|&&c| overlap(c, claim));
        if overlapping.count() == 1 {
            return Ok(claim.0);
        }
    }

    Err("failed to find nonoverlapping claim".into())
}

/// A claim from the puzzle input.
type Claim = (usize, (usize, usize), (usize, usize));

/// Checks whether two claims overlap.
fn overlap(r1: Claim, r2: Claim) -> bool {
    let (_, (x1, y1), (w1, h1)) = r1;
    let (_, (x2, y2), (w2, h2)) = r2;
    let (l1, t1, r1, b1) = (x1, y1, x1 + w1, y1 + h1);
    let (l2, t2, r2, b2) = (x2, y2, x2 + w2, y2 + h2);

    !(l2 > r1 || r2 < l1 || t2 > b1 || b2 < t1)
}

/// Parses the puzzle input.
fn parse(input: &str) -> impl Iterator<Item = Claim> + '_ {
    input.lines().filter_map(|line| {
        let (id, data) = line.split_once(" @ ")?;
        let (topleft, size) = data.split_once(": ")?;
        let (x, y) = topleft.split_once(',')?;
        let (w, h) = size.split_once('x')?;

        Some((
            id[1..].parse().ok()?,
            (x.parse().ok()?, y.parse().ok()?),
            (w.parse().ok()?, h.parse().ok()?),
        ))
    })
}
