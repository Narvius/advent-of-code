/// Find the sum of distances between each pair of stars with space expanded.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(distances(input, 1))
}

/// Find the sum of distances between each pair of stars with space expanded a million times.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(distances(input, 999999))
}

/// Finds the sum of distances between each pair of stars in the input, taking into account space
/// expansion by a factor of `expansion`.
fn distances(input: &str, expansion: usize) -> usize {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();

    // Find empty columns and rows.
    let empty_columns: Vec<_> = (0..map[0].len())
        .filter(|&x| (0..map.len()).all(|y| map[y][x] == b'.'))
        .collect();

    let empty_rows: Vec<_> = (0..map.len())
        .filter(|&y| (0..map[0].len()).all(|x| map[y][x] == b'.'))
        .collect();

    // Create mappings such that (x_map[x], y_map[y]) is the same spot on the map but with space
    // expansion taken into account.
    let x_map = indices_to_map(map[0].len(), expansion, &empty_columns);
    let y_map = indices_to_map(map.len(), expansion, &empty_rows);

    // Find all stars in the input.
    let stars: Vec<_> = (0..map.len())
        .flat_map(|y| (0..map[y].len()).map(move |x| (x, y)))
        .filter(|&(x, y)| map[y][x] == b'#')
        .map(|(x, y)| (x_map[x], y_map[y]))
        .collect();

    // Calculate distances between all pairs. This double counts compared to what the puzzle asks
    // for, so we halve the final result.
    let full_distances: usize = stars
        .iter()
        .flat_map(|&(x1, y1)| {
            stars
                .iter()
                .map(move |&(x2, y2)| x1.abs_diff(x2) + y1.abs_diff(y2))
        })
        .sum();

    full_distances / 2
}

/// Given a list of spots that should expand (`indices`), returns a mapping such that if `n` is the
/// coordinate before expansion, `mapping[n]` is the coordinate after expansion. `count` is the
/// total number of coordinates (so width/height).
fn indices_to_map(count: usize, expansion: usize, indices: &[usize]) -> Vec<usize> {
    let mut map = vec![0; count];
    let mut taken = 0;
    for (i, m) in map.iter_mut().enumerate() {
        if let Some(&skip) = indices.get(taken) {
            if skip == i {
                taken += 1;
            }
        }

        *m = i + taken * expansion;
    }
    map
}
