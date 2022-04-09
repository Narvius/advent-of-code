use std::collections::HashSet;

/// Find the number of houses visited by Santa.
pub fn one(input: &str) -> Result<String, String> {
    let mut map: HashSet<_> = HashSet::from([(0, 0)]);
    walk(&mut map, input.chars());
    Ok(map.len().to_string())
}

/// Find the number of houses visited by Santa and Robo-Santa.
pub fn two(input: &str) -> Result<String, String> {
    let mut map: HashSet<_> = HashSet::from([(0, 0)]);
    let mut i = 0;
    let (a, b): (Vec<_>, Vec<_>) = input.chars().partition(|_| {
        i += 1;
        i % 2 == 0
    });
    walk(&mut map, a);
    walk(&mut map, b);
    Ok(map.len().to_string())
}

/// Executes a Santa walk.
fn walk(map: &mut HashSet<(i32, i32)>, steps: impl IntoIterator<Item = char>) {
    let mut position = (0, 0);
    for step in steps {
        match step {
            '<' => position.0 -= 1,
            '^' => position.1 -= 1,
            '>' => position.0 += 1,
            'v' => position.1 += 1,
            _ => {}
        };
        map.insert(position);
    }
}
