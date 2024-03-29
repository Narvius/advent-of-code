use std::collections::HashMap;

use crate::common::bfs;

/// Interpreting labels as teleports between each other, find the length of
/// the shortest path from `AA` to `ZZ`.
pub fn one(input: &str) -> crate::Result<usize> {
    let map = parse(input)?;

    bfs(
        map.entrance,
        |&n| map.neighbours(n, None),
        |&n| n == map.exit,
    )
    .ok_or("no result".into())
}

/// Interpreting labels as teleports to a recursively deeper or shallower
/// maze with the same layout, find the length of the shortest path from
/// `AA` to `ZZ`.
pub fn two(input: &str) -> crate::Result<usize> {
    let map = parse(input)?;

    bfs(
        (map.entrance, 0),
        |&n| map.neighbours_layers(n),
        |&n| n == (map.exit, 0),
    )
    .ok_or("no result".into())
}

/// A parsed map from the puzzle input.
struct Map {
    /// The square labelled `AA`.
    entrance: (i32, i32),
    /// The square labelled `ZZ`.
    exit: (i32, i32),
    /// Raw map data. While it technically still has the labels, anything
    /// other than `.`s are ignored.
    map: Vec<Vec<u8>>,
    /// Portal data. Key = starting tile of the jump, Value = target tile
    /// after teleporting, as well as a bool that says if the portal being
    /// entered is an outer (`true`) or inner (`false`) portal.
    ///
    /// This bool is relevant only in part 2.
    portals: HashMap<(i32, i32), PortalInfo>,
}

impl Map {
    /// Returns all walkable neighbours of a tile, including teleports as
    /// described in the first part of the puzzle.
    fn neighbours(
        &self,
        (x, y): (i32, i32),
        layer: Option<i32>,
    ) -> impl Iterator<Item = (i32, i32)> + '_ {
        let direct = DELTAS
            .into_iter()
            .map(move |(dx, dy)| (x + dx, y + dy))
            .filter(|&(x, y)| self.map[y as usize][x as usize] == b'.');

        let teleporter = match layer {
            Some(n) => self
                .portals
                .get(&(x, y))
                .and_then(|(p, outer)| (n > 0 || !outer).then_some(*p)),
            None => self.portals.get(&(x, y)).map(|v| v.0),
        };

        direct.chain(teleporter)
    }

    /// Returns all walkable neighbours of a tile, including recursive
    /// teleports as described in the second part of the puzzle.
    fn neighbours_layers(
        &self,
        ((x, y), layer): ((i32, i32), i32),
    ) -> impl Iterator<Item = ((i32, i32), i32)> + '_ {
        let direct = DELTAS
            .into_iter()
            .map(move |(dx, dy)| ((x + dx, y + dy), layer))
            .filter(|&((x, y), _)| self.map[y as usize][x as usize] == b'.');

        let teleporter = self.portals.get(&(x, y)).and_then(|(p, outer)| {
            (layer > 0 || !outer).then_some((*p, layer + 2 * i32::from(!outer) - 1))
        });

        direct.chain(teleporter)
    }
}

/// Position offsets to consider for basic neighbour lists.
const DELTAS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

type Label = (u8, u8);
type PortalInfo = ((i32, i32), bool);

/// Checks if the given tile is a teleporter label, and returns information
/// about the associated teleporter. The information includes the two-byte
/// label, the position of the teleporter tile, and whether it is an inner
/// or outer teleport (relevant for part 2).
///
/// A tile is a teleporter label if it is a letter directly adjacent to an open
/// tile.
fn portal_info(map: &[Vec<u8>], (x, y): (i32, i32)) -> Option<(Label, PortalInfo)> {
    let center = map[y as usize][x as usize];
    if !center.is_ascii_alphabetic() {
        return None;
    }

    // Bounds within which a teleport is considered to be "inner".
    let ((il, it), (ir, ib)) = ((5, 5), (map[0].len() - 5, map.len() - 5));

    let mut pair = (b' ', b' ');
    let mut portal = None;
    for (dx, dy) in DELTAS {
        let c = *map
            .get((y + dy) as usize)
            .and_then(|line| line.get((x + dx) as usize))
            .unwrap_or(&b' ');

        match c {
            b'.' => {
                // An adjacent open tile is the actual teleporter.
                // It is `outer` if it's not within the inner bounds rectangle
                // described by (il, it, ir, ib).
                portal = Some((
                    (x + dx, y + dy),
                    !((il..=ir).contains(&(x as usize)) && (it..=ib).contains(&(y as usize))),
                ))
            }
            c if c.is_ascii_alphabetic() => {
                pair = match (dx == 1) || (dy == 1) {
                    true => (center, c),
                    false => (c, center),
                }
            }
            _ => {}
        }
    }

    portal.map(|portal| (pair, portal))
}

/// Parses the puzzle input into a [`Map`].
fn parse(input: &str) -> crate::Result<Map> {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let mut portals = HashMap::new();
    let mut one_side = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, _) in line.bytes().enumerate() {
            if let Some((label, (position, outer))) = portal_info(&map, (x as i32, y as i32)) {
                match one_side.remove(&label) {
                    Some((other_position, other_outer)) => {
                        portals.insert(position, (other_position, outer));
                        portals.insert(other_position, (position, other_outer));
                    }
                    None => {
                        one_side.insert(label, (position, outer));
                    }
                }
            }
        }
    }

    Ok(Map {
        entrance: one_side[&(b'A', b'A')].0,
        exit: one_side[&(b'Z', b'Z')].0,
        map,
        portals,
    })
}
