/// Find the best horizontal alignment if fuel costs are linear.
pub fn one(input: &str) -> crate::Result<i32> {
    let mut crabs: Vec<i32> = input.split(',').filter_map(|s| s.parse().ok()).collect();
    // The cheapest vertical alignment is at the median. Consider what happens if you move the
    // target right: All elements to the left of it have to move 1 more, and all elements on the
    // right, 1 less. That means, unless there's the same number of elements on either side,
    // you can reduce the cost by moving towards the median.
    crabs.sort();
    let target = crabs[crabs.len() / 2];
    Ok(crabs.iter().map(|&i| (target - i).abs()).sum())
}

/// Find the best horizontal alignment if fuel costs are triangular.
pub fn two(input: &str) -> crate::Result<i32> {
    let crabs: Vec<i32> = input.split(',').filter_map(|s| s.parse().ok()).collect();
    // The cheapest vertical alignment is the average (arithmetic mean). Since the cost grows
    // O(n^2) with distance, we want to limit the highest individual distance, but also stay
    // close to the bulk of points. No rigorous explanation here, unfortunately; but trying
    // out a few examples on paper shows it works.
    let target = crabs.iter().copied().sum::<i32>() / crabs.len() as i32;
    Ok(crabs
        .iter()
        .map(|&i| {
            let dist = (target - i).abs();
            dist * (dist + 1) / 2
        })
        .sum())
}
