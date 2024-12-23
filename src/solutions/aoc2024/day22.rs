use std::{collections::HashMap, mem::replace};

/// Find the sum of the 2000th generated secret number of each buyer.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(parse(input)
        .map(|n| (0..2000).fold(n, |n, _| step(n)))
        .sum())
}

/// Find the best amount of bananas obtainable from selling through they negotiating monkey.
///
/// For each buyer, finds all possible sales; and then simply sums up the price for each
/// negotiation pattern across every buyer, and picks the highest one.
pub fn two(input: &str) -> crate::Result<usize> {
    parse(input)
        .fold(HashMap::<_, usize>::new(), |mut earnings, n| {
            // Collect all prices (secret number % 10).
            let ns: Vec<_> = (0..=2000)
                .scan(n, |s, _| Some((replace(s, step(*s)) % 10) as i32))
                .collect();

            // The first time a pattern of 4 consecutive differences shows up, mark the price.
            let mut first_prices = HashMap::new();
            for w in ns.windows(5) {
                let pattern = (w[1] - w[0], w[2] - w[1], w[3] - w[2], w[4] - w[3]);
                first_prices.entry(pattern).or_insert(w[4] as usize);
            }

            // Add the price for every pattern into the main earnings map.
            for (pattern, price) in first_prices {
                *earnings.entry(pattern).or_default() += price;
            }

            earnings
        })
        .into_values()
        .max()
        .ok_or("no result".into())
}

/// Finds the next secret number, based on the previous one. While it's probably not faster to
/// transform all of the operations (multiplication, division, modulo) into bit-based operations, I
/// thought it was funny that you could. So I did.
fn step(n: usize) -> usize {
    const PRUNE: usize = !(usize::MAX << 24);

    let n = ((n << 6) ^ n) & PRUNE;
    let n = ((n >> 5) ^ n) & PRUNE;
    ((n << 11) ^ n) & PRUNE
}

/// Parses the puzzle input into a series of secret numbers.
fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().filter_map(|n| n.parse().ok())
}
