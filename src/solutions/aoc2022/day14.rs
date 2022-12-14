/// Find the amount of tiles of sand that settle before any of it touches the floor.
pub fn one(input: &str) -> crate::Result<usize> {
    find_settled_count(input, |p, floor| p.1 + 1 == floor)
}

/// Find the amount of tiles of sand that settle before the source becomes blocked.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(1 + find_settled_count(input, |p, _| p == (500, 0))?)
}

/// Counts the number of tiles of sand that settle before the `condition` is fulfilled. The
/// condition check gets two parameters: the coordinates on which sand settled, and the Y
/// coordinate of the floor.
fn find_settled_count(input: &str, condition: fn(Point, i32) -> bool) -> crate::Result<usize> {
    let (mut map, end_line) = parse(input)?;
    (0..)
        .map(move |_| drop_sand(&mut map, end_line))
        .position(|p| condition(p, end_line))
        .ok_or_else(|| "no result".into())
}

/// Drops a single bit of sand until it settles, marks it on the map, and returns the
/// coordinates it settled at.
fn drop_sand(map: &mut Map, end_line: i32) -> (i32, i32) {
    let mut pos = (500, 0);

    loop {
        let candidate = [(0, 1), (-1, 1), (1, 1)]
            .map(|(dx, dy)| (pos.0 + dx, pos.1 + dy))
            .into_iter()
            .find(|&(cx, cy)| cy < end_line && !map[cx as usize][cy as usize]);

        match candidate {
            Some(new_pos) => pos = new_pos,
            None => {
                map[pos.0 as usize][pos.1 as usize] = true;
                return pos;
            }
        }
    }
}

type Map = Vec<Vec<bool>>;
type Point = (i32, i32);

/// Parses the puzzle input into a map and the Y position of the infinite-width floor.
fn parse(input: &str) -> crate::Result<(Map, i32)> {
    fn range(a: i32, b: i32) -> std::ops::RangeInclusive<i32> {
        if a > b {
            b..=a
        } else {
            a..=b
        }
    }

    let lines: Vec<Vec<Point>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .filter_map(|s| {
                    let (x, y) = s.split_once(',')?;
                    Some((x.parse().ok()?, y.parse().ok()?))
                })
                .collect()
        })
        .collect();

    let max_y = 2 + lines
        .iter()
        .flat_map(|v| v.iter())
        .fold(0, |acc, p| acc.max(p.1));

    let mut map = vec![vec![false; max_y as usize]; 1000];
    for line in lines {
        for segment in line.windows(2) {
            for x in range(segment[0].0, segment[1].0) {
                map[x as usize][segment[0].1 as usize] = true;
            }
            for y in range(segment[0].1, segment[1].1) {
                map[segment[0].0 as usize][y as usize] = true;
            }
        }
    }

    Ok((map, max_y))
}
