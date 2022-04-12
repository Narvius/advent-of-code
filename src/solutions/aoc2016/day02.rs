use std::collections::HashMap;

/// Follow the instruction on the simple keypad, return the resulting code.
pub fn one(input: &str) -> Result<String, String> {
    let map = build_map(-1, -1, &["123", "456", "789"]);
    Ok(walk(input, map))
}

/// Follow the instruction on the advanced keypad, return the resulting code.
pub fn two(input: &str) -> Result<String, String> {
    let map = build_map(-2, -2, &["  1  ", " 234 ", "56789", " ABC ", "  D  "]);
    Ok(walk(input, map))
}

/// Follows the keypad instructions, and returns the resulting code.
fn walk(input: &str, map: HashMap<(i32, i32), char>) -> String {
    let mut s = String::new();
    let mut p = (0, 0);
    for line in input.lines() {
        for c in line.chars() {
            let pp = match c {
                'L' => (p.0 - 1, p.1),
                'U' => (p.0, p.1 - 1),
                'R' => (p.0 + 1, p.1),
                'D' => (p.0, p.1 + 1),
                _ => p,
            };
            if map.contains_key(&pp) {
                p = pp;
            }
        }
        s.push(map[&p]);
    }
    s
}

/// Builds a map compatible with `walk`. `x` and `y` are the top left corner.
fn build_map(x: i32, y: i32, cs: &[&str]) -> HashMap<(i32, i32), char> {
    HashMap::from_iter((0..cs.len()).flat_map(|dy| {
        cs[dy]
            .char_indices()
            .filter_map(move |(dx, c)| (c != ' ').then(|| ((x + dx as i32, y + dy as i32), c)))
    }))
}
