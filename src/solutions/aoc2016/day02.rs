use std::collections::HashMap;

/// Follow the instruction on the simple keypad, return the resulting code.
pub fn one(input: &str) -> crate::Result<String> {
    let map = build_map(&["123", "456", "789"]);
    Ok(walk(input, (1, 1), map))
}

/// Follow the instruction on the advanced keypad, return the resulting code.
pub fn two(input: &str) -> crate::Result<String> {
    let map = build_map(&["  1  ", " 234 ", "56789", " ABC ", "  D  "]);
    Ok(walk(input, (2, 2), map))
}

/// Follows the keypad instructions, and returns the resulting code.
fn walk(input: &str, mut pos: (i32, i32), map: HashMap<(i32, i32), char>) -> String {
    let mut output = String::new();
    for line in input.lines() {
        for c in line.chars() {
            let p = match c {
                'L' => (pos.0 - 1, pos.1),
                'U' => (pos.0, pos.1 - 1),
                'R' => (pos.0 + 1, pos.1),
                'D' => (pos.0, pos.1 + 1),
                _ => pos,
            };
            if map.contains_key(&p) {
                pos = p;
            }
        }
        output.push(map[&pos]);
    }
    output
}

/// Builds a map compatible with `walk`. `x` and `y` are the top left corner.
fn build_map(cs: &[&str]) -> HashMap<(i32, i32), char> {
    HashMap::from_iter((0..cs.len()).flat_map(|y| {
        cs[y]
            .char_indices()
            .filter_map(move |(x, c)| (c != ' ').then_some(((x as i32, y as i32), c)))
    }))
}
