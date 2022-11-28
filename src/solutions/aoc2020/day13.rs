/// Find the bus that leaves soonest after the timestamp, and calculate a checksum from it.
pub fn one(input: &str) -> crate::Result<i64> {
    let (timestamp, buses) = parse(input).ok_or("parse failed")?;
    let (bus, time) = buses
        .into_iter()
        .map(|(bus, _)| (bus, bus - timestamp % bus))
        .min_by_key(|&(_, time)| time)
        .ok_or("no result")?;
    Ok(bus * time)
}

/// Find the first point in time starting at which all buses leave one after another on
/// each consecutive minute.
pub fn two(input: &str) -> crate::Result<i64> {
    let (_, buses) = parse(input).ok_or("parse failed")?;

    // This is basically a big modulo constraint puzzle. We have to count up until, for each
    // bus, [(time + position in list) % busnumber] is zero. Fortunately, this can be made faster;
    // by combining all of these constraints that are true modulo constraints into
    // one, we can create a list of candidate numbers that is much smaller than the entire
    // number line.

    // All constraints that aren't true modulo constraints.
    let constraints: Vec<_> = buses.iter().copied().filter(|bus| bus.1 >= bus.0).collect();
    // One true modulo constraint calculated from all the true modulo constraints in the
    // input, using the Chinese Remainder Theorem.
    let true_constraint = buses
        .iter()
        .copied()
        .filter(|bus| bus.1 < bus.0)
        .reduce(combine_constraints);

    if let Some((n, offset)) = true_constraint {
        for result in (-offset..).step_by(n as usize) {
            if constraints
                .iter()
                .all(|(n, offset)| (result + offset) % n == 0)
            {
                return Ok(result);
            }
        }
    }

    Err("unreachable".into())
}

/// A modulo constraint, consisting of a divisor and an offset. Basically, this constraint
/// filters out numbers `x` that don't match `(x + offset) % divisor == 0`.
///
/// If the offset is less than the divisor, this is a "true modulo constraint"--it can be
/// rearranged into `x % divisor == divisor - offset`. Two constraints like that can be combined
/// into one. For details, see [`combine_constraints`].
type Constraint = (i64, i64);

/// Parses the input into an initial timestamp, and a list of constraints.
fn parse(input: &str) -> Option<(i64, Vec<Constraint>)> {
    let mut lines = input.lines();
    let timestamp = lines.next()?.parse().ok()?;
    let constraints = lines
        .next()?
        .split(',')
        .enumerate()
        .filter_map(|(i, c)| c.parse().ok().map(|c| (c, i as i64)))
        .collect();

    Some((timestamp, constraints))
}

/// Combines two modulo constraints using the Chinese Remainder Theorem.
/// See: https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Using_the_existence_construction
fn combine_constraints((an, ao): Constraint, (bn, bo): Constraint) -> (i64, i64) {
    let (_, x, y) = extended_euclidean(an, bn);
    let q = an * bn;
    (q, i64::rem_euclid(an * x * bo + bn * y * ao, q))
}

/// Extended Euclidean algorithm that returns two Bezout coefficients alongside the
/// greatest common divisor of two numbers. The Bezout coefficients are used in the
/// Chinese Remainder Theorem to combine two true modulo constraints into one.
///
/// Straightforward translation of the pseudocode from
/// Wikipedia (https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm)
fn extended_euclidean(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quot = old_r / r;
        (old_r, r) = (r, old_r - quot * r);
        (old_s, s) = (s, old_s - quot * s);
        (old_t, t) = (t, old_t - quot * t);
    }

    (old_r, old_s, old_t)
}
