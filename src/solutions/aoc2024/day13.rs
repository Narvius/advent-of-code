/// Find the total cost required to win all winnable machines.
pub fn one(input: &str) -> crate::Result<i128> {
    Ok(parse(input).filter_map(win_cost).sum())
}

/// Find the total cost required to win all winnable machines if 10000000000000 is added to both
/// target coordinates of every machine.
pub fn two(input: &str) -> crate::Result<i128> {
    Ok(parse(input)
        .map(|(x1, y1, x2, y2, x, y)| (x1, y1, x2, y2, x + 10000000000000, y + 10000000000000))
        .filter_map(win_cost)
        .sum())
}

/// Return the win cost for a machine from the input; returns `None` if it isn't winnable.
fn win_cost((x1, y1, x2, y2, x, y): Machine) -> Option<i128> {
    // Just algebra. You can derive it from:
    // x = N * x1 + M * x2
    // y = N * y1 + M * y2

    let n_num = x2 * y - x * y2;
    let n_denom = y1 * x2 - x1 * y2;

    if n_num % n_denom != 0 {
        return None;
    }

    let n = n_num / n_denom;

    let m_num = x - n * x1;
    let m_denom = x2;

    if m_num % m_denom != 0 {
        return None;
    }

    Some(3 * n + m_num / m_denom)
}

/// Parses the puzzle input into a list of machine parameters.
fn parse(input: &str) -> impl Iterator<Item = Machine> + '_ {
    input.split("\r\n\r\n").filter_map(|chunk| {
        let mut lines = chunk.lines().filter_map(|line| {
            let (_, v) = line.split_once(": ")?;
            let (x, y) = v.split_once(", ")?;
            Some((x[2..].parse::<i128>().ok()?, y[2..].parse::<i128>().ok()?))
        });

        let (x1, y1) = lines.next()?;
        let (x2, y2) = lines.next()?;
        let (x, y) = lines.next()?;

        Some((x1, y1, x2, y2, x, y))
    })
}

type Machine = (i128, i128, i128, i128, i128, i128);
