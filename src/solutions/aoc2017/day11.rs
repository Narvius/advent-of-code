/// Finds the distance from origin after walking all the way.
pub fn one(input: &str) -> crate::Result<i32> {
    match distances(input).last() {
        Some(d) => Ok(d),
        None => Err("no steps taken".into()),
    }
}

/// Finds the furthest away from origin at any point during the trip.
pub fn two(input: &str) -> crate::Result<i32> {
    match distances(input).max() {
        Some(d) => Ok(d),
        None => Err("no steps taken".into()),
    }
}

/// Follows the steps from the input and returns the distance from origin after each step.
fn distances(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.split(',').scan((0i32, 0i32), |(x, y), dir| {
        let (dx, dy) = match dir {
            "n" => (0, -1),
            "ne" => (1, -1),
            "se" => (1, 0),
            "s" => (0, 1),
            "sw" => (-1, 1),
            "nw" => (-1, 0),
            _ => (0, 0),
        };
        (*x, *y) = (*x + dx, *y + dy);
        Some(x.abs() + y.abs())
    })
}
