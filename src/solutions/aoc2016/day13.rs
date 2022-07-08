use std::collections::{HashMap, VecDeque};

/// Find how many steps it takes to reach (31, 39) from (1, 1).
pub fn one(input: &str) -> crate::Result<i32> {
    let mut map = Map::new(input.parse().map_err(|_| "parse failed".to_owned())?);
    Ok(pathfind(&mut map, (1, 1), (31, 39)))
}

/// Find how many distinct tiles are reachable within 50 steps of the starting position.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut map = Map::new(input.parse().map_err(|_| "parse failed".to_owned())?);
    pathfind(&mut map, (1, 1), (31, 39));
    Ok(map
        .cache
        .values()
        .filter(|&&v| (0..=50).contains(&v))
        .count())
}

/// Floodfills a [`Map`] from the given `start`, setting the value of all open spaces to the
/// distance from it. Stops once it reaches `end`, and returns the distance for it.
fn pathfind(map: &mut Map, start: (i32, i32), end: (i32, i32)) -> i32 {
    let mut candidates = VecDeque::from([start]);

    while let Some(pos) = candidates.pop_front() {
        for (ox, oy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let target = (pos.0 + ox, pos.1 + oy);
            if target.0 < 0 || target.1 < 0 {
                continue;
            }

            if *map.value_at_mut(target) == 0 {
                *map.value_at_mut(target) = *map.value_at_mut(pos) + 1;
                candidates.push_back(target);
            }

            if target == end {
                return *map.value_at_mut(target);
            }
        }
    }

    0
}

struct Map {
    /// Contains information whether the given tile is a walkable or not.
    cache: HashMap<(i32, i32), i32>,
    /// The puzzle input that determines the shape of the map.
    seed: i32,
}

impl Map {
    /// Constructs a new map from the given seed.
    fn new(seed: i32) -> Self {
        Self {
            cache: HashMap::new(),
            seed,
        }
    }

    /// Gets the value of a tile. -1 means "wall", 0 means "untested open space" and a value
    /// above 0 means "distance from starting point". See [`pathfind`] for more information.
    fn value_at_mut(&mut self, (x, y): (i32, i32)) -> &mut i32 {
        self.cache.entry((x, y)).or_insert_with(|| {
            let v = x * x + 3 * x + 2 * x * y + y + y * y + self.seed;
            if v.count_ones() % 2 == 0 {
                0
            } else {
                -1
            }
        })
    }
}
