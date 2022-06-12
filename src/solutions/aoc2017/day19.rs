/// Return the string seen after walking the routing diagram.
pub fn one(input: &str) -> Result<String, String> {
    walk(input).map(|(seen, _)| seen)
}

/// Return the number of steps needed to walk the routing diagram.
pub fn two(input: &str) -> Result<String, String> {
    walk(input).map(|(_, steps)| steps.to_string())
}

/// Walks the routing diagram and returns the seen string as well as the number of steps
/// taken.
fn walk(diagram: &str) -> Result<(String, usize), String> {
    let map = parse(diagram);
    let mut pos = {
        let starting_x = map[0]
            .iter()
            .position(|&c| c.is_some())
            .ok_or_else(|| "no starting position".to_owned())? as i32;
        (starting_x, 0)
    };
    let mut dir = (0, 1);
    let mut seen = String::new();
    let mut steps = 1; // walking ONTO the diagram counts as a step

    while let Some((d, t)) = next_step(&map, pos, dir) {
        (pos, dir, steps) = ((pos.0 + d.0, pos.1 + d.1), d, steps + 1);
        if let Some(c) = t {
            seen.push(c as char);
        }
    }

    Ok((seen, steps))
}

/// Returns the direction and tile for the next step.
fn next_step(map: &Map, p: (i32, i32), (x, y): (i32, i32)) -> Option<((i32, i32), Option<u8>)> {
    [(x, y), (y, x), (-y, -x)]
        .into_iter()
        .filter_map(|d| Some((d, map[(p.1 + d.1) as usize][(p.0 + d.0) as usize]?)))
        .next()
}

/// The type for a map used in this puzzle.
/// 
/// From outermost to innermost type:
/// - vector of rows
/// - vector of cells
/// - option: `Some` if walkable, `None` if not
/// - option<u8>: contained letter
type Map = Vec<Vec<Option<Option<u8>>>>;

/// Parses the puzzle input into a two-dimensional map.
fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|c| match c {
                    b'|' | b'-' | b'+' => Some(None),
                    b' ' => None,
                    _ => Some(Some(c)),
                })
                .collect()
        })
        .collect()
}
