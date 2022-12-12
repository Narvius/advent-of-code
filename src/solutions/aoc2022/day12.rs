use std::collections::{HashSet, VecDeque};

/// Find the length of the shortest path between the start and end tiles.
pub fn one(input: &str) -> crate::Result<usize> {
    let (start, end, data) = parse(input);
    Ok(bfs(data, start, Box::new(move |p, _| p == end)))
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
    Ok(bfs(data, start, Box::new(|_, height| height == b'z')))
}

/// A breadth-first search that returns the length of the path that was found.
fn bfs(data: Data, start: Point, end: EndCheck) -> usize {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::from([start]);
    let mut result = usize::MAX;

    while let Some(((x, y), s)) = queue.pop_front() {
        let Some(source_height) = get_height(&data, (x, y)) else { continue; };

        if end((x, y), source_height) {
            result = result.min(s);
            continue;
        }

        for p in [(-1, 0), (0, -1), (1, 0), (0, 1)].map(|(dx, dy)| (x + dx, y + dy)) {
            let Some(target_height) = get_height(&data, p) else { continue; };
            if target_height <= (source_height + 1) && visited.insert(p) {
                queue.push_back((p, s + 1));
            }
        }
    }

    result
}

/// Gets the height value for a given point.
fn get_height(data: &Data, (x, y): Point) -> Option<u8> {
    data.get(y as usize)?.get(x as usize).copied()
}

/// Heightmap data.
type Data = Vec<Vec<u8>>;
/// A function that checks if a point is the end.
type EndCheck = Box<dyn Fn(Point, u8) -> bool>;
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
