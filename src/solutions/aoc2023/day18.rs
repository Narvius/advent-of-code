/// Find the size of the lava lagoon resulting from digging according to the given instructions.
pub fn one(input: &str) -> crate::Result<i64> {
    Ok(lagoon_size(input, false))
}

/// Find the size of the lava lagoon using the encoded instructions.
pub fn two(input: &str) -> crate::Result<i64> {
    Ok(lagoon_size(input, true))
}

/// Calculates the size of the lava lagoon after digging according to the instructions and then
/// hollowing the resulting shape out.
///
/// Just like [day 10], uses the [shoelace formula] and [Pick's theorem]; though the latter is
/// slightly adjusted to count the edge too, instead of only the points inside.
///
/// [day 10]: crate::solutions::aoc2023::day10::two
/// [shoelace formula]: https://en.wikipedia.org/wiki/Shoelace_formula
/// [Pick's theorem]: https://en.wikipedia.org/wiki/Pick%27s_theorem
fn lagoon_size(input: &str, decode: bool) -> i64 {
    let mut points = 1;
    let mut pos = (0, 0);
    let mut area = 0;

    for (dir, count) in parse(input, decode) {
        points += count;
        let next_pos = match dir {
            b'R' | b'0' => (pos.0 + count, pos.1),
            b'D' | b'1' => (pos.0, pos.1 + count),
            b'L' | b'2' => (pos.0 - count, pos.1),
            b'U' | b'3' => (pos.0, pos.1 - count),
            _ => unreachable!(),
        };
        area += pos.0 * next_pos.1 - pos.1 * next_pos.0;
        pos = next_pos;
    }

    area / 2 + points / 2 + 1
}

/// Parses the puzzle input into (direction byte, distance) pairs. If `decode` is given, takes the
/// instruction encoded in the colour instead.
fn parse(input: &str, decode: bool) -> impl Iterator<Item = (u8, i64)> + '_ {
    input.lines().filter_map(move |line| {
        let (dir, rest) = line.split_once(' ')?;
        let (count, encoded) = rest.split_once(' ')?;
        Some(if decode {
            let val = encoded.trim_matches(&['(', ')', '#'][..]);
            let count = i64::from_str_radix(&val[0..5], 16).ok()?;
            (val.as_bytes()[5], count)
        } else {
            (dir.as_bytes()[0], count.parse().ok()?)
        })
    })
}
