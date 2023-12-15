/// Find the [HASH](hash) of each input instruction, and sum them all.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(input.split(',').map(hash).sum())
}

/// Find the lens configuration described the the input and calculate a checksum.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    let instructions = input
        .split(',')
        .filter_map(|s| s.trim().split_once(&['-', '='][..]));

    for (label, target) in instructions {
        let index = hash(label);
        if target.is_empty() {
            boxes[index].retain(|e| e.0 != label);
        } else if let Some(lens) = boxes[index].iter_mut().find(|e| e.0 == label) {
            *lens = (label, target.parse()?);
        } else {
            boxes[index].push((label, target.parse()?));
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
    s.trim()
        .bytes()
        .fold(0, |acc, n| (acc + n as usize) * 17 % 256)
}
