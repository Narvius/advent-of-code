use std::collections::HashSet;

/// Find the position of the first collision.
pub fn one(input: &str) -> crate::Result<String> {
    let (grid, mut carts) = parse(input);

    loop {
        // Carts move in a specific order: top-to-bottom, left-to-right.
        carts.sort_unstable_by_key(|cart| cart.pos.1 * 10000 + cart.pos.0);
        for i in 0..carts.len() {
            if let Some((x, y)) = step(&grid, &mut carts, i) {
                return Ok(format!("{x},{y}"));
            }
        }
    }
}

/// Find the position of the final cart after all other carts have collided.
pub fn two(input: &str) -> crate::Result<String> {
    let (grid, mut carts) = parse(input);

    // Run full ticks (one move per cart) until only one cart remains.
    while carts.len() > 1 {
        // Remember crashes that occured in this tick and don't move carts that participate
        // in them.
        let mut crashes = HashSet::new();
        carts.sort_unstable_by_key(|cart| cart.pos.1 * 10000 + cart.pos.0);
        for i in 0..carts.len() {
            if !crashes.contains(&carts[i].pos) {
                if let Some(p) = step(&grid, &mut carts, i) {
                    crashes.insert(p);
                }
            }
        }
        // Remove all carts that have crashed between ticks.
        carts.retain(|c| !crashes.contains(&c.pos));
    }

    let (x, y) = carts[0].pos;
    Ok(format!("{x},{y}"))
}

/// Steps the cart with the given index once.
fn step(grid: &[&[u8]], carts: &mut [Cart], cart: usize) -> Option<(i32, i32)> {
    let Cart { pos: (x, y), dir, turns } = carts[cart];
    
    carts[cart].dir = match grid[y as usize][x as usize] {
        b'/' => SLASH[dir],
        b'\\' => BSLSH[dir],
        b'+' => {
            carts[cart].turns += 1;
            (4 + dir + turns % 3 - 1) % 4
        }
        _ => dir
    };

    let (dx, dy) = DELTA[carts[cart].dir];
    let pos = (x + dx, y + dy);

    carts[cart].pos = pos;
    (carts.iter().filter(|c| c.pos == pos).count() >= 2).then(|| pos)
}

/// Parses the puzzle input into a grid of bytes (the map) and a list of carts.
fn parse(input: &str) -> (Vec<&[u8]>, Vec<Cart>) {
    let mut carts = vec![];
    let lines: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();

    fn dir_from_byte(b: u8) -> Option<usize> {
        Some(match b {
            b'<' => 0,
            b'^' => 1,
            b'>' => 2,
            b'v' => 3,
            _ => return None,
        })
    }

    for (y, line) in lines.iter().enumerate() {
        for (x, &b) in line.iter().enumerate() {
            if let Some(dir) = dir_from_byte(b) {
                carts.push(Cart {
                    pos: (x as i32, y as i32),
                    dir,
                    turns: 0,
                })
            }
        }
    }

    (lines, carts)
}

/// A single cart, represented as a position, a direction index, and the number of turns
/// performed.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Cart {
    pos: (i32, i32),
    dir: usize,
    turns: usize,
}

/// Maps a direction index to the coordinate delta when going that direction, starting at
/// 0 for left and counting up going clockwise.
const DELTA: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
/// Maps the direction going into a / bend to the direction going out of it.
const SLASH: [usize; 4] = [3, 2, 1, 0];
/// Maps the direction going into a \ bend to the direction going out of it.
const BSLSH: [usize; 4] = [1, 0, 3, 2];
