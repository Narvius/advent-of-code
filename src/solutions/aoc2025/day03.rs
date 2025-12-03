/// Create the highest total output joltage with 2 batteries per bank.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(construct_batteries::<2>(input))
}

/// Create the highest total output joltage with 12 batteries per bank.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(construct_batteries::<12>(input))
}

/// Given a pick size of `N` batteries in each bank, constructs the largest possible joltage
/// arrangements for each bank, and sums them all.
fn construct_batteries<const N: usize>(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut batteries = [0; N];

            for (i, n) in line.bytes().map(|b| b - b'0').enumerate() {
                for index in 0..N {
                    if (i + N - 1 - index) < line.len() && n > batteries[index] {
                        batteries[index] = n;
                        batteries[index + 1..].fill(0);
                        break;
                    }
                }
            }

            batteries
                .iter()
                .fold(0usize, |acc, n| 10 * acc + *n as usize)
        })
        .sum()
}
