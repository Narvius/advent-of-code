use std::collections::HashMap;

/// Find the sum of sector IDs of all real rooms.
pub fn one(input: &str) -> Result<String, String> {
    Ok(parse(input)
        .filter_map(|(name, checksum, id)| {
            let top_chars = {
                let mut chars = HashMap::new();
                for c in name.chars().filter(|c| c.is_alphabetic()) {
                    *chars.entry(c).or_insert(0) += 1;
                }
                let mut v: Vec<_> = chars.into_iter().collect();
                v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
                v.into_iter().map(|(c, _)| c).take(5)
            };

            top_chars.eq(checksum.chars()).then(|| id)
        })
        .sum::<i32>()
        .to_string())
}

/// Find the sector ID of the north pole object storage.
pub fn two(input: &str) -> Result<String, String> {
    Ok(parse(input)
        .filter_map(|(name, _, id)| {
            let shift = (id % 26) as u8;
            let shifted = String::from_utf8(
                name.bytes()
                    .filter(|c| c.is_ascii_alphabetic())
                    .map(|c| (c - b'a' + shift) % 26 + b'a')
                    .collect(),
            )
            .ok()?;

            (shifted.contains("north") && shifted.contains("pole")).then(|| id)
        })
        .next()
        .ok_or_else(|| "failed to find room".to_owned())?
        .to_string())
}

/// Parses each line of puzzle input into a triplet of the relevant information: the sector name,
/// the checksum, and the parsed sector ID.
fn parse(input: &str) -> impl Iterator<Item = (&str, &str, i32)> {
    input.lines().filter_map(|line| {
        let (data, checksum) = line.split_once('[')?;
        let id_pos = data.find(|c: char| c.is_numeric())?;

        Some((
            &data[0..id_pos - 1],
            &checksum[0..5],
            data[id_pos..].parse().ok()?,
        ))
    })
}
