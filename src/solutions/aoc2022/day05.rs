pub fn one(input: &str) -> crate::Result<String> {
    let (mut stacks, commands) = parse(input).ok_or("parse failed")?;

    for (n, from, to) in commands {
        for _ in 0..n {
            let elem = stacks[from as usize].pop().ok_or("popped too much")?;
            stacks[to as usize].push(elem);
        }
    }

    Ok(stacks
        .into_iter()
        .filter_map(|v| v.last().copied())
        .collect())
}

pub fn two(input: &str) -> crate::Result<&str> {
    Err("unimplemented".into())
}

fn parse(input: &str) -> Option<(Vec<Vec<char>>, impl Iterator<Item = (i32, i32, i32)> + '_)> {
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
    Some((
        stacks,
        lines.filter_map(|line| {
            let mut items = line
                .split_ascii_whitespace()
                .filter_map(|v| v.parse::<i32>().ok());
            Some((items.next()?, items.next()? - 1, items.next()? - 1))
        }),
    ))
}
