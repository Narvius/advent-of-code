/// Calculate how badly you're hitting the firewall if travelling with no delay.
pub fn one(input: &str) -> Result<String, String> {
    Ok(parse(input)
        .filter(|&layer| is_hit(0, layer))
        .map(|(i, s)| i * s)
        .sum::<usize>()
        .to_string())
}

/// Find the lowest delay that allows travelling through the firewall without hits.
pub fn two(input: &str) -> Result<String, String> {
    let items: Vec<_> = parse(input).collect();
    let mut delay = 0;
    loop {
        delay += 1;

        if !items.iter().any(|&layer| is_hit(delay, layer)) {
            return Ok(delay.to_string());
        }
    }
}

/// Checks whether the given layer will be hit if a starting `delay` is used.
fn is_hit(delay: usize, (depth, range): (usize, usize)) -> bool {
    (depth + delay) % (2 * (range - 1)) == 0
}

/// Parses the puzzle input into a series of firewall layers.
fn parse(input: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    input.lines().filter_map(|line| {
        let (index, size) = line.split_once(": ")?;
        Some((index.parse().ok()?, size.parse().ok()?))
    })
}
