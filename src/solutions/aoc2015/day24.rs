pub fn one(input: &str) -> Result<String, String> {
    let data: Vec<_> = input.lines().filter_map(|s| s.parse().ok()).collect();
    Ok(find_quantum_entanglement(&data, 3)?.to_string())
}

pub fn two(input: &str) -> Result<String, String> {
    let data: Vec<_> = input.lines().filter_map(|s| s.parse().ok()).collect();
    Ok(find_quantum_entanglement(&data, 4)?.to_string())
}

/// Calculates the smallest possible quantum entanglement value (see puzzle description) for the
/// given set of `weights` if they were split up into `groups` groups of equal total weights.
fn find_quantum_entanglement(weights: &[usize], groups: usize) -> Result<usize, String> {
    let group_weight = weights.iter().copied().sum::<usize>() / groups;

    // smallest theoretically possible size is the number of largest elements we need to take
    // before the sum is larger than the target. So if we find any group of that size, that means
    // we only need to consider those.
    let best_size = {
        let (mut remaining, mut i) = (group_weight as i32, weights.len() - 1);
        while remaining > 0 {
            remaining -= weights[i] as i32;
            i -= 1;
        }
        weights.len() - i
    };

    groups_of_size(group_weight, best_size, weights)
        .map(|g| g.iter().copied().reduce(|a, b| a * b).unwrap())
        .min()
        .ok_or_else(|| "no quantum configuration found".into())
}

/// Returns all possible combinations from `weights` of the given `size` that sum up to `weight`.
fn groups_of_size(
    weight: usize,
    size: usize,
    weights: &[usize],
) -> impl Iterator<Item = Vec<usize>> + '_ {
    (0..2u32.pow(weights.len() as u32))
        .filter(move |mask| mask.count_ones() == size as u32)
        .filter_map(move |mask| {
            let mut result = Vec::with_capacity(3);
            for (i, &weight) in weights.iter().enumerate() {
                if mask & (1 << i) > 0 {
                    result.push(weight);
                }
            }
            (result.iter().copied().sum::<usize>() == weight).then(|| result)
        })
}
