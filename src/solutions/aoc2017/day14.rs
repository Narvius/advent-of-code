use std::collections::HashSet;

/// Count the number of squares that are used, which corresponds to the number of 1 bits across
/// 128 consecutive knot hashes.
pub fn one(input: &str) -> Result<String, String> {
    let mut sum = 0;
    for i in 0..128 {
        sum += super::day10::knot_hash(format!("{}-{}", input, i).bytes())
            .into_iter()
            .map(|b| b.count_ones())
            .sum::<u32>();
    }
    Ok(sum.to_string())
}

/// Count the number of groups in the map derived from knot hashes.
pub fn two(input: &str) -> Result<String, String> {
    let mut used = HashSet::new();
    let mut groups = 0;

    for y in 0..128 {
        let hash = super::day10::knot_hash(format!("{}-{}", input, y).bytes());
        for x in 0..128 {
            if (hash[x as usize / 8] & (1 << (7 - x % 8))) > 0 {
                used.insert((x, y));
            }
        }
    }

    while let Some(p) = used.iter().next().copied().and_then(|p| used.take(&p)) {
        groups += 1;
        let mut stack = vec![p];
        while let Some((x, y)) = stack.pop() {
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                if used.remove(&(x + dx, y + dy)) {
                    stack.push((x + dx, y + dy));
                }
            }
        }
    }

    Ok(groups.to_string())
}
