/// Count the number of possible passwords.
pub fn one(input: &str) -> crate::Result<usize> {
    count_valid_passwords(input, |b| {
        let is_ascending = b.windows(2).all(|w| w[0] <= w[1]);
        let has_pair = b.windows(2).any(|w| w[0] == w[1]);
        is_ascending && has_pair
    })
}

/// Count the number of possible passwords using a more strict rule.
pub fn two(input: &str) -> crate::Result<usize> {
    count_valid_passwords(input, |b| {
        let is_ascending = b.windows(2).all(|w| w[0] <= w[1]);
        let has_pair = {
            let mut counts = [0; 10];
            for &b in b {
                counts[(b - b'0') as usize] += 1;
            }
            counts.contains(&2)
        };

        is_ascending && has_pair
    })
}

/// Counts how many passwords in a range given in the input are `valid`.
fn count_valid_passwords(input: &str, valid: fn(&[u8]) -> bool) -> crate::Result<usize> {
    let (a, b) = input.split_once('-').ok_or("invalid input")?;
    let (a, b): (i32, i32) = (a.parse()?, b.parse()?);

    Ok((a..=b).filter(|n| valid(n.to_string().as_bytes())).count())
}
