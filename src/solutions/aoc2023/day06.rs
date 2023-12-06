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
fn ways_to_beat((time, distance): Race) -> usize {
    // Binary search; start at the exact middle (which is pretty much guaranteed to win), then take
    // steps down until no longer winning, then steps up until winning again.

    let wins = |speed| speed * (time - speed) > distance;

    let mut speed = time / 2;
    let mut step = speed / 2;

    while step > 0 {
        match wins(speed) {
            true => speed -= step,
            false => speed += step,
        }
        step /= 2;
    }

    // Microadjust up and down so we're exactly on the lowest winning speed.
    while !wins(speed) {
        speed += 1;
    }

    while wins(speed - 1) {
        speed -= 1;
    }

    // Since the winning formula is a quadratic inequality (-speed^2 + time * speed > distance), we
    // know that there's a symmetrical slice of the parabola above `y = distance`. This is the
    // amount of numbers on that slice.
    1 + time - 2 * speed
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
