/// Find the games that are possible with the provided amount of cubes.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(parse(input)
        .enumerate()
        .filter(|&(_, (r, g, b))| r <= 12 && g <= 13 && b <= 14)
        .map(|(n, _)| n + 1)
        .sum())
}

/// For each game, find the minimum number of cubes required.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(parse(input).map(|(r, g, b)| r * g * b).sum())
}

/// Parse the puzzle input in such a way that the order from the input file is preserved (making
/// returning the Game Id unnecessary), and the returned value is a 3-tuple of the highest amount
/// of red, green and blue cubes shown, respectively.
fn parse(input: &str) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    input.lines().filter_map(|line| {
        let (_, subsets) = line.split_once(": ")?;
        let (mut r, mut g, mut b) = (0, 0, 0);
        for entry in subsets.split(&[':', ';', ','][..]) {
            let (n, color) = entry.trim().split_once(' ')?;
            match color {
                "red" => r = r.max(n.parse().ok()?),
                "green" => g = g.max(n.parse().ok()?),
                "blue" => b = b.max(n.parse().ok()?),
                _ => {}
            };
        }
        Some((r, g, b))
    })
}
