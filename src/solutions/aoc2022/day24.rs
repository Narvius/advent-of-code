use std::collections::{HashSet, VecDeque};

/// Find the length of the shortest path to the exit.
pub fn one(input: &str) -> crate::Result<usize> {
    let map = parse(input).ok_or("parse failed")?;
    Ok(map.shortest_path(1))
}

/// Find the length of the shortest path to the exit, start and exit again.
pub fn two(input: &str) -> crate::Result<usize> {
    let map = parse(input).ok_or("parse failed")?;
    Ok(map.shortest_path(3))
}

/// A point in 2D space.
type Point = (i32, i32);

/// A map of the valley.  
struct Map {
    /// `data[z]` contains the position of all blizzards after `z` steps,
    /// thus forming a sort-of 3D map. After `width * height`, the
    /// blizzards start repeating positions.
    data: Vec<HashSet<Point>>,
    width: usize,
    height: usize,
    start: (i32, i32),
    end: (i32, i32),
}

impl Map {
    /// Finds the length of the shortest path to alternatingly reach the end an start,
    /// `target_completions` times.
    ///
    /// This is done via a breadth-first search, treating the map like a 3D map,
    /// where the third dimension is each unique blizzard state (it loops after
    /// `width * height` steps). Other than that, the implementation is relatively
    /// unremarkable.
    fn shortest_path(&self, target_completions: usize) -> usize {
        let mut queue = VecDeque::from([(self.start.0, self.start.1, 0, 0)]);
        let mut visited = HashSet::new();
        let (xs, ys) = (0..self.width as i32, 0..self.height as i32);

        while let Some((x, y, steps, completions)) = queue.pop_front() {
            let cycle = steps % self.data.len();
            let target = if completions % 2 == 0 {
                self.end
            } else {
                self.start
            };

            if self.data[cycle].contains(&(x, y)) || !visited.insert((x, y, cycle, completions)) {
                continue;
            }

            if (x, y) == target && completions == target_completions {
                return steps;
            }

            let neighbours = [(0, 1), (1, 0), (0, 0), (0, -1), (-1, 0)]
                .into_iter()
                .map(|(dx, dy)| {
                    let hit_end = usize::from((x + dx, y + dy) == target);
                    (x + dx, y + dy, steps + 1, completions + hit_end)
                })
                .filter(|&(x, y, _, _)| {
                    (x, y) == self.start
                        || (x, y) == self.end
                        || (xs.contains(&x) && ys.contains(&y))
                });

            queue.extend(neighbours);
        }

        unreachable!()
    }
}

/// Returns a list of the positions of all blizzards at every point.
fn parse(input: &str) -> Option<Map> {
    let map: Vec<_> = input
        .lines()
        .filter(|line| !line.contains("###"))
        .map(|line| line.trim_matches('#'))
        .collect();
    let cycles = map.len() * map[0].len();

    let mut data = vec![HashSet::new(); cycles];
    for i in 0..cycles as i32 {
        for y in 0..map.len() as i32 {
            for x in 0..map[y as usize].len() as i32 {
                let p = match map[y as usize].as_bytes()[x as usize] {
                    b'<' => Some(((x - i).rem_euclid(map[y as usize].len() as i32), y)),
                    b'^' => Some((x, (y - i).rem_euclid(map.len() as i32))),
                    b'>' => Some(((x + i).rem_euclid(map[y as usize].len() as i32), y)),
                    b'v' => Some((x, (y + i).rem_euclid(map.len() as i32))),
                    _ => None,
                };

                data[i as usize].extend(p);
            }
        }
    }
    Some(Map {
        data,
        width: map[0].len(),
        height: map.len(),
        start: (0, -1),
        end: (map[0].len() as i32 - 1, map.len() as i32),
    })
}
