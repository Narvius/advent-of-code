/// Find the number of tiles on the line y = 2000000 that cannot contain an unknown beacon.
///
/// Concept: Find all the slices of that line that are covered, sum the points. Slices can be
/// found easily thanks to the manhattan distance constraint (see [`scan_ranges`]).
pub fn one(input: &str) -> crate::Result<usize> {
    let scanline = 2000000;
    let sensors = parse(input);
    let beacons_on_scanline = sensors
        .iter()
        .filter_map(|&(_, (bx, by), _)| (by == scanline).then_some(bx))
        .collect::<std::collections::HashSet<_>>()
        .len();

    Ok(empty_squares_on_line(&sensors, scanline) - beacons_on_scanline)
}

/// Find the only tile with nonnegative coordinates less than 4 million that can contain an
/// unknown beacon.
///
/// Concept: Check all points along the edges of each beacon. This is possible because we
/// know that there's only one correct square; that means it MUST be adjacent to the
/// area covered by a beacon, because otherwise there would be more than one valid square.
pub fn two(input: &str) -> crate::Result<i64> {
    const LIMIT: i32 = 4000000;

    fn is_hidden_beacon(sensors: &[Sensor], p: Point) -> bool {
        if !(0..=LIMIT).contains(&p.0) || !(0..=LIMIT).contains(&p.1) {
            return false;
        }

        !sensors
            .iter()
            .any(|&(sensor, _, range)| manhattan_distance(sensor, p) <= range)
    }

    let sensors = parse(input);

    for &((x, y), _, range) in &sensors {
        let distance = range + 1;
        for dx in -distance..=distance {
            let dy = distance.abs() - dx.abs();
            if is_hidden_beacon(&sensors, (x + dx, y + dy)) {
                return Ok((x + dx) as i64 * LIMIT as i64 + (y + dy) as i64);
            }
            if is_hidden_beacon(&sensors, (x + dx, y - dy)) {
                return Ok((x + dx) as i64 * LIMIT as i64 + (y - dy) as i64);
            }
        }
    }

    Err("no result".into())
}

/// Data about one sensor; contains its own position, the position of the nearest beacon, and
/// the manhattan distance between the two.
type Sensor = (Point, Point, i32);
type Point = (i32, i32);

/// Counts the number of squares with the given Y position that are *guaranteed* to not
/// contain an unknown beacon.
///
/// This is done by first finding all relevant X position ranges, then counting the number
/// of points in those ranges. The counting is implemented more efficiently than just splatting
/// them all in a HashSet and counting the size, though.
fn empty_squares_on_line(sensors: &[Sensor], line: i32) -> usize {
    let mut ranges: Vec<_> = sensors
        .iter()
        .map(move |&((x, y), _, range)| {
            let max_delta = range - (line - y).abs();
            (x - max_delta, x + max_delta)
        })
        .collect();
    let mut result = 0;
    let mut counted_up_to = i32::MIN;

    ranges.sort_unstable_by_key(|&(min, _)| min);

    for (min, max) in ranges {
        let actual_min = counted_up_to.max(min);
        counted_up_to = counted_up_to.max(max + 1);
        result += (actual_min..max + 1).len();
    }

    result
}

/// The manhattan (taxicab) distance between two points.
fn manhattan_distance((ax, ay): Point, (bx, by): Point) -> i32 {
    (ax - bx).abs() + (ay - by).abs()
}

/// Parses the puzzle input into a series of pairs containing positions of each sensor and
/// the beacon closest to them.
fn parse(input: &str) -> Vec<Sensor> {
    fn parse_point(s: &str) -> Option<Point> {
        let (x, y) = s.split_once(", ")?;
        Some((x[2..].parse().ok()?, y[2..].parse().ok()?))
    }

    input
        .lines()
        .filter_map(|line| {
            let s = line.strip_prefix("Sensor at ")?;
            let (sensor, beacon) = s.split_once(": closest beacon is at ")?;
            let (sensor, beacon) = (parse_point(sensor)?, parse_point(beacon)?);
            Some((sensor, beacon, manhattan_distance(sensor, beacon)))
        })
        .collect()
}
