/// Find the total amount of fuel required for the given input weights.
pub fn one(input: &str) -> crate::Result<i32> {
    run(input, |n| n / 3 - 2)
}

/// Find the total fuel weight if fuel weight also requires additional fuel.
pub fn two(input: &str) -> crate::Result<i32> {
    run(input, |n| {
        std::iter::successors(Some(n), |&n| (n > 6).then_some(n / 3 - 2))
            .skip(1)
            .sum()
    })
}

/// Applies the `weight` function to each line of the input, and finds the sum.
fn run(input: &str, weight: fn(i32) -> i32) -> crate::Result<i32> {
    Ok(input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok().map(weight))
        .sum())
}
