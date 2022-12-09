use std::collections::HashSet;

/// For a rope of length 2, find the number of tiles visited by the tail.
pub fn one(input: &str) -> crate::Result<usize> {
    run_snake(input, 2)
}

/// For a rope of length 10, find the number of tiles visited by the tail.
pub fn two(input: &str) -> crate::Result<usize> {
    run_snake(input, 10)
}

/// Executes a series of moves from input for a snake with the given `length`, and returns
/// the number of tiles visited by the snake tail.
fn run_snake(input: &str, length: usize) -> crate::Result<usize> {
    let mut snake = vec![(0, 0); length];
    let mut visited = HashSet::from([(0, 0)]);

    for (n, (dx, dy)) in parse(input) {
        for _ in 0..n {
            snake[0] = (snake[0].0 + dx, snake[0].1 + dy);
            for i in 1..snake.len() {
                let [head, tail] = &mut snake[(i - 1)..=i] else { unreachable!() };

                if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
                    break;
                }

                tail.0 += (head.0 - tail.0).signum();
                tail.1 += (head.1 - tail.1).signum();
            }

            visited.insert(snake[snake.len() - 1]);
        }
    }

    Ok(visited.len())
}

/// Parses the puzzle into a series of moves (length and direction delta).
fn parse(input: &str) -> impl Iterator<Item = (i32, (i32, i32))> + '_ {
    input.lines().filter_map(|line| {
        let (dir, n) = line.split_once(' ')?;
        Some((
            n.parse().ok()?,
            match dir {
                "L" => (-1, 0),
                "U" => (0, -1),
                "R" => (1, 0),
                "D" => (0, 1),
                _ => None?,
            },
        ))
    })
}
