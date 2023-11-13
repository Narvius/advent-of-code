use std::collections::{HashMap, HashSet};

/// Find the largest number accepted by MONAD.
pub fn one(input: &str) -> crate::Result<String> {
    produce_monad_number(input, true)
}

/// Find the lowest number accepted by MONAD.
pub fn two(input: &str) -> crate::Result<String> {
    produce_monad_number(input, false)
}

/// Produces either the `largest` or smallest number accepted by MONAD from the puzzle input.
fn produce_monad_number(input: &str, largest: bool) -> crate::Result<String> {
    let digits: Vec<_> = match largest {
        true => (1..=9).collect(),
        false => (1..=9).rev().collect(),
    };

    let (checks, adds) = parse(input)?;
    let mut target_zs = HashSet::from([0]);
    let mut result: HashMap<i32, Vec<i32>> = HashMap::new();

    // Go backwards, starting with the least significant (last to be input) digit. Try out all
    // combinations of input digits and `target_z`s, finding new targets for the next-most
    // significant digit.
    //
    // `result` contains a mapping from a `z` produced to the series of digits that produces it.
    for index in (0..14).rev() {
        let mut new_zs = HashSet::new();
        for &w in &digits {
            for &target_z in &target_zs {
                if let Some(z) = backwards(w, target_z, checks[index], adds[index]) {
                    new_zs.insert(z);

                    let mut number = result.entry(target_z).or_default().clone();
                    number.push(w);
                    result.insert(z, number);
                }
            }
        }
        target_zs = new_zs
    }

    result.entry(0).or_default().reverse();
    Ok(result[&0]
        .iter()
        .map(|&n| (b'0' + n as u8) as char)
        .collect())
}

/// Does the MONAD calculations, backwards; producing possible `z` register values that would lead
/// to the `target_z`.
///
/// The puzzle input is 14 mostly-identical blocks of the same 18 lines of "code". Each block does
/// a calculation dependent on five total values:
/// - `w`, the digit we pass in
/// - `z`, result from the previous block
/// - `check`, a constant from the 5th line after a read
/// - `add`, a constant from the 15th line after a read
///
/// The same calculation can then be translated to Rust:
///
/// ```
/// fn monad_step(w: i32, z: i32, check: i32, add: i32) -> i32 {
///     if z % 26 != w - check {
///         26 * z + w + add
///     } else {
///         z / 26
///     }
/// }
/// ```
///
/// (Note: Technically, on both branches `z` is divided by a fifth parameter, `div`. It is parsed
/// from the 4th line after a read. However, it can be statically inlined. It is 1 if `check > 9`,
/// and 26 otherwise. `check` being 9 or less means we can conceivably reach the else branch, and
/// the way the number shake out, every time we *could* reach the else branch, we *have* to. The if
/// branch only increases `z`, but we eventually want to arrive at 0. Therefore, I've inlined
/// `div`--in the if branch, it becomes `z / 1`, ie. simply `z`, in the else branch, it corresponds
/// to the 26 factor we divide by.)
///
/// [`backwards`] is the result of inverting this calculation to produce values that, if plugged
/// into `monad_step`, will produce `target_z`. There may be up to two such values, one produced
/// from the if branch, and one produced from the else branch.
fn backwards(w: i32, target_z: i32, check: i32, add: i32) -> Option<i32> {
    if check > 9 {
        // If `target_z` got produced, it would be from the if branch.
        let partial = target_z - w - add;
        // `partial` here is equal to `26 * z`. So in order to have arrived here, it must be divisible
        // by 26. If it is, we divide it out the 26 factor to arrive at the original `z`.
        (partial % 26 == 0).then_some(partial / 26)
    } else {
        // It `target_z` got produced, it would be from the else branch.
        let partial = w - check;
        // `partial` here is equal to `z % 26`. For that to be possible, it must be in `(0..26)`. The
        // `z` we produce must fulfill two conditions: `z % 26 = partial`, and `z / 26 = target_z`.
        ((0..26).contains(&partial)).then_some(partial + target_z * 26)
    }
}

/// Extracts the constants required to power [`backwards`] from the input. See there for more
/// details.
fn parse(input: &str) -> crate::Result<(Vec<i32>, Vec<i32>)> {
    let lines: Vec<_> = input.lines().collect();
    let (mut checks, mut adds) = (vec![], vec![]);

    for block in lines.chunks(18) {
        checks.push(block[5][6..].parse()?);
        adds.push(block[15][6..].parse()?);
    }

    Ok((checks, adds))
}
