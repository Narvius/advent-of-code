/// Find the total caolories carried by the top elf.
pub fn one(input: &str) -> crate::Result<i32> {
    Elves(input.lines()).max().ok_or_else(|| "no result".into())
}

/// Find the total calories carried by the top three elves.
pub fn two(input: &str) -> crate::Result<i32> {
    let mut elves: Vec<_> = Elves(input.lines()).collect();
    elves.sort_unstable_by_key(|e| i32::MAX - e);
    Ok(elves[0..3].iter().copied().sum())
}

/// An iterator over all elves in the input.
struct Elves<'a>(std::str::Lines<'a>);

impl Iterator for Elves<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = 0;
        for line in self.0.by_ref() {
            if line.is_empty() {
                return Some(sum);
            } else {
                sum += line.parse::<i32>().ok()?;
            }
        }

        None
    }
}
