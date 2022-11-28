/// Find the joltage chain checksum (1-jolt difference count times 3-jolt difference count).
pub fn one(input: &str) -> crate::Result<usize> {
    let data = parse(input);
    Ok(data.windows(2).filter(|w| w[1] - w[0] == 1).count()
        * data.windows(2).filter(|w| w[1] - w[0] == 3).count())
}

/// Find the number of possible joltage chains that can be constructed.
///
/// We don't actually need to construct each of the possible chains, just count how many
/// there are. We can solve this by solving shorter sub-chains, and summing up those
/// sub-solutions in a clever fashion.
///
/// Basically, the solution for a chain with `k` added is the sum of all smaller solutions
/// that are within 3 joltage points of `k`.
///
/// As an example, if you have the chain 1, 2, 3, 4, 6:
///
/// ```text
/// Subchain [1] has 1 solution (1).
/// Subchain [1, 2] has 1 solution (12).
/// Subchain [1, 2, 3] has 2 solutions (13, 123).
/// Subchain [1, 2, 3, 4] has 4 solutions (14, 124, 134, 1234).
/// Subchain [1, 2, 3, 4, 6] has 6 solutions (136, 1236, 146, 1246, 1346, 12346).
///
/// Let SC(n) = # of solutions for Subchain with n first elements.
///
/// SC(1) =                     1 = 1
/// SC(2) =                 SC(1) = 1
/// SC(3) =         SC(1) + SC(2) = 2
/// SC(4) = SC(1) + SC(2) + SC(3) = 4
/// SC(5) =         SC(3) + SC(4) = 6
/// ```
///
/// Which elements are included in that sum above is decided by the puzzle constraints; `SC(5)`
/// includes only `SC(3)` and `SC(4)` but not `SC(2)`, because the third element (joltage 3) and
/// fourth element (joltage 4) are within 3 units of the fifth element (joltage 6); whereas the
/// second element (joltage 2) is 4 units away.
///
/// `SC(# of joltages)` then is the number of arrangements for the entire chain.
pub fn two(input: &str) -> crate::Result<i64> {
    fn sub_solution_count(data: &[i32], cache: &[i64], i: i32, j: i32) -> Option<i64> {
        let j = usize::try_from(i - j).ok()?;
        (data.get(i as usize)? - data.get(j)? <= 3).then(|| cache[j])
    }

    let data = parse(input);
    let mut cache = vec![0; data.len()];
    cache[0] = 1;

    for i in 1..data.len() as i32 {
        for j in 1..=3 {
            cache[i as usize] += sub_solution_count(&data, &cache, i, j).unwrap_or(0);
        }
    }

    cache.last().copied().ok_or_else(|| "no result".into())
}

/// Parses the puzzle list into a sorted list of joltages, including a 0 and `max + 3`
/// described in the puzzle input.
fn parse(input: &str) -> Vec<i32> {
    let mut data: Vec<i32> = input.lines().filter_map(|line| line.parse().ok()).collect();
    data.push(0);
    data.push(3 + data.iter().max().unwrap());
    data.sort();
    data
}
