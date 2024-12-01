use std::collections::HashMap;

/// Read the two vertically-printed lists in the input, sort them in ascending order, find the
/// absolute difference between each corresponding pair of items, and sum all those differences.
pub fn one(input: &str) -> crate::Result<u32> {
    let (mut left, mut right) = (vec![], vec![]);
    for (l, r) in input.lines().filter_map(|line| line.split_once("   ")) {
        left.push(l.parse::<i32>()?);
        right.push(r.parse::<i32>()?);
    }

    left.sort_unstable();
    right.sort_unstable();

    Ok(left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum())
}

/// For each entry in the left list, multiply it by the number of times it appears in the right
/// list; sum all those numbers.
pub fn two(input: &str) -> crate::Result<i32> {
    let (mut left, mut right) = (vec![], HashMap::new());
    for (l, r) in input.lines().filter_map(|line| line.split_once("   ")) {
        left.push(l.parse::<i32>()?);
        *right.entry(r.parse::<i32>()?).or_default() += 1;
    }

    Ok(left
        .into_iter()
        .map(|k| k * right.get(&k).unwrap_or(&0))
        .sum())
}
