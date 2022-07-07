/// Find the number of file server pairs where one could fit its data on the other.
pub fn one(input: &str) -> crate::Result<String> {
    let data = parse(input);
    let mut result = 0;
    for i in 0..data.len() {
        for j in 0..data.len() {
            if i != j && data[i].used > 0 && data[i].used <= data[j].avail {
                result += 1;
            }
        }
    }
    Ok(result.to_string())
}

/// Find the least number of steps required to move the goal data to (0, 0).
pub fn two(input: &str) -> crate::Result<String> {
    // This setup behaves like a sliding puzzle. There's only three types of servers: Big, small
    // and open.
    // Big ones can never have their data moved, and as such behave like walls.
    // Small ones all have enough space to store each other's data. They will behave like the
    // tiles in a sliding puzzle.
    // And then there is one singular "open" server with 0% memory usage. That is the hole we
    // move around like in a sliding puzzle, and that's called the "cursor" here.
    // The walls are arranged in one horizontal line that we need to navigate around with the
    // cursor; and moving the goal data to the left requires moving the hole onto it from the
    // left (again, like in a sliding puzzle). Thus it takes 5 steps (down, left, left, up, right)
    // to move the goal data to the left by one.

    let mut wall = usize::MAX;
    let mut cursor = (0, 0);
    let mut goal_x = 0;
    let mut steps = 0;

    // Extract cursor and wall positions from the input data.
    for node in parse(input) {
        match (node.size > 100, node.avail > 50) {
            (true, _) => wall = wall.min(node.pos.0),
            (_, true) => cursor = node.pos,
            _ => goal_x = goal_x.max(node.pos.0),
        }
    }

    // Aligning ourselves with the goal: Y coordinate is how much we need to step upwards, wall
    // dictates how far we have to go left before we can start going right. We're done when we
    // reach the coordinates of the goal data, which also moves it one to the left already.
    steps += cursor.1 + 2 * (cursor.0 + 1 - wall);
    steps += goal_x - cursor.0;
    cursor.0 = goal_x;
    goal_x -= 1;

    // From here we can just loop the 5 step sequence to move it one left until it reaches (0, 0).
    steps += 5 * goal_x;

    Ok(steps.to_string())
}

/// A single file server, as per the puzzle description.
struct Node {
    pos: (usize, usize),
    size: usize,
    used: usize,
    avail: usize,
}

/// Parses the puzzle input into a series of [`Node`]s.
fn parse(input: &str) -> Vec<Node> {
    input
        .lines()
        .filter_map(|line| {
            let tokens: Vec<_> = line.split_whitespace().collect();
            if let &[label, size, used, avail, _] = tokens.as_slice() {
                let (_, xy) = label.split_once("-x")?;
                let (x, y) = xy.split_once("-y")?;

                Some(Node {
                    pos: (x.parse().ok()?, y.parse().ok()?),
                    size: size[0..size.len() - 1].parse().ok()?,
                    used: used[0..used.len() - 1].parse().ok()?,
                    avail: avail[0..avail.len() - 1].parse().ok()?,
                })
            } else {
                None
            }
        })
        .collect()
}
