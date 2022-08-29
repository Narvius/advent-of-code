use std::collections::VecDeque;

/// Run the elf game with the given number of players and marbles, get the
/// highest score.
pub fn one(input: &str) -> crate::Result<usize> {
    let (players, marbles) = parse(input).ok_or("failed parse")?;
    highest_score(players, marbles)
}

/// Like [`one`], but with 100x more marbles.
pub fn two(input: &str) -> crate::Result<usize> {
    let (players, marbles) = parse(input).ok_or("failed parse")?;
    highest_score(players, marbles * 100)
}

/// Runs the game and returns the highest score. Built on top of a ring buffer deque.
fn highest_score(players: usize, marbles: usize) -> crate::Result<usize> {
    let mut circle = VecDeque::from([0]);
    let mut scores = vec![0; players];

    for marble in 1..=marbles {
        if marble % 23 == 0 {
            circle.rotate_right(7);
            scores[marble % players] += marble + circle.pop_back().unwrap_or(0);
            circle.rotate_left(1);
        } else {
            circle.rotate_left(1);
            circle.push_back(marble);
        }
    }

    scores.into_iter().max().ok_or_else(|| "no scores".into())
}

/// Parses player and marble counts from the input.
fn parse(input: &str) -> Option<(usize, usize)> {
    let mut tokens = input.split_ascii_whitespace();
    Some((tokens.next()?.parse().ok()?, tokens.nth(5)?.parse().ok()?))
}
