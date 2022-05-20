use std::collections::HashSet;

/// Gets the next valid password.
pub fn one(input: &str) -> Result<String, String> {
    let mut bytes: Vec<_> = input.bytes().collect();
    while !valid_password(&bytes) {
        increment(&mut bytes);
    }
    String::from_utf8(bytes).map_err(|_| "failed to convert [u8] to String".into())
}

/// Gets the NEXT next valid password.
pub fn two(input: &str) -> Result<String, String> {
    let mut bytes: Vec<_> = input.bytes().collect();
    while !valid_password(&bytes) {
        increment(&mut bytes);
    }
    increment(&mut bytes);
    while !valid_password(&bytes) {
        increment(&mut bytes);
    }
    String::from_utf8(bytes).map_err(|_| "failed to convert [u8] to String".into())
}

/// Checks whether a password is valid.
fn valid_password(s: &[u8]) -> bool {
    let mut pairs = HashSet::new();
    pairs.extend(s.windows(2).filter(|w| w[0] == w[1]).map(|w| w[0]));

    let has_ascending = s.windows(3).any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);
    let no_bads = s.iter().all(|c| !b"iol".contains(c));
    let has_pairs = pairs.len() >= 2;

    has_ascending && no_bads && has_pairs
}

/// Increments a password by one.
fn increment(s: &mut [u8]) {
    for i in (0..s.len()).rev() {
        if s[i] == b'z' {
            s[i] = b'a';
        } else {
            s[i] += 1;
            break;
        }
    }
}
