/// Find the position card 2019 will end up at after doing the shuffle.
pub fn one(input: &str) -> crate::Result<i128> {
    const CARDS: i128 = 10007;
    let (a, b) = combine_shuffle(parse(input)?, false, CARDS);
    Ok(normalize(a * 2019 + b, CARDS))
}

/// Find which card will be at index 2020 after an absurd number of shuffles
/// with an absurdly large deck.
pub fn two(input: &str) -> crate::Result<i128> {
    const CARDS: i128 = 119315717514047;
    const SHUFFLES: i128 = 101741582076661;
    const TARGET: i128 = 2020;

    let mut f = combine_shuffle(parse(input)?, true, CARDS);
    let mut all_shuffles_combined = (1, 0);

    // f = 1 shuffle
    // f o f = f' = 2 shuffles
    // f' o f' = f'' = 4 shuffles
    // f'' o f'' = f''' = 8 shuffles
    // and so on
    //
    // We use this to build `all_shuffles_combined` up into a linear function
    // that does `SHUFFLES` shuffles in one go.
    let mut mask = 1;
    while mask <= SHUFFLES {
        if mask & SHUFFLES > 0 {
            all_shuffles_combined = compose_modular_linear_fn(all_shuffles_combined, f, CARDS);
        }
        f = compose_modular_linear_fn(f, f, CARDS);
        mask <<= 1;
    }

    let (a, b) = all_shuffles_combined;
    Ok(normalize(a * TARGET + b, CARDS))
}

/// Collapses an entire shuffle sequence into a single linear function that can
/// be used to compute the new position of a card after that shuffle.
///
/// If `reversed`, it instead returns a linear function that can be used to
/// find the *previous* position of a card, before that shuffle.
fn combine_shuffle(shuffle: Vec<Action>, reversed: bool, cards: i128) -> (i128, i128) {
    let shuffle: Box<dyn Iterator<Item = Action>> = match reversed {
        true => Box::new(shuffle.into_iter().rev()),
        false => Box::new(shuffle.into_iter()),
    };

    shuffle
        .map(|a| action_to_linear_fn(a, reversed, cards))
        .fold((1, 0), |f, g| compose_modular_linear_fn(f, g, cards))
}

/// Converts a shuffle action into a coefficient pair for a linear function.
/// In other words, each of those shuffle actions can be represented as a
/// formula that computes the new position after that shuffle; and this
/// returns that formula, of the form `ax + b`.
fn action_to_linear_fn(action: Action, reversed: bool, cards: i128) -> (i128, i128) {
    use Action as A;
    match (action, reversed) {
        // Straight incremental deal results in the position being stretched
        // out `increment` times.
        (A::Increment(increment), false) => (increment as i128, 0),
        // Cutting results in being shifted backwards.
        (A::Cut(cut), false) => (1, -cut as i128),
        // Dealing reverses the own position with regards to the deck.
        (A::Deal, false) => (-1, cards - 1),

        // Reversed increment does... something. This is derived from some
        // algebra; you can invert `y = ax + b` by swapping `x` and `y`, then
        // solving for `y`. This is that translated into code.
        (A::Increment(increment), true) => {
            let increment = increment as i128;
            let mut multiplier = 1;
            let mut result = 0;
            while multiplier % increment != 0 {
                result += multiplier / increment;
                multiplier %= increment;
                multiplier += cards;
            }
            (result + multiplier / increment, 0)
        }
        // Reversed cut results in being shifted forwards.
        (A::Cut(cut), true) => (1, cut as i128),
        // Reversed dealing is identical to non-reversed dealing.
        (A::Deal, true) => (-1, cards - 1),
    }
}

/// Given two functions `y = ax + b` and `y = cx + d`, returns a combined
/// function that has the same effect as applying one, then the other.
fn compose_modular_linear_fn(
    (a, b): (i128, i128),
    (c, d): (i128, i128),
    modulo: i128,
) -> (i128, i128) {
    (normalize(a * c, modulo), normalize(b * c + d, modulo))
}

/// Given a number, makes sure it is within `(0..modulo)`.
fn normalize(n: i128, modulo: i128) -> i128 {
    if n >= 0 {
        n % modulo
    } else {
        modulo - (-n % modulo)
    }
}

/// The possible shuffle actions, as described in the puzzle input.
enum Action {
    Increment(usize),
    Cut(i32),
    Deal,
}

/// Parses the puzzle input into a list of [`Action`]s.
fn parse(input: &str) -> crate::Result<Vec<Action>> {
    input
        .lines()
        .map(|line| {
            Ok(if let Some(n) = line.strip_prefix("deal with increment ") {
                Action::Increment(n.parse()?)
            } else if let Some(n) = line.strip_prefix("cut ") {
                Action::Cut(n.parse()?)
            } else if line == "deal into new stack" {
                Action::Deal
            } else {
                Err("invalid line")?
            })
        })
        .collect()
}
