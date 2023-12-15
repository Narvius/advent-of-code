/// Find the [HASH](hash) of each input instruction, and sum them all.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(input.trim().split(',').map(hash).sum())
}

/// Find the lens configuration described the the input and calculate a checksum.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    let instructions = input.trim().split(',');
    for (label, target) in instructions.filter_map(|s| s.split_once(&['-', '='][..])) {
        let slot = &mut boxes[hash(label)];
        if target.is_empty() {
            slot.retain(|e| e.0 != label);
        } else if let Some(lens) = slot.iter_mut().find(|e| e.0 == label) {
            *lens = (label, target.parse()?);
        } else {
            slot.push((label, target.parse()?));
        }
    }

    Ok(boxes
        .into_iter()
        .enumerate()
        .flat_map(|(box_nr, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(move |(lens_nr, (_, power))| (box_nr + 1) * (lens_nr + 1) * power)
        })
        .sum())
}

type Lens<'a> = (&'a str, usize);

/// Calculates the Holiday ASCII String Helper algorithm value for a string.
fn hash(s: &str) -> usize {
    s.bytes().fold(0, |acc, n| (acc + n as usize) * 17 % 256)
}
