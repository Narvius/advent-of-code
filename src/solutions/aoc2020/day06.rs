/// Sum the counts of distinct letters in each chunk.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(line_chunks(input)
        .into_iter()
        .map(|chunk| {
            chunk
                .into_iter()
                .flat_map(|line| line.chars())
                .collect::<std::collections::HashSet<_>>()
                .len()
        })
        .sum())
}

/// Sum the number of letters appearing in every line in a chunk, for all chunks.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(line_chunks(input)
        .into_iter()
        .map(|chunk| {
            ('a'..='z')
                .filter(|&c| chunk.iter().all(|line| line.contains(c)))
                .count()
        })
        .sum())
}

/// Splits the input into chunks separated by empty lines.
fn line_chunks(s: &str) -> Vec<Vec<&str>> {
    let mut r = vec![];
    let mut v = vec![];
    for line in s.lines() {
        if line.is_empty() {
            r.push(v);
            v = vec![];
        } else {
            v.push(line);
        }
    }
    if !v.is_empty() {
        r.push(v);
    }
    r
}
