/// Find the winner of the White Elephant party using neighbour elimination.
pub fn one(input: &str) -> crate::Result<String> {
    // Taken from https://www.exploringbinary.com/powers-of-two-in-the-josephus-problem/
    let elves: i64 = input.parse().map_err(|_| "invalid input".to_owned())?;

    let exponent = (elves as f64).log2().floor() as i32;
    let closest_2pow = 2.0f64.powi(exponent) as i64;

    Ok((2 * (elves - closest_2pow) + 1).to_string())
}

/// Find the winner of the White Elephant party using opposite elimination.
pub fn two(input: &str) -> crate::Result<String> {
    // Derived by hand by looking at the pattern in the first 100 results.
    let elves: i64 = input.parse().map_err(|_| "invalid input".to_owned())?;

    let exponent = (elves as f64).log(3.0).ceil() as i32 - 1;
    let closest_3pow = 3.0f64.powi(exponent) as i64;

    let mut result = elves - closest_3pow;
    if result > closest_3pow {
        result += 2 * result - elves;
    }
    Ok(result.to_string())
}
