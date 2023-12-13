/// Find the total reflection score.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(total_reflection_score(input, false))
}

/// Find the total smudged reflection score.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(total_reflection_score(input, true))
}

/// Finds the total reflection score of all maps in an input string.
fn total_reflection_score(input: &str, smudged: bool) -> usize {
    input
        .split("\r\n\r\n")
        .map(|chunk| {
            let map: Vec<_> = chunk.lines().map(|l| l.as_bytes()).collect();
            reflection_score(&map, smudged)
        })
        .sum()
}

/// Finds the reflection score of a map (number of columns to the left of a vertical reflection
/// line; or 100 times the number of rows above a horizontal reflection line). If `smudged` is set,
/// one tile needs to be flipped in the reflected part.
///
/// Note that we don't actually need to find which tile to flip in that case; we just need to find
/// a reflection that is exactly 1 tile off. So we can simply track whether there was a mistake,
/// anywhere.
fn reflection_score(map: &[&[u8]], smudged: bool) -> usize {
    // Find reflection around a vertical line.
    'outer: for x in 1..map[0].len() {
        let mut mistake = !smudged;
        let dx = usize::min(x, map[0].len() - x);
        for dx in 1..=dx {
            for row in map {
                if row[x - dx] != row[x + dx - 1] {
                    match mistake {
                        true => continue 'outer,
                        false => mistake = true,
                    }
                }
            }
        }
        if mistake {
            return x;
        }
    }

    // Find reflection around a horizontal line.
    'outer: for y in 1..map.len() {
        let mut mistake = !smudged;
        let dy = usize::min(y, map.len() - y);
        for dy in 1..=dy {
            for x in 0..map[0].len() {
                if map[y - dy][x] != map[y + dy - 1][x] {
                    match mistake {
                        true => continue 'outer,
                        false => mistake = true,
                    }
                }
            }
        }
        if mistake {
            return y * 100;
        }
    }

    0
}
