/// Find the position of the start of packet marker (after string of 4 distinct characters).
pub fn one(input: &str) -> crate::Result<usize> {
    start_marker(input, 4)
}

/// Find the position of the start of message marker (after string of 14 distinct characters).
pub fn two(input: &str) -> crate::Result<usize> {
    start_marker(input, 14)
}

/// Find the position just after reading a string of `len` distinct characters.
fn start_marker(input: &str, len: usize) -> crate::Result<usize> {
    let cs: Vec<_> = input.trim().chars().collect();
    let (i, _) = cs
        .windows(len)
        .enumerate()
        .find(|&(_, cs)| {
            let mut cs = cs.to_vec();
            cs.sort_unstable();
            cs.dedup();
            cs.len() == len
        })
        .ok_or("no start marker")?;

    Ok(i + len)
}
