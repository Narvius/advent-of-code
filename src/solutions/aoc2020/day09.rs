use std::cmp::Ordering;

/// Find the invalid number in the input (one that doesn't have a pair of numbers summing to it
/// in the previous 25 entries).
pub fn one(input: &str) -> crate::Result<i32> {
    run(input, false)
}

/// Find the encryption weakness of the input; a number derived from the result of `one`.
/// See [`run`] for details.
pub fn two(input: &str) -> crate::Result<i32> {
    run(input, true)
}

/// Combined code for [`one`] and [`two`], ran if `find_encryption_weakness` is false or true,
/// respectively.
///
/// The encryption weakness is the sum of the smallest and largest number in a run of
/// consecutive numbers that sum up to the invalid number (the result from [`one`]).
fn run(input: &str, find_encyption_weakness: bool) -> crate::Result<i32> {
    let data: Vec<_> = input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();

    let invalid_number = data
        .windows(26)
        .find(|w| !any_two_sum_to(&w[0..25], w[25]))
        .map(|w| w[25])
        .ok_or("no result")?;

    if !find_encyption_weakness {
        return Ok(invalid_number);
    }

    let mut total;
    for start in 0..data.len() {
        total = data[start];
        for n in 1..=(data.len() - start) {
            total += data[start + n];
            match total.cmp(&invalid_number) {
                Ordering::Less => continue,
                Ordering::Equal => {
                    let range = &data[start..=(start + n)];
                    return Ok(range.iter().min().ok_or("empty range")?
                        + range.iter().max().ok_or("empty range")?);
                }
                Ordering::Greater => break,
            }
        }
    }

    Err("no result".into())
}

/// Checks if any two numbers in `data` sum to `sum`.
fn any_two_sum_to(data: &[i32], sum: i32) -> bool {
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            if data[i] + data[j] == sum {
                return true;
            }
        }
    }
    false
}
