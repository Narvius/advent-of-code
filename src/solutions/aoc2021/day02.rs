/// Run the submarine course, reading the commands in a simple way.
pub fn one(input: &str) -> crate::Result<i32> {
    let (h, d) = parse(input).fold((0i32, 0i32), |(h, d), cmd| match cmd {
        Command::Up(v) => (h, d - v),
        Command::Down(v) => (h, d + v),
        Command::Forward(v) => (h + v, d),
    });

    Ok(h * d)
}

/// Run the submarine course, reading the commands in a slightly convoluted way described in the
/// problem statement.
pub fn two(input: &str) -> crate::Result<i32> {
    let (h, d, _) = parse(input).fold((0i32, 0i32, 0i32), |(h, d, a), cmd| match cmd {
        Command::Up(v) => (h, d, a - v),
        Command::Down(v) => (h, d, a + v),
        Command::Forward(v) => (h + v, d + a * v, a),
    });

    Ok(h * d)
}

fn parse(input: &str) -> impl Iterator<Item = Command> + '_ {
    fn item(line: &str) -> Option<Command> {
        let (dir, arg) = line.split_once(' ')?;
        Some(match dir {
            "up" => Command::Up(arg.parse().ok()?),
            "down" => Command::Down(arg.parse().ok()?),
            "forward" => Command::Forward(arg.parse().ok()?),
            _ => return None,
        })
    }

    input.lines().filter_map(item)
}

/// A parsed line of puzzle input.
enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}
