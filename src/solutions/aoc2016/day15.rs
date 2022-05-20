/// Find the first time at which the ball can drop through all discs.
pub fn one(input: &str) -> Result<String, String> {
    Ok(parse(input)
        .reduce(combine_modulo_constraints)
        .ok_or_else(|| "no input".to_owned())?
        .1 // The "m" from x % n = m. By definition the smallest possible positive x.
        .to_string())
}

/// Find the first time at which the ball can drop through all discs, including an extra one.
pub fn two(input: &str) -> Result<String, String> {
    Ok(parse(&format!(
        "{}Disc #7 has 11 positions; at time=0, it is at position 0.\n",
        input
    ))
    .reduce(combine_modulo_constraints)
    .ok_or_else(|| "no input".to_owned())?
    .1 // The "m" from x % n = m. By definition the smallest possible positive x.
    .to_string())
}

/// Uses the Chinese Remainder theorem to combine two modulo constraints into a single equivalent
/// modulo constraint.
///
/// A modulo constraint is a pair of two numbers `(n, m)` in the formula `x % n = m`. As an
/// example, the modulo constraints `(2, 0)` and `(3, 0)` say that there is some `x` that fulfills
/// both `x % 2 = 0` and `x % 3 = 0`. This is equivalent to the single modulo constraint `(6, 0)`,
/// that is, `x % 6 = 0`.
fn combine_modulo_constraints((n1, m1): (i64, i64), (n2, m2): (i64, i64)) -> (i64, i64) {
    let (_, bx, by) = extended_euclidian(n1, n2);
    (n1 * n2, (n1 * bx * m2 + n2 * by * m1).rem_euclid(n1 * n2))
}

/// Calculates the greatest common divisor using the Euclidean algorithm; as well as finds a
/// pair of Bézout coefficients for them. These coefficiens are used in the Chinese Remainder
/// Theorem to combine two module constraints into one. Adapted from the pseudocode at
/// <https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm>.
fn extended_euclidian(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_r, old_s, old_t)
}

/// Parses the puzzle input into a series of modulo constraints. Note that this encapsulates a bit
/// of important logic for the puzzle, accounting for the time difference between each disc.
fn parse(input: &str) -> impl Iterator<Item = (i64, i64)> + '_ {
    input.lines().enumerate().filter_map(|(i, line)| {
        let tokens: Vec<_> = line[0..line.len() - 1].split(' ').collect();
        let n = tokens[3].parse::<i64>().ok()?;
        let initial = tokens[11].parse::<i64>().ok()?;
        // `m` has to account for the delay between reaching each consecutive disc.
        // Disc #1 with 13 positions starting at 1 corresponds to a modulo constraint of
        // (13, 11), because it has 13 positions (the divisor) and after 1 second it will need to
        // rotate 11 more times to be in position zero. Disc #2 with 19 positions and starting at
        // 10 will result in (19, 7), because after two seconds it will need to move 7 more times
        // to be in position zero, and so on.
        Some((n, (n - initial - i as i64 - 1).rem_euclid(n)))
    })
}
