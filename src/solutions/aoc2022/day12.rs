use std::collections::{HashSet, VecDeque};

/// Find the length of the shortest path between the start and end tiles.
pub fn one(input: &str) -> crate::Result<usize> {
    let (start, end, data) = parse(input);
    shortest_path(data, start, Some(end))
}

/// Find the length of the shortest path between the end tile and *any* lowest-elevation tile.
///
/// It's easier to search from the end tile, rather than from each lowest-elevation tile.
/// However, the neighbour check isn't symmetric; going from the highest point, we could just
/// jump down and be there in like 20 steps. So in order to search from the end but still get
/// the correct result, we *invert* the heightmap: The lowest elevation becomes the highest.
///
/// This way, searching from the end behaves correctly, only taking steps that would be possible
/// going the other way.
pub fn two(input: &str) -> crate::Result<usize> {
    let (_, start, mut data) = parse(input);
    for tile in data.iter_mut().flat_map(|v| v.iter_mut()) {
        *tile = b'z' - *tile;
    }
    shortest_path(data, start, None)
}

/// A breadth-first search that returns the length of the path that was found. If no `end` is
/// given, the search will finish once a height of 'z' is reached.
fn shortest_path(data: Data, start: Point, end: Option<(i32, i32)>) -> crate::Result<usize> {
    // Gets the height at a given point.
    fn height(data: &Data, (x, y): Point) -> Option<u8> {
        data.get(y as usize)?.get(x as usize).copied()
    }

    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::from([start]);

    while let Some(((x, y), s)) = queue.pop_front() {
        let Some(source_height) = height(&data, (x, y)) else { continue; };

        if end.map(|p| p == (x, y)).unwrap_or(source_height == b'z') {
            return Ok(s);
        }

        for p in [(-1, 0), (0, -1), (1, 0), (0, 1)].map(|(dx, dy)| (x + dx, y + dy)) {
            let Some(target_height) = height(&data, p) else { continue; };
            if target_height <= (source_height + 1) && visited.insert(p) {
                queue.push_back((p, s + 1));
            }
        }
    }

    Err("no path found".into())
}

/// Heightmap data.
type Data = Vec<Vec<u8>>;
/// A 2D point in space.
type Point = (i32, i32);

/// Parses the puzzle input, returning the start/end points and the heightmap data.
fn parse(input: &str) -> (Point, Point, Data) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let data = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, c)| match c {
                    b'S' => {
                        start = (x as i32, y as i32);
                        0
                    }
                    b'E' => {
                        end = (x as i32, y as i32);
                        b'z' - b'a'
                    }
                    c => c - b'a',
                })
                .collect::<Vec<_>>()
        })
        .collect();

    (start, end, data)
}
