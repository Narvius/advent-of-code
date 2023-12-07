/// Order poker hands by strength, and calculate a checksum.
pub fn one(input: &str) -> crate::Result<usize> {
    total_winnings(input, false)
}

/// order poker hands by strength, and calculate a checksum; but treat J cards as jokers.
pub fn two(input: &str) -> crate::Result<usize> {
    total_winnings(input, true)
}

/// Finds the total winnings of a set of poker hands as per the puzzle description.
fn total_winnings(input: &str, joker: bool) -> crate::Result<usize> {
    let mut items: Vec<_> = parse(input)
        .map(|(hand, bid)| (hand_value(hand, joker), bid))
        .collect();

    items.sort_unstable_by_key(|item| item.0);
    Ok(items
        .into_iter()
        .enumerate()
        .map(|(n, (_, bid))| (n + 1) * bid)
        .sum())
}

const ORDER: &[u8] = b"23456789TJQKA";
const JOKER_ORDER: &[u8] = b"J23456789TQKA";

/// Creates a comparison key to sort poker hands by.
fn hand_value(hand: &[u8], joker: bool) -> (u8, [usize; 5]) {
    // Count how much of which card we have.
    let mut joker_count = 0;
    let mut counts = vec![];
    let mut seen = vec![];
    for &c in hand {
        if joker && c == b'J' {
            joker_count += 1;
        } else if let Some(p) = seen.iter().position(|&card| c == card) {
            counts[p] += 1;
        } else {
            seen.push(c);
            counts.push(1);
        }
    }

    // Adding the joker count to the highest count will always produce the best hand.
    counts.sort_unstable();
    if let Some(c) = counts.last_mut() {
        *c += joker_count;
    } else {
        counts.push(joker_count);
    }

    // Determine hand strength based purely on card counts. Because we sorted earlier, we can do
    // direct slice matching.
    let strength = match counts.as_slice() {
        [1, 1, 1, 1, 1] => 1,
        [1, 1, 1, 2] => 2,
        [1, 2, 2] => 3,
        [1, 1, 3] => 4,
        [2, 3] => 5,
        [1, 4] => 6,
        [5] => 7,
        _ => 0,
    };

    // Convert raw cards into their strength.
    let order = if joker { JOKER_ORDER } else { ORDER };
    let card_value = |c| order.iter().position(|&b| c == b).unwrap();
    let hand = [hand[0], hand[1], hand[2], hand[3], hand[4]].map(card_value);

    (strength, hand)
}

/// Parses the puzzle input into a hand and the relevant bid value.
fn parse(input: &str) -> impl Iterator<Item = (&[u8], usize)> + '_ {
    input.lines().filter_map(|line| {
        let (hand, bid) = line.split_once(' ')?;
        Some((hand.as_bytes(), bid.parse().ok()?))
    })
}
