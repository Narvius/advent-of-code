/// Count how many times the rotating dial ends at 0 after finishing a rotation.
pub fn one(input: &str) -> crate::Result<usize> {
    count_zeroes(input, false)
}

/// Count how many times the rotating dial clicks on 0 throughout all the rotations.
pub fn two(input: &str) -> crate::Result<usize> {
    count_zeroes(input, true)
}

/// Counts how many times the dial reaches position 0. If `count_passes` is false, only counts
/// zeroes reached at the end of individual movements; if it is true, counts any zero passed at
/// any point in any movement.
fn count_zeroes(input: &str, count_passes: bool) -> crate::Result<usize> {
    let (mut count, mut dial) = (0, 50);
    for line in input.lines() {
        let n = if &line[0..1] == "L" { -1 } else { 1 } * line[1..].parse::<i32>()?;
        if count_passes {
            let (guaranteed, n) = (n.abs() as usize / 100, n % 100);
            count += guaranteed + usize::from(dial != 0 && (dial + n) < 0 || (dial + n) > 100);
        }
        dial = (dial + n).rem_euclid(100);
        count += usize::from(dial == 0)
    }
    Ok(count)
}
