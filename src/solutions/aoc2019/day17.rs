use std::fmt::Write;

use crate::common::intcode::v2::*;

/// Find the sum of a value calculated for each intersection in the scaffold map.
pub fn one(input: &str) -> crate::Result<usize> {
    let Layout { map, .. } = read_layout(input)?;

    const DELTAS: [(i32, i32); 5] = [(-1, 0), (0, -1), (1, 0), (0, 1), (0, 0)];

    let mut result = 0;
    for x in 1..map[0].len() - 1 {
        for y in 1..map.len() - 1 {
            if DELTAS
                .iter()
                .all(|&(dx, dy)| map[(y as i32 + dy) as usize][(x as i32 + dx) as usize])
            {
                result += x * y;
            }
        }
    }

    Ok(result)
}

/// Make the robot walk the entire scaffold, using the limited movement routine system provided.
/// Then return the final number it outputs.
pub fn two(input: &str) -> crate::Result<Int> {
    let data = read_layout(input)?;
    let (main, subs) = into_routines(to_walk_plan(data))?;

    // Prepare program, feed input lines it, feed hard-coded "n" for video feed to it.
    let mut p = Program::with_capacity(input, 5000)?;
    p.code[0] = 2;
    for line in std::iter::once(main).chain(subs.into_iter()) {
        for b in line.trim_end_matches(',').bytes() {
            p.input.push_back(b as Int);
        }
        p.input.push_back(10);
    }
    p.input.push_back(b'n' as Int);
    p.input.push_back(10);

    p.run()?;

    // There's a bunch of output, the last one added is our result.
    p.output.pop_back().ok_or("no result".into())
}

/// Information about the scaffolding and robot extracted from the program.
struct Layout {
    map: Vec<Vec<bool>>,
    robot: ((i32, i32), (i32, i32)),
}

/// Splits the provided walk plan into a main routine and three subroutines.
fn into_routines(mut walk_plan: String) -> crate::Result<(String, [String; 3])> {
    // Creates a copy of the substring corresponding to the first `tokens` tokens in the
    // provided walk plan.
    fn from_tokens(s: &str, tokens: usize) -> String {
        let length = tokens + s.split(',').take(tokens).map(|v| v.len()).sum::<usize>();
        s[0..length].to_string()
    }

    // Try subroutines made of different amounts of tokens. If a combination of subroutines
    // covers the entire plan, we have a solution.
    // 1) every subroutine must have an even number of tokens (each pair is a turn + straight)
    // 2) reasonable assumption that each subroutine is at least three such pairs
    // 3) subroutines can't be longer than 10 tokens, ever (each token is at least 2 chars)
    for a in [6, 8, 10] {
        for b in [6, 8, 10] {
            for c in [6, 8, 10] {
                let mut plan = walk_plan.clone();
                let mut subroutines = [String::new(), String::new(), String::new()];

                for (i, n) in [a, b, c].into_iter().enumerate() {
                    // Build a subroutine from the first `n` tokens.
                    subroutines[i] = from_tokens(&plan, n);
                    // Remove parts covered by this subroutine.
                    plan = plan.replace(&subroutines[i], "");
                }

                if plan.is_empty() {
                    walk_plan = walk_plan.replace(&subroutines[0], "A,");
                    walk_plan = walk_plan.replace(&subroutines[1], "B,");
                    walk_plan = walk_plan.replace(&subroutines[2], "C,");

                    return Ok((walk_plan, subroutines));
                }
            }
        }
    }

    Err("no solution".into())
}

/// Construct the walk plan out of scaffold data.
///
/// A walk plan is a string made of comma-delimited entries, where each entry is either an L, an
/// R or a number. Note that letters and numbers always alternate.
fn to_walk_plan(Layout { map: data, robot }: Layout) -> String {
    let mut result = String::new();
    let (mut pos, mut dir) = robot;

    fn walkable(map: &[Vec<bool>], (x, y): (i32, i32), (dx, dy): (i32, i32)) -> bool {
        let (x, y) = (usize::try_from(x + dx), usize::try_from(y + dy));
        if let (Ok(x), Ok(y)) = (x, y) {
            *map.get(y).and_then(|m| m.get(x)).unwrap_or(&false)
        } else {
            false
        }
    }

    fn dirs((x, y): (i32, i32)) -> impl Iterator<Item = ((i32, i32), &'static str)> {
        [((x, y), ""), ((y, -x), "L,"), ((-y, x), "R,")].into_iter()
    }

    while let Some((next_dir, tag)) = dirs(dir).find(|dir| walkable(&data, pos, dir.0)) {
        result.push_str(tag);
        if dir == next_dir {
            let mut distance = 0;
            while walkable(&data, pos, dir) {
                distance += 1;
                pos = (pos.0 + dir.0, pos.1 + dir.1);
            }
            write!(result, "{distance},").expect("write to string doesn't fail");
        }

        dir = next_dir;
    }
    result
}

/// Executes the input program and extracts the scaffolding map from it.
fn read_layout(input: &str) -> crate::Result<Layout> {
    let mut p = Program::with_capacity(input, 5000)?;
    let mut map = vec![vec![]];
    let mut pos = (0, 0);
    let mut dir = (0, 0);

    p.run()?;
    while let Some(c) = p.output.pop_front() {
        match c as u8 {
            b'#' => map.last_mut().unwrap().push(true),
            b'.' => map.last_mut().unwrap().push(false),
            b'<' | b'^' | b'>' | b'v' => {
                map.last_mut().unwrap().push(true);
                pos = (map.last().unwrap().len() as i32 - 1, map.len() as i32 - 1);
                dir = match c as u8 {
                    b'<' => (-1, 0),
                    b'^' => (0, -1),
                    b'>' => (1, 0),
                    b'v' => (0, 1),
                    _ => return Err("unreachable".into()),
                }
            }
            10 => map.push(vec![]),
            _ => return Err("unknown output from program".into()),
        }
    }

    while map.last_mut().unwrap().is_empty() {
        map.pop();
    }

    Ok(Layout {
        map,
        robot: (pos, dir),
    })
}
