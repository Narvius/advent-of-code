use crate::common::product;

/// Find the number of key/lock combinations that could work.
pub fn one(input: &str) -> crate::Result<usize> {
    let (keys, locks) = parse(input);

    Ok(product(keys, locks)
        .filter(|((_, key), (_, lock))| {
            key.iter()
                .zip(lock.iter())
                .map(|(a, b)| a + b)
                .all(|n| n <= 5)
        })
        .count())
}

/// Freebie!
pub fn two(_input: &str) -> crate::Result<&str> {
    Ok("done!")
}

type Item = (bool, [i32; 5]);

/// Parses the puzzle input into lists of keys and locks.
fn parse(input: &str) -> (Vec<Item>, Vec<Item>) {
    input
        .split("\n\n")
        .map(to_column_counts)
        .partition(|&(is_key, _)| is_key)
}

/// Given a 5x6 rectangle from the input, converts it into
fn to_column_counts(item: &str) -> Item {
    let is_key = item.starts_with('#');
    let mut result = [-1; 5];
    for line in item.lines() {
        for (i, c) in line.char_indices() {
            if c == '#' {
                result[i] += 1
            }
        }
    }
    (is_key, result)
}
