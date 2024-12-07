/// Find the sum of the values of all expressions that can be formed using only addition and
/// multiplication.
pub fn one(input: &str) -> crate::Result<i64> {
    sum_formable_expressions(input, false)
}

/// Find the sum of the values of all expressions that can be formed using only addition,
/// multiplication, and concatenation.
pub fn two(input: &str) -> crate::Result<i64> {
    sum_formable_expressions(input, true)
}

/// Sums all formable expressions in the input. Uses only addition and multiplication when
/// `with_concat` is false; also uses `concatenate` when it is `true`.
fn sum_formable_expressions(input: &str, with_concat: bool) -> crate::Result<i64> {
    Ok(parse(input)
        .filter(|(total, nums)| formable(&nums[1..], nums[0], *total, with_concat))
        .map(|p| p.0)
        .sum())
}

/// Checks if the `target` is reachable using the remaining `nums`, the partially-processed result
/// in `acc`, and admissible operators (+, *; and `concatenate` if `concat` is true).
///
/// Ordinarily, I would split this into two versions (one with concat, the other without) to avoid
/// the additional branching, but tests indicated that the Rust compiler is clever enough to
/// sufficiently optimize that part, and I saw no performance difference.
fn formable(nums: &[i64], acc: i64, target: i64, concat: bool) -> bool {
    if nums.is_empty() || acc > target {
        return acc == target;
    }

    let (num, nums) = (nums[0], &nums[1..]);
    (concat && formable(nums, concatenate(acc, num), target, concat))
        || formable(nums, acc * num, target, concat)
        || formable(nums, acc + num, target, concat)
}

/// Concatenates two numbers.
fn concatenate(left: i64, right: i64) -> i64 {
    left * 10i64.pow((f64::log10(right as f64) + 1.0) as u32) + right
}

/// Parses the puzzle input into (expected result, numbers in expression) pairs.
fn parse(input: &str) -> impl Iterator<Item = (i64, Vec<i64>)> + '_ {
    input.lines().filter_map(|line| {
        let (total, numbers) = line.split_once(": ")?;
        Some((
            total.parse().ok()?,
            numbers
                .split_ascii_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect(),
        ))
    })
}
