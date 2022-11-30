use std::collections::HashMap;

/// Reconstruct the map; find the product of the four corner tile IDs.
pub fn one(input: &str) -> crate::Result<usize> {
    let map = build_map(parse(input)?);
    let e = map.len() - 1;
    Ok(map[0][0].0.id * map[e][0].0.id * map[0][e].0.id * map[e][e].0.id)
}

/// Count the total roughness of the sea on the map (number of # tiles that aren't part of
/// a monster).
pub fn two(input: &str) -> crate::Result<usize> {
    let map = build_map(parse(input)?);
    Ok(total_roughness(map))
}

type TempMap = HashMap<(i32, i32), (Tile, Transform)>;
type Map = Vec<Vec<(Tile, Transform)>>;

/// A list of points that constitute a sea monster.
const MONSTER_OFFSETS: [(i32, i32); 15] = [
    (0, 1),
    (1, 2),
    (4, 2),
    (5, 1),
    (6, 1),
    (7, 2),
    (10, 2),
    (11, 1),
    (12, 1),
    (13, 2),
    (16, 2),
    (17, 1),
    (18, 0),
    (18, 1),
    (19, 1),
];

/// Calculates the total roughness of the map; ie. the number of rough tiles (#) that aren't
/// part of a monster.
fn total_roughness(map: Map) -> usize {
    let mut roughness = 0;
    let mut monsters = 0;

    for y in 0..8 * map.len() as i32 {
        for x in 0..8 * map[0].len() as i32 {
            if rough_at(&map, (x, y)) {
                roughness += 1;
            }
            if monster_at(&map, (x, y)) {
                monsters += 1;
            }
        }
    }

    roughness - monsters * MONSTER_OFFSETS.len()
}

/// Checks if the given tile is "rough" (the character '#'). Coordinates beyond the map never
/// rough.
fn rough_at(map: &Map, (x, y): (i32, i32)) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    let (tile_x, tile_y) = ((x / 8) as usize, (y / 8) as usize);

    if let Some((tile, t)) = map.get(tile_y).and_then(|vs| vs.get(tile_x)) {
        let c = *tile
            .data
            .get(t.apply1d(((1 + x % 8) as usize, (1 + y % 8) as usize)))
            .unwrap_or(&b' ');
        c == b'#'
    } else {
        false
    }
}

/// Checks if there is a monster at the given coordinate. To be a monster, it must match the
/// pattern given by [`MONSTER_OFFSETS`].
fn monster_at(map: &Map, (x, y): (i32, i32)) -> bool {
    for t in Transform::all() {
        if MONSTER_OFFSETS
            .map(|p| t.apply(p))
            .map(|(dx, dy)| (x + dx, y + dy))
            .iter()
            .all(|&p| rough_at(map, p))
        {
            return true;
        }
    }
    false
}

/// Reconstructs the map from a list of tiles.
///
/// Algorithm is simple: Place down a tile, any random tile; then try to attach more tiles to
/// it, until the entire map is filled.
fn build_map(mut tiles: Vec<Tile>) -> Map {
    let mut map = HashMap::new();
    let mut open = vec![(0, 0)];

    while let Some(p) = open.pop() {
        if place(&mut map, &mut tiles, p) {
            open.extend([(-1, 0), (0, -1), (1, 0), (0, 1)].map(|v| (p.0 + v.0, p.1 + v.1)));
        }
    }

    let min_x = map.keys().map(|p| p.0).min().unwrap_or(0);
    let max_x = map.keys().map(|p| p.0).max().unwrap_or(0);
    let min_y = map.keys().map(|p| p.1).min().unwrap_or(0);
    let max_y = map.keys().map(|p| p.1).max().unwrap_or(0);

    let mut result = vec![];
    for y in min_y..=max_y {
        let mut next = vec![];
        for x in min_x..=max_x {
            next.push(map.remove(&(x, y)).unwrap());
        }
        result.push(next);
    }
    result
}

