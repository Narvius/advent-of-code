/// Counts how many lights are on after all instructions are executed.
pub fn one(input: &str) -> crate::Result<String> {
    Ok(run_lights(
        input,
        |b| if b { 1 } else { 0 },
        |b, c| match c {
            "toggle" => !b,
            "on" => true,
            "off" => false,
            _ => unreachable!(),
        },
    )
    .to_string())
}

/// Counts the total brightness after all instructions are executed.
pub fn two(input: &str) -> crate::Result<String> {
    Ok(run_lights(
        input,
        |b| b,
        |b, c| match c {
            "toggle" => b + 2,
            "on" => b + 1,
            "off" => b.saturating_sub(1),
            _ => unreachable!(),
        },
    )
    .to_string())
}

/// Executes the shared puzzle logic. For each line of inputs, runs `step` on all relevant
/// coordinates, and at the end sums the outputs of `collect` for each position.
fn run_lights<E>(input: &str, collect: fn(E) -> usize, step: fn(E, &str) -> E) -> usize
where
    E: Copy + Default,
{
    let mut lights = vec![E::default(); 1_000_000];
    for (c, (x0, y0), (x1, y1)) in input.lines().filter_map(parse) {
        for y in y0..=y1 {
            for x in x0..=x1 {
                let val = &mut lights[1000 * y + x];
                *val = step(*val, c);
            }
        }
    }
    lights.into_iter().map(collect).sum()
}

type Line<'a> = (&'a str, (usize, usize), (usize, usize));

/// Parses a line of puzzle input.
fn parse(line: &str) -> Option<Line> {
    fn make<'a>(command: &'a str, a: &str, b: &str) -> Option<Line<'a>> {
        let (x0, y0) = a.split_once(',')?;
        let (x1, y1) = b.split_once(',')?;

        Some((
            command,
            (x0.parse().ok()?, y0.parse().ok()?),
            (x1.parse().ok()?, y1.parse().ok()?),
        ))
    }

    let line: Vec<_> = line.split(' ').collect();
    match line.len() {
        4 => make(line[0], line[1], line[3]),
        5 => make(line[1], line[2], line[4]),
        _ => None,
    }
}
