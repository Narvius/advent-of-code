use std::collections::HashMap;

/// Find how many ways there are to make 150 using the given containers.
pub fn one(input: &str) -> crate::Result<String> {
    let containers: Vec<i32> = input.lines().filter_map(|s| s.parse().ok()).collect();

    Ok((0..2usize.pow(containers.len() as u32))
        .map(|mask| resolve_mask(&containers, mask).0)
        .filter(|&n| n == 150)
        .count()
        .to_string())
}

/// Find how many ways there are to make 150 litres with the (tied) least amount of given
/// containers.
pub fn two(input: &str) -> crate::Result<String> {
    let containers: Vec<i32> = input.lines().filter_map(|s| s.parse().ok()).collect();

    let mut buckets = HashMap::new();
    for mask in 0..2usize.pow(containers.len() as u32) {
        if let (150, count) = resolve_mask(&containers, mask) {
            *buckets.entry(count).or_insert(0) += 1;
        }
    }
    let least = buckets.keys().min().unwrap_or(&0);
    Ok(buckets.get(least).unwrap_or(&0).to_string())
}

/// Resolves a container combination to a total volume and count of containers. `mask` is a bit mask
/// where the `i`th bith says whether the `i`th container is included or not.
fn resolve_mask(containers: &[i32], mask: usize) -> (i32, i32) {
    let (mut sum, mut count) = (0, 0);
    for (i, &val) in containers.iter().enumerate() {
        if mask & (1 << i) > 0 {
            sum += val;
            count += 1;
        }
    }
    (sum, count)
}
