/// Find the number of trees on the given slope.
pub fn one(input: &str) -> crate::Result<usize> {
    let map: Map = input.lines().collect();
    Ok(trees_on_slope(&map, (3, 1)))
}

/// Find the product of the numbers of trees on all given slopes.
pub fn two(input: &str) -> crate::Result<usize> {
    let map: Map = input.lines().collect();
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .map(|p| trees_on_slope(&map, p))
        .into_iter()
        .reduce(|a, b| a * b)
        .ok_or_else(|| "no result".into())
}

type Map<'a> = Vec<&'a str>;

/// Produces a list of all coordinates reached when jumping by `(dx, dy)` every step.
/// `(mx, my)` is the bounds of the map. When `my` is reached, the slope terminates; when
/// `mx` is reached, it wraps back around to 0.
fn slope((dx, dy): (i32, i32), (mx, my): (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    (0..(my / dy)).map(move |n| ((dx * n) % mx, dy * n))
}

/// Counts the number of trees on a slope.
fn trees_on_slope(map: &Map, d: (i32, i32)) -> usize {
    slope(d, (map[0].len() as i32, map.len() as i32))
        .filter(|&(x, y)| map[y as usize].as_bytes()[x as usize] == b'#')
        .count()
}
