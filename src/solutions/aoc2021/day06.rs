/// Count the number of anglerfish after 80 days.
pub fn one(input: &str) -> crate::Result<u64> {
    Ok(count_lanternfish_after_days(input, 80))
}

/// Count the number of anglerfish after 256 days.
pub fn two(input: &str) -> crate::Result<u64> {
    Ok(count_lanternfish_after_days(input, 256))
}

/// Counts the number of lanternfish that would exist after the given amount of days, using a line
/// from the puzzle input as an initial population.
/// There is no need to keep a full list of all lanternfish--we can put them into nine buckets
/// depending on how much time there is until they spawn a new fish. That way we can move around
/// entire generations at once, instead of having to do each fish individually.
fn count_lanternfish_after_days(input: &str, days: usize) -> u64 {
    let mut fish = [0; 9];
    for c in input.split(',') {
        fish[c.parse::<usize>().unwrap()] += 1;
    }

    for _ in 0..days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }

    fish.iter().sum()
}
