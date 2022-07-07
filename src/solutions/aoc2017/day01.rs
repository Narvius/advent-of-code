/// Find the sum of digits that match the next digit (wrapping around).
pub fn one(input: &str) -> crate::Result<String> {
    Ok(matchsum(input.as_bytes(), 1).to_string())
}

/// Find the sum of digits that match the digit halfway around the list (wrapping around).
pub fn two(input: &str) -> crate::Result<String> {
    Ok(matchsum(input.as_bytes(), input.len() / 2).to_string())
}

/// Shared logic for both parts. Finds the sum of digits that match the digit `offset`
/// characters further in the input, wrapping around if necessary.
fn matchsum(data: &[u8], offset: usize) -> i32 {
    let mut result = 0;
    for i in 0..data.len() {
        if data[i] == data[(i + offset) % data.len()] {
            result += (data[i] - b'0') as i32;
        }
    }
    result
}
