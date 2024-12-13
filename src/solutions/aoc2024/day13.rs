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
    // x = A * x1 + B * x2
    // y = A * y1 + B * y2

    let a_num = x2 * y - x * y2;
    let a_denom = y1 * x2 - x1 * y2;

    if a_num % a_denom != 0 {
        return None;
    }

    let a = a_num / a_denom;

    let b_num = x - a * x1;
    let b_denom = x2;

    if b_num % b_denom != 0 {
        return None;
    }

    Some(3 * a + b_num / b_denom)
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
