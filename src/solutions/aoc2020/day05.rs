/// Find the highest seat Id.
pub fn one(input: &str) -> crate::Result<i32> {
    input
        .lines()
        .filter_map(seat_id)
        .max()
        .ok_or_else(|| "no result".into())
}

/// Find the seat Id not in the input, but for which both adjacent Ids are in the input.
pub fn two(input: &str) -> crate::Result<i32> {
    let mut ids: Vec<_> = input.lines().filter_map(seat_id).collect();
    ids.sort();
    ids.windows(2)
        .find(|w| (w[1] - w[0]) == 2)
        .map(|w| w[0] + 1)
        .ok_or_else(|| "no result".into())
}

/// Calculates the seat Id from an input line.
fn seat_id(line: &str) -> Option<i32> {
    let row = line[0..7].to_string().replace('F', "0").replace('B', "1");
    let col = line[7..10].to_string().replace('L', "0").replace('R', "1");

    Some(8 * i32::from_str_radix(&row, 2).ok()? + i32::from_str_radix(&col, 2).ok()?)
}
