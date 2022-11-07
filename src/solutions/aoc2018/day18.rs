use std::collections::HashMap;

/// Find the resource value after ten minutes.
pub fn one(input: &str) -> crate::Result<usize> {
    let mut map: Map = input.parse()?;
    for _ in 0..10 {
        map.step();
    }
    Ok(map.resource_value())
}

/// Find the resource value after one billion minutes.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut map: Map = input.parse()?;
    let mut past = HashMap::new();
    let target = 1000000000;

    // Note: This implementation is relatively slow, since we end up cloning the entire map
    // hundreds of times to use as hash keys. But it's efficient *enough* for now.

    // Part 1: Find the repetition, and fast-forward until nearly the end using it.
    let mut g = 0;
    while g < target {
        map.step();
        if let Some(g2) = past.insert(map.data.clone(), g) {
            // Hit a repeated state, so we know it cycles infinitely. No need to calculate
            // all those reps, just jump ahead. We know the stride (length of the cycle), and
            // can calculate how many full cycles we can skip, and do that. Then we end up
            // really close to the 1 billion generations.
            let stride = g2 - g;
            let cycles = (target - g) / stride;
            g += stride * cycles + 1;
            break;
        } else {
            g += 1;
        }
    }

    // Part 2: Manually finish off the couple last steps--could theoretically look up the
    // true final state from "past", but that's not worth the effort, we only need a couple more.
    for g in g..target {
        map.step();
    }
    Ok(map.resource_value())
}

/// Map of the lumber collection area; effectively a three-state cellular automaton.
struct Map {
    data: Vec<Tile>,
    buffer: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    /// The current resource value of the lumber collection area.
    fn resource_value(&self) -> usize {
        let (_, t, y) = as_counts(self.data.iter().copied());
        t * y
    }

    /// Steps the cellular automaton.
    fn step(&mut self) {
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                if let Some(tile) = self.get((x, y)) {
                    let tile = match (tile, as_counts(self.neighbours((x, y)))) {
                        (Tile::Open, (_, t, _)) if t >= 3 => Tile::Tree,
                        (Tile::Tree, (_, _, y)) if y >= 3 => Tile::Yard,
                        (Tile::Yard, (_, t, y)) => {
                            if t >= 1 && y >= 1 {
                                Tile::Yard
                            } else {
                                Tile::Open
                            }
                        }
                        _ => tile,
                    };

                    self.buffer[y as usize * self.width + x as usize] = tile;
                }
            }
        }
        std::mem::swap(&mut self.data, &mut self.buffer);
    }

    /// Returns all tiles adjacent to this one.
    fn neighbours(&self, (x, y): (i32, i32)) -> impl Iterator<Item = Tile> + '_ {
        DELTAS
            .into_iter()
            .filter_map(move |(dx, dy)| self.get((x + dx, y + dy)))
    }

    /// Gets the tile at the given coordinates, if it exists.
    fn get(&self, (x, y): (i32, i32)) -> Option<Tile> {
        if (0..self.width as i32).contains(&x) && (0..self.height as i32).contains(&y) {
            Some(self.data[y as usize * self.width + x as usize])
        } else {
            None
        }
    }
}

/// Converts an iterator over tiles into counts of open, wooded and lumberyard tiles in it.
fn as_counts(tiles: impl Iterator<Item = Tile>) -> (usize, usize, usize) {
    tiles.fold((0, 0, 0), |(o, t, y), tile| match tile {
        Tile::Open => (o + 1, t, y),
        Tile::Tree => (o, t + 1, y),
        Tile::Yard => (o, t, y + 1),
    })
}

impl std::str::FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::with_capacity(2500);
        for line in s.lines() {
            for c in line.chars() {
                map.push(match c {
                    '.' => Tile::Open,
                    '|' => Tile::Tree,
                    '#' => Tile::Yard,
                    _ => return Err("failed parse"),
                })
            }
        }

        let width = s.lines().next().ok_or("failed parse")?.len();
        let height = s.lines().count();

        Ok(Self {
            data: map,
            buffer: vec![Tile::Open; width * height],
            width,
            height,
        })
    }
}

/// A single tile.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Tile {
    /// An open tile.
    Open,
    /// A wooded tile.
    Tree,
    /// A lumberyard.
    Yard,
}

/// Contains offsets to all eight adjacent tiles.
#[rustfmt::skip]
const DELTAS: [(i32, i32); 8] = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];
