/// Find the number of safe spots in a 40 row room.
pub fn one(input: &str) -> Result<String, String> {
    Ok(count_safe_tiles(input, 40).to_string())
}

/// Find the number of safe spots in a 400000 row room.
pub fn two(input: &str) -> Result<String, String> {
    Ok(count_safe_tiles(input, 400000).to_string())
}

/// Counts the number of safe tiles in `rows` rows, starting with the given row.
///
/// Note that the four conditions for a trap are logically equivalent to `left xor right`--and,
/// by extension, this means that we can construct the next state by `xor`ing the previous state
/// shifted to the left and right by one, respectively.
fn count_safe_tiles(input: &str, rows: usize) -> i32 {
    let mut key = {
        let mut line = 0;
        for (i, b) in input.bytes().enumerate() {
            if b == b'^' {
                line |= 1u128 << i;
            }
        }
        line
    };

    let len = input.len() as i32;
    let mask = (1u128 << len) - 1;
    let mut count = len - key.count_ones() as i32;

    for _ in 1..rows {
        let next = mask & (key << 1 ^ key >> 1);
        count += len - next.count_ones() as i32;
        key = next;
    }

    count
}
