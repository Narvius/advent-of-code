use std::collections::HashMap;

/// Find the weight on the northern support beams.
///
/// Can be done without actually moving the rocks; after writing part 2 I could technically replace
/// this with less code by factoring out parsing/score calculation and using that + [`tilt`], but
/// I'm happy with this solution.
pub fn one(input: &str) -> crate::Result<usize> {
    let map: Vec<_> = input.lines().map(str::as_bytes).collect();

    let mut total = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == b'O' {
                let mut added_weight = 0;
                for dy in 1..=y {
                    match map[y - dy][x] {
                        b'.' => added_weight += 1,
                        b'#' => break,
                        _ => {}
                    }
                }
                total += map.len() - y + added_weight;
            }
        }
    }
    Ok(total)
}

/// Simulate 1000000000 spin cycles, and find the weight on the northern support beams.
pub fn two(input: &str) -> crate::Result<i32> {
    let mut map = Map {
        data: input.bytes().filter(|c| !c.is_ascii_control()).collect(),
        width: input.lines().next().map(str::len).unwrap_or(0) as i32,
        height: input.lines().count() as i32,
    };
    let mut seen = HashMap::from([(map.data.clone(), 0)]);

    for step in 1.. {
        spin_cycle(&mut map);

        let repeat = seen.insert(map.data.clone(), step);
        if let Some(repeat) = repeat {
            let left = (1000000000 - step) % (step - repeat);
            let (data, _) = seen.into_iter().find(|(_, v)| *v == repeat + left).unwrap();

            let weight = |(i, tile): (usize, &u8)| match tile {
                b'O' => map.height - i as i32 / map.width,
                _ => 0,
            };

            return Ok(data.iter().enumerate().map(weight).sum());
        }
    }

    unreachable!()
}

struct Map {
    /// Contains the map data, as a flat array.
    data: Vec<u8>,
    width: i32,
    height: i32,
}

impl Map {
    /// Returns the element at the given position. Treats out of bounds positions as unmoveable
    /// rocks.
    fn get(&self, x: i32, y: i32) -> u8 {
        if !((0..self.width).contains(&x) && (0..self.height).contains(&y)) {
            b'#'
        } else {
            self.data[(y * self.width + x) as usize]
        }
    }

    /// Sets the given position to `value`. Doesn't do anything for out-of-bounds positions.
    fn set(&mut self, x: i32, y: i32, value: u8) {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.data[(y * self.width + x) as usize] = value;
        }
    }
}

/// Performs a single tilt. `ys` and `xs` are given as parameters because we might have to iterate
/// from different directions to avoid spurious collisions. For example, when tilting east, we want
/// to move the right-most rocks first, when tilting west, we want to move the left-most ones
/// first.
fn tilt<Y, X>(map: &mut Map, ys: Y, xs: X, (dx, dy): (i32, i32))
where
    Y: Iterator<Item = i32>,
    X: Iterator<Item = i32> + Clone,
{
    for y in ys {
        for x in xs.clone() {
            if map.get(x, y) == b'O' {
                map.set(x, y, b'.');
                for d in 1.. {
                    if map.get(x + dx * d, y + dy * d) != b'.' {
                        map.set(x + dx * (d - 1), y + dy * (d - 1), b'O');
                        break;
                    }
                }
            }
        }
    }
}

/// Does one spin cycle, as described in the puzzle input.
fn spin_cycle(map: &mut Map) {
    tilt(map, 0..map.height, 0..map.width, (0, -1)); // North
    tilt(map, 0..map.height, 0..map.width, (-1, 0)); // West
    tilt(map, (0..map.height).rev(), 0..map.width, (0, 1)); // South
    tilt(map, 0..map.height, (0..map.width).rev(), (1, 0)); // East
}
