/// Find the number of ways to beat each race in the input, and multiply them together.
pub fn one(input: &str) -> crate::Result<usize> {
    let time = input.lines().next().ok_or("insufficient input")?;
    let distance = input.lines().nth(1).ok_or("insufficient input")?;

    let time = time.split_whitespace().filter_map(|n| n.parse().ok());
    let distance = distance.split_whitespace().filter_map(|n| n.parse().ok());

    let races = time.zip(distance);

    Ok(races.map(ways_to_beat).product())
}

/// Find the number of ways to beat the race if we treat all digits on each input line as one big
/// number.
pub fn two(input: &str) -> crate::Result<usize> {
    let time = input.lines().next().ok_or("insufficient input")?;
    let distance = input.lines().nth(1).ok_or("insufficient input")?;

    let race = (parse_as_single(time)?, parse_as_single(distance)?);
    Ok(ways_to_beat(race))
}

/// Finds the amount of ways the given race can be beaten.
///
/// We know that the distance travelled follows a quadratic formula:
/// `f(speed) = -speed^2 + time * speed`
///
/// Also, we want to be further than the given distance:
/// `f(speed) > distance`
///
/// Resulting in this quadratic inequality with respect to `speed`:
/// `-speed^2 + time * speed - distance > 0`
///
/// So we can factor this any given `time` and `distance` and find the interval of `speed`s where
/// the left-hand side is positive for essentially free.
fn ways_to_beat((time, distance): Race) -> usize {
    let a = -1.0;
    let b = time as f64;
    let c = -(distance as f64);

    let x1 = (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
    let x2 = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);

    let (x1, x2) = (x1.ceil() as usize, x2.floor() as usize);
    1 + x2 - x1
}

/// (Time, Distance)
type Race = (usize, usize);

/// Parses a line of puzzle input as a single number, disregarding whitespace.
fn parse_as_single(line: &str) -> crate::Result<usize> {
    Ok(line
        .trim_start_matches(|c: char| c.is_alphabetic() || c == ':')
        .replace(' ', "")
        .parse()?)
}
