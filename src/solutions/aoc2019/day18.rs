use std::collections::{HashMap, HashSet, VecDeque};

use crate::common::astar;

/// Find the shortest time to find all keys in the maze.
pub fn one(input: &str) -> crate::Result<i32> {
    let distances = build_distances(input, false);
    astar::shortest_path_length(State(b'@', 0u32), &distances).ok_or("no result".into())
}

/// Split the map into 4 separate maps, each with their own explorer. Only one
/// explorer is active at any given time. Find the shortest time to find all
/// keys.
pub fn two(input: &str) -> crate::Result<i32> {
    let distances = build_distances(input, true);
    astar::shortest_path_length(State4(*b"@$%^", 0u32), &distances).ok_or("no result".into())
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State(u8, u32);

impl<'a> astar::Node<'a> for State {
    type Cost = i32;
    type Env = HashMap<(u8, u8), (usize, u32)>;

    fn next(&self, env: &'a Self::Env) -> Box<dyn Iterator<Item = (Self, Self::Cost)> + 'a> {
        let &Self(location, keys) = self;
        Box::new((b'a'..=b'z').filter_map(move |target| {
            let (distance, required_keys) = *env.get(&(location, target))?;
            let has_keys = keys & required_keys == required_keys;

            has_keys.then_some((
                Self(target, keys | (1 << (target - b'a') as u32)),
                distance as i32,
            ))
        }))
    }

    fn heuristic(&self, _: &Self::Env) -> Self::Cost {
        (ALL_KEYS.count_ones() - self.1.count_ones()) as i32
    }

    fn done(&self, _: &Self::Env) -> bool {
        self.1 == ALL_KEYS
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State4([u8; 4], u32);

impl<'a> astar::Node<'a> for State4 {
    type Cost = i32;
    type Env = HashMap<(u8, u8), (usize, u32)>;

    fn next(&self, env: &'a Self::Env) -> Box<dyn Iterator<Item = (Self, Self::Cost)> + 'a> {
        let &Self(rs, keys) = self;
        Box::new(rs.into_iter().enumerate().flat_map(move |(i, location)| {
            (b'a'..=b'z').filter_map(move |target| {
                let (distance, required_keys) = *env.get(&(location, target))?;
                let has_keys = keys & required_keys == required_keys;

                has_keys.then(|| {
                    let mut rs = rs;
                    rs[i] = target;
                    let keys = keys | (1 << (target - b'a') as u32);
                    (Self(rs, keys), distance as i32)
                })
            })
        }))
    }

    fn heuristic(&self, _: &Self::Env) -> Self::Cost {
        (ALL_KEYS.count_ones() - self.1.count_ones()) as i32
    }

    fn done(&self, _: &Self::Env) -> bool {
        self.1 == ALL_KEYS
    }
}

const ALL_KEYS: u32 = 0b00_00001_11111_11111_11111_11111_11111;

/// Builds a hash map that maps pairs of destinations to the distance between them and a
/// bitmask of required keys to reach the other one.
fn build_distances(input: &str, advanced: bool) -> HashMap<(u8, u8), (usize, u32)> {
    let mut map: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut result = HashMap::new();

    if advanced {
        let x = map[0].len() as i32 / 2;
        let y = map.len() as i32 / 2;

        for dx in -1..=1 {
            for dy in -1..=1 {
                map[(y + dy) as usize][(y + dx) as usize] = b'#';
            }
        }

        map[(y - 1) as usize][(x - 1) as usize] = b'@';
        map[(y - 1) as usize][(x + 1) as usize] = b'$';
        map[(y + 1) as usize][(x - 1) as usize] = b'%';
        map[(y + 1) as usize][(x + 1) as usize] = b'^';
    }

    for (y, line) in map.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c.is_ascii_lowercase() || b"@$%^".contains(&c) {
                for (target, distance, mask) in floodfill_from(&map, (x, y)) {
                    result.entry((c, target)).or_insert((distance, mask));
                }
            }
        }
    }

    result
}

/// Floodfills the vault map from the given point, returning all interesting destinations
/// reached, alongside the distances and required keys for them (as a bitmask).
fn floodfill_from(map: &[Vec<u8>], start: (usize, usize)) -> Vec<(u8, usize, u32)> {
    let mut targets = vec![];

    let mut visited = HashSet::from([start]);
    let mut queue = VecDeque::new();
    queue.push_back((start, 0, 0u32));

    while let Some(((x, y), distance, mask)) = queue.pop_front() {
        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let (x, y) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);

            if !visited.insert((x, y)) {
                continue;
            }

            match map[y][x] {
                b'#' => continue,
                b'.' => queue.push_back(((x, y), distance + 1, mask)),
                b'A'..=b'Z' => {
                    let index = (map[y][x] - b'A') as u32;
                    queue.push_back(((x, y), distance + 1, mask | (1 << index)));
                }
                b'a'..=b'z' => {
                    queue.push_back(((x, y), distance + 1, mask));
                    targets.push((map[y][x], distance + 1, mask));
                }
                _ => {}
            }
        }
    }

    targets
}