/// Tries to place a tile at the given coordinates; returns whether the operation was
/// successful. In order for a tile to be able to be placed, all four edges must match
/// the surrounding tiles.
fn place(map: &mut TempMap, candidates: &mut Vec<Tile>, (x, y): (i32, i32)) -> bool {
    if map.contains_key(&(x, y)) {
        return false;
    }

    for c in 0..candidates.len() {
        for t in Transform::all() {
            // Check if the left edge matches the board.
            if let Some((left_tile, left_t)) = map.get(&(x - 1, y)) {
                if candidates[c].left(t).ne(left_tile.right(*left_t)) {
                    continue;
                }
            }
            // Check if the top edge matches the board.
            if let Some((top_tile, top_t)) = map.get(&(x, y - 1)) {
                if candidates[c].top(t).ne(top_tile.bottom(*top_t)) {
                    continue;
                }
            }
            // Check if the right edge matches the board.
            if let Some((right_tile, right_t)) = map.get(&(x + 1, y)) {
                if candidates[c].right(t).ne(right_tile.left(*right_t)) {
                    continue;
                }
            }
            // Check if the left edge matches the board.
            if let Some((bottom_tile, bottom_t)) = map.get(&(x, y + 1)) {
                if candidates[c].bottom(t).ne(bottom_tile.top(*bottom_t)) {
                    continue;
                }
            }

            let tile = candidates.remove(c);
            map.insert((x, y), (tile, t));
            return true;
        }
    }

    false
}

/// Parses the puzzle input into a list of tiles.
fn parse(input: &str) -> crate::Result<Vec<Tile>> {
    let mut lines = input.lines();
    let mut tiles = vec![];

    while let Some(("Tile", n)) = lines
        .next()
        .and_then(|line| line.trim_end_matches(':').split_once(' '))
    {
        let mut tile = Tile {
            id: n.parse()?,
            data: vec![],
        };
        for _ in 0..10 {
            tile.data
                .extend(lines.next().ok_or("malformed input")?.bytes())
        }
        lines.next();
        tiles.push(tile);
    }

    Ok(tiles)
}

/// A single tile.
struct Tile {
    id: usize,
    data: Vec<u8>,
}

impl Tile {
    /// Gets the left edge of this tile, after being transformed by `t`.
    fn left(&self, t: Transform) -> impl Iterator<Item = u8> + '_ {
        (0..10).map(move |i| self.data[t.apply1d((0, i))])
    }

    /// Gets the top edge of this tile, after being transformed by `t`.
    fn top(&self, t: Transform) -> impl Iterator<Item = u8> + '_ {
        (0..10).map(move |i| self.data[t.apply1d((i, 0))])
    }

    /// Gets the right edge of this tile, after being transformed by `t`.
    fn right(&self, t: Transform) -> impl Iterator<Item = u8> + '_ {
        (0..10).map(move |i| self.data[t.apply1d((9, i))])
    }

    /// Gets the bottom edge of this tile, after being transformed by `t`.
    fn bottom(&self, t: Transform) -> impl Iterator<Item = u8> + '_ {
        (0..10).map(move |i| self.data[t.apply1d((i, 9))])
    }
}

/// A transformation that can be applied to the tiles; rotation (90, 180, 270 degrees), and
/// horizontal and vertical flipping. Since rotating by 180 is the same as flipping both ways,
/// we only need to keep track of 90 degrees of rotation, hence three bools: rotated, horizontal
/// flip and vertical flip.
#[derive(Copy, Clone)]
struct Transform(bool, bool, bool);

impl Transform {
    /// Returns an iterator of all possible transforms.
    fn all() -> impl Iterator<Item = Transform> {
        let tf = [true, false];
        tf.into_iter().flat_map(move |r| {
            tf.into_iter()
                .flat_map(move |h| tf.into_iter().map(move |v| Transform(r, h, v)))
        })
    }

    /// Given a 2D coordinate constained to a 10x10 space, applies this transformation, and then
    /// returns the corresponding 1D coordinate.
    #[rustfmt::skip]
    fn apply1d(&self, mut p: (usize, usize)) -> usize {
        let &Transform(r, h, v) = self;
        if r { p = (9 - p.1, p.0); }
        if h { p = (9 - p.0, p.1); }
        if v { p = (p.0, 9 - p.1); }
        p.1 * 10 + p.0
    }

    /// Applies this transformation to the given point.
    #[rustfmt::skip]
    fn apply(&self, mut p: (i32, i32)) -> (i32, i32) {
        let &Transform(r, h, v) = self;
        if r { p = (-p.1, p.0); }
        if h { p = (-p.0, p.1); }
        if v { p = (p.0, -p.1); }
        p
    }
}
