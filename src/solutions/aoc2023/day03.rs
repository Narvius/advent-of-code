/// For each symbol, find all adjacent numbers, and find their sum.
///
/// Note that in the puzzle input, each number is only ever adjacent to up to one symbol; so
/// searching from the symbol side will not result in any number being counted twice.
pub fn one(input: &str) -> crate::Result<i32> {
    let is_symbol = |c: u8| !c.is_ascii_digit() && c != b'.';
    adjacent_number_pair_sum(input, is_symbol, |(n1, n2)| n1 + n2)
}

/// Find each `*` with two adjacent numbers, multiply them; find the sum of all such products.
pub fn two(input: &str) -> crate::Result<i32> {
    adjacent_number_pair_sum(input, |c| c == b'*', |(n1, n2)| n1 * n2)
}

/// For every position that matches `include`, finds the 1 or 2 adjacent numbers to that position;
/// maps them with `f` and then returns the sum of all such results.
fn adjacent_number_pair_sum(
    input: &str,
    include: fn(u8) -> bool,
    f: fn((i32, i32)) -> i32,
) -> crate::Result<i32> {
    let map: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();

    Ok((0..map.len())
        .flat_map(|y| (0..map[y].len()).map(move |x| (x, y)))
        .filter(|&(x, y)| include(map[y][x]))
        .map(|(x, y)| f(adjacent_numbers(&map, (x as i32, y as i32))))
        .sum())
}

/// Finds up to two numbers adjacent to the given position. If a number wasn't found, returns 0 for
/// it.
///
/// Note that in the puzzle input, there is only ever up to two numbers adjacent to any given
/// symbol.
fn adjacent_numbers(map: &[&[u8]], (x, y): (i32, i32)) -> (i32, i32) {
    let mut first = None;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            if let Some((n1, l1, r1)) = number_at(map, (x + dx, y + dy)) {
                match first {
                    Some((n2, y2, l2, r2)) if (l1, r1, y2) != (l2, r2, y + dy) => return (n1, n2),
                    None => first = Some((n1, y + dy, l1, r1)),
                    _ => {}
                }
            }
        }
    }
    (first.map(|(n1, _, _, _)| n1).unwrap_or(0), 0)
}

/// Parses the number at the given position, and returns the number alongside two numbers that form
/// the (exclusive) range of x coordinates that got parsed.
fn number_at(map: &[&[u8]], (mut x, y): (i32, i32)) -> Option<(i32, usize, usize)> {
    fn get(map: &[&[u8]], (x, y): (i32, i32)) -> Option<u8> {
        let (x, y) = (usize::try_from(x).ok()?, usize::try_from(y).ok()?);
        map.get(y)?.get(x).copied()
    }

    if !get(map, (x, y))?.is_ascii_digit() {
        return None;
    }

    let mut stride = 1;
    // Scan left until we no longer hit a digit.
    while get(map, (x - 1, y)).unwrap_or(b' ').is_ascii_digit() {
        x -= 1;
        stride += 1;
    }

    // Scan right until we no longer hit a digit.
    while get(map, (x + stride as i32, y))
        .unwrap_or(b' ')
        .is_ascii_digit()
    {
        stride += 1;
    }

    // Actually parse the number. Unfortunately we allocate a new string here just for the parse;
    // theoretically, that could be avoided by passing the original line as a string and slicing
    // it; but not worth the effort probably.
    let (y, x) = (y as usize, x as usize);
    let num = String::from_utf8(map[y][x..x + stride].to_vec()).ok()?;
    Some((num.parse().ok()?, x, x + stride))
}
