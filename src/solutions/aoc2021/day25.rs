/// Count how many times the sea cucumbers move before they stop.
pub fn one(input: &str) -> crate::Result<usize> {
    let mut map: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    for i in 1.. {
        let moved_east = walk(&mut map, b'>', (1, 0));
        let moved_south = walk(&mut map, b'v', (0, 1));

        if !moved_east && !moved_south {
            return Ok(i);
        }
    }

    unreachable!()
}

/// Freebie!
pub fn two(_input: &str) -> crate::Result<&str> {
    Ok("done")
}

/// Performs one step of sea cucumber walking, for the given `herd`, in the
/// given direction (`dx`, `dy`).
fn walk(map: &mut [Vec<u8>], herd: u8, (dx, dy): (usize, usize)) -> bool {
    let mut to_move = vec![];
    let width = map[0].len();
    let height = map.len();

    for y in 0..height {
        for x in 0..width {
            if map[y][x] == herd && map[(y + dy) % height][(x + dx) % width] == b'.' {
                to_move.push((x, y));
            }
        }
    }

    for &(x, y) in &to_move {
        map[y][x] = b'.';
        map[(y + dy) % height][(x + dx) % width] = herd;
    }

    !to_move.is_empty()
}
