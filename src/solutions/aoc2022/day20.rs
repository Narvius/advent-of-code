/// Mix the list once and calculate a checksum.
pub fn one(input: &str) -> crate::Result<i64> {
    let (mut items, mut order) = parse(input, 1);
    mix_list(&mut items, &mut order)?;
    grove_coordinates(&items)
}

/// Pre-multiply numbers in the list, mix them 10 times, and calculate the same checksum.
pub fn two(input: &str) -> crate::Result<i64> {
    let (mut items, mut order) = parse(input, 811589153);
    for _ in 0..10 {
        mix_list(&mut items, &mut order)?;
    }
    grove_coordinates(&items)
}

/// Mixes the list as per the puzzle input. `items` and `order` are mixed in the exact same way,
/// so that consecutive invocations with the same lists will behave correctly for part 2.
fn mix_list(items: &mut Vec<i64>, order: &mut Vec<usize>) -> crate::Result<()> {
    for i in 0..items.len() {
        let Some(p) = order.iter().position(|&n| i == n) else { Err("failed")? };
        let item = items.remove(p);
        let new_pos = (p as i64 + item).rem_euclid(items.len() as i64) as usize;
        items.insert(new_pos, item);
        let item = order.remove(p);
        order.insert(new_pos, item);
    }
    Ok(())
}

/// Calculates the grove coordinates from a list of items.
fn grove_coordinates(items: &[i64]) -> crate::Result<i64> {
    let Some(zero) = items.iter().position(|&n| n == 0) else { Err("no zero")? };
    let len = items.len();

    Ok(items[(zero + 1000) % len] + items[(zero + 2000) % len] + items[(zero + 3000) % len])
}

/// Parses the puzzle input into a list of numbers (with an optional multiplier applied), and
/// an "order" list, which indicates the order in which numbers should be moved. For details,
/// see the puzzle description.
fn parse(input: &str, multiplier: i64) -> (Vec<i64>, Vec<usize>) {
    let items: Vec<_> = input
        .lines()
        .filter_map(|v| v.parse::<i64>().ok())
        .map(|v| v * multiplier)
        .collect();
    let order = Vec::from_iter(0..items.len());
    (items, order)
}
