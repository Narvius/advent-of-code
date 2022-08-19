/// Find the strongest possible bridge.
pub fn one(input: &str) -> crate::Result<usize> {
    let items = parse(input);
    let mask = (1 << items.len()) - 1;

    fn strongest(items: &[(usize, usize)], power: usize, port: usize, mask: u64) -> usize {
        let mut result = power;
        for (i, &(a, b)) in items.iter().enumerate() {
            if available(mask, i) && (a == port || b == port) {
                let strongest = strongest(items, power + a + b, a + b - port, without(mask, i));
                result = result.max(strongest);
            }
        }
        result
    }

    Ok(strongest(&items, 0, 0, mask))
}

/// Find the strongest possible bridge amongst all the longest possible bridges.
pub fn two(input: &str) -> crate::Result<usize> {
    let items = parse(input);
    let mask = (1 << items.len()) - 1;

    fn longest(items: &[(usize, usize)], power: usize, port: usize, mask: u64) -> (usize, u32) {
        let mut result = (power, items.len() as u32 - mask.count_ones());
        for (i, &(a, b)) in items.iter().enumerate() {
            if available(mask, i) && (a == port || b == port) {
                let (strength, length) =
                    longest(items, power + a + b, a + b - port, without(mask, i));
                if result.1 < length || (result.1 == length && result.0 < strength) {
                    result = (strength, length);
                }
            }
        }
        result
    }

    Ok(longest(&items, 0, 0, mask).0)
}

/// Checks whether the `i`thbit in a bitmask is set.
fn available(mask: u64, i: usize) -> bool {
    (mask & (1 << i)) > 0
}

/// Returns a bitmask with the `i`th bit turned off.
fn without(mask: u64, i: usize) -> u64 {
    mask ^ (1 << i)
}

/// Parses the puzzle input.
fn parse(s: &str) -> Vec<(usize, usize)> {
    s.lines()
        .filter_map(|line| {
            let (a, b) = line.split_once('/')?;
            Some((a.parse().ok()?, b.parse().ok()?))
        })
        .collect()
}
