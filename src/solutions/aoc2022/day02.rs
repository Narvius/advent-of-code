/// Follow the strategy guide assuming that column 2 indicates your hand, find total score.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(input
        .lines()
        .map(|line| {
            (line.as_bytes()[2] - b'X' + 1) as i32
                + match (line.as_bytes()[0], line.as_bytes()[2]) {
                    // We win.
                    (b'A', b'Y') | (b'B', b'Z') | (b'C', b'X') => 6,
                    // Draw.
                    (b'A', b'X') | (b'B', b'Y') | (b'C', b'Z') => 3,
                    // Loss.
                    _ => 0,
                }
        })
        .sum())
}

/// Follow the strategy guide knowing that column 2 indicates the outcome of that round;
/// find total score.
pub fn two(input: &str) -> crate::Result<i32> {
    Ok(input
        .lines()
        .map(|line| {
            let (opponent, outcome) = (line.as_bytes()[0] - b'A', line.as_bytes()[2] - b'X');
            match outcome {
                // We have to lose, so we pick the option 1 lower than the opponent.
                0 => 1 + (opponent as i32 + 2) % 3,
                // Draw. Same option as opponent.
                1 => 3 + 1 + opponent as i32,
                // We have to win, so we pick the option 1 higher than the opponent.
                _ => 6 + 1 + (opponent as i32 + 1) % 3,
            }
        })
        .sum())
}
