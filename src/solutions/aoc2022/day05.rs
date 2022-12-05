use std::cmp::Ordering;

/// Moving crates one at a time, execute all commands; then read the top string.
pub fn one(input: &str) -> crate::Result<String> {
    run_crates(input, |from, to, n| {
        for _ in 0..n {
            to.push(from.pop().unwrap());
        }
    })
}

/// Moving crates all together, execute all commands; then read the top string.
pub fn two(input: &str) -> crate::Result<String> {
    run_crates(input, |from, to, n| {
        to.extend(from.drain((from.len() - n as usize)..))
    })
}

/// A method that modifies the `from` and `to` stacks by moving the given amount of crates.
type MoveFn = fn(&mut Vec<char>, &mut Vec<char>, i32);
/// A command from puzzle input; the number of crates to move, and the indices of the `from`
/// and `to` stacks.
type Command = (i32, usize, usize);

/// Executes all commands given in the puzzle input and returns a string built from the top
/// character of each stack. `move_fn` is called to perform the actual stack manipulation.
fn run_crates(input: &str, move_fn: MoveFn) -> crate::Result<String> {
    let (mut stacks, commands) = parse(input).ok_or("parse failed")?;

    for (n, from, to) in commands {
        let (a, b) = stacks.split_at_mut(from.max(to));
        match from.cmp(&to) {
            Ordering::Less => move_fn(&mut a[from], &mut b[0], n),
            Ordering::Equal => continue,
            Ordering::Greater => move_fn(&mut b[0], &mut a[to], n),
        };
    }

    Ok(stacks
        .into_iter()
        .filter_map(|v| v.last().copied())
        .collect())
}

/// Parses the puzzle input into a list of stacks, and a series of commands.
fn parse(input: &str) -> Option<(Vec<Vec<char>>, impl Iterator<Item = Command> + '_)> {
    let mut stacks = vec![vec![]; 9];
    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.starts_with(" 1") {
            break;
        }

        for i in 0..9 {
            let c = line[(1 + 4 * i)..(2 + 4 * i)].chars().next()?;
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    lines.next();
    let commands = lines.filter_map(|line| {
        let mut items = line
            .split_ascii_whitespace()
            .filter_map(|v| v.parse::<i32>().ok());
        Some((
            items.next()?,
            (items.next()? - 1) as usize,
            (items.next()? - 1) as usize,
        ))
    });

    Some((stacks, commands))
}
