use std::collections::HashMap;

/// Find the sum of the 2000th generated secret number of each buyer.
pub fn one(input: &str) -> crate::Result<usize> {
    let mut buyers: Vec<_> = parse(input).collect();
    for _ in 0..2000 {
        for n in &mut buyers {
            *n = step(*n);
        }
    }
    Ok(buyers.into_iter().sum())
}

/// Find the best amount of bananas obtainable from selling through they negotiating monkey.
///
/// For each buyer, finds all possible sales; and then simply sums up the price for each
/// negotiation pattern across every buyer, and picks the highest one.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut bananas: HashMap<(i32, i32, i32, i32), usize> = HashMap::new();
    for mut n in parse(input) {
        #[allow(unused_assignments)]
        let (mut m4, mut m3, mut m2, mut m1) = (None, None, None, None);
        let mut patterns = HashMap::new();
        for _ in 0..2000 {
            (m4, m3, m2, m1, n) = (m3, m2, m1, Some(n % 10), step(n));

            if let (Some(m4), Some(m3), Some(m2), Some(m1)) = (m4, m3, m2, m1) {
                let key = (
                    m3 as i32 - m4 as i32,
                    m2 as i32 - m3 as i32,
                    m1 as i32 - m2 as i32,
                    n as i32 % 10 - m1 as i32,
                );

                patterns.entry(key).or_insert(n % 10);
            }
        }
        for (key, count) in patterns {
            *bananas.entry(key).or_default() += count;
        }
    }
    bananas.into_values().max().ok_or("no result".into())
}

/// Finds the next secret number, based on the previous one. While it's probably not faster to
/// transform all of the operations (multiplication, division, modulo) into bit-based operations, I
/// thought it was funny that you could. So I did.
fn step(n: usize) -> usize {
    const PRUNE: usize = usize::MAX ^ (usize::MAX << 24);

    let n = ((n << 6) ^ n) & PRUNE;
    let n = ((n >> 5) ^ n) & PRUNE;
    ((n << 11) ^ n) & PRUNE
}

/// Parses the puzzle input into a series of secret numbers.
fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().filter_map(|n| n.parse().ok())
}
