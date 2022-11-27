use std::collections::HashMap;

/// Play the game using a deterministic d100 (as described in the puzzle). Find the score of the
/// losing player multiplied by the amount of rolls done.
pub fn one(input: &str) -> crate::Result<usize> {
    let (mut p1, mut p2) = parse(input).ok_or("failed to parse input")?;
    let (mut s1, mut s2) = (0, 0);
    let (mut die, mut rolls) = (0, 0);
    let mut player2next = false;

    loop {
        let (p, s, other_s) = if player2next {
            (&mut p2, &mut s2, &mut s1)
        } else {
            (&mut p1, &mut s1, &mut s2)
        };

        rolls += 3;
        let to_move: usize = [die + 1, (die + 1) % 100 + 1, (die + 2) % 100 + 1]
            .into_iter()
            .sum();

        *p = (*p + to_move) % 10;
        *s += *p + 1;
        die = (die + 3) % 100;
        player2next = !player2next;

        if *s >= 1000 {
            return Ok(*other_s * rolls);
        }
    }
}

/// Play the game using a d3; count how many possible games each player can win, and get the higher
/// of those two numbers.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut universes =
        HashMap::from([(State::new(parse(input).ok_or("failed to parse input")?), 1)]);

    let (mut w1, mut w2) = (0, 0);
    while !universes.is_empty() {
        let key = *universes.keys().next().unwrap();
        let (state, times) = universes.remove_entry(&key).unwrap();
        let (subw1, subw2, new) = split_roll(state);
        w1 += times * subw1;
        w2 += times * subw2;
        for (subtimes, state) in new {
            *universes.entry(state).or_insert(0) += times * subtimes;
        }
    }

    Ok(w1.max(w2))
}

/// A game state.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    score1: u8,
    pos1: u8,
    score2: u8,
    pos2: u8,
    player2next: bool,
}

impl State {
    fn new((p1, p2): (usize, usize)) -> State {
        State {
            score1: 0,
            pos1: p1 as u8,
            score2: 0,
            pos2: p2 as u8,
            player2next: false,
        }
    }
}

/// Given a game state, advances it by one turn; then returns the number of winning universes for
/// players 1 and 2 respectively, as well as a list of undecided universes.
fn split_roll(state: State) -> (usize, usize, Vec<(usize, State)>) {
    // We are spawning 27 timelines.
    // Moving by 3 (1x), 4 (3x), 5 (6x), 6 (x7), 7 (x6), 8 (x3) and 9 (x1). These are simply all
    // the possible outcomes of 3d3.

    let (mut wins1, mut wins2, mut v) = (0, 0, vec![]);
    for (roll, times) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let mut state = state;
        let (score, pos, wins) = if state.player2next {
            (&mut state.score2, &mut state.pos2, &mut wins2)
        } else {
            (&mut state.score1, &mut state.pos1, &mut wins1)
        };

        *pos = (*pos + roll) % 10;
        *score += *pos + 1;
        if *score >= 21 {
            *wins += times;
        } else {
            state.player2next = !state.player2next;
            v.push((times, state));
        }
    }
    (wins1, wins2, v)
}

/// Parses the starting positions for each player from the input.
fn parse(input: &str) -> Option<(usize, usize)> {
    let mut lines = input.lines();
    Some((
        lines.next()?.split_once(": ")?.1.parse::<usize>().ok()? - 1,
        lines.next()?.split_once(": ")?.1.parse::<usize>().ok()? - 1,
    ))
}
