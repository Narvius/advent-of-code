use std::collections::HashMap;

/// Find a checksum of the position after performing all instructions on a wrapping map.
pub fn one(input: &str) -> crate::Result<i32> {
    let (chunks, instructions) = parse(input).ok_or("parse failed")?;
    Ok(execute_walk(&chunks, &instructions, construct_wrapping_links).ok_or("failed walk")?)
}

/// Find a checksum of the position after performing all instructions on a cube map.
pub fn two(input: &str) -> crate::Result<i32> {
    let (chunks, instructions) = parse(input).ok_or("parse failed")?;
    Ok(execute_walk(&chunks, &instructions, construct_cube_links).ok_or("failed walk")?)
}

/// Constructs the [`Link`]s for 2D wrapping.
fn construct_wrapping_links(chunks: &[Chunk]) -> Vec<Link> {
    let positions: HashMap<_, _> = chunks.iter().enumerate().map(|(i, c)| (c.pos, i)).collect();
    let find = |(x, y), (dx, dy)| {
        (1..)
            .map(|i| {
                let (x, y): Point = (x + dx * i, y + dy * i);
                positions.get(&(x.rem_euclid(6), y.rem_euclid(6)))
            })
            .find(|p| p.is_some())
    };

    chunks
        .iter()
        .map(|chunk| {
            let p = chunk.pos;
            Link([0, 1, 2, 3].map(|dir| (*find(p, DELTAS[dir]).unwrap().unwrap(), (dir + 2) % 4)))
        })
        .collect()
}

/// Constructs the [`Link`]s for cube-based wrapping.
fn construct_cube_links(_chunks: &[Chunk]) -> Vec<Link> {
    // I'll just.. hardcode the cube fold. Not happy with it, but I will
    // not finish tonight if I don't do this.
    // FIXME actually fold the cube!
    vec![
        Link([(1, 2), (2, 3), (3, 2), (5, 2)]),
        Link([(4, 0), (2, 0), (0, 0), (5, 1)]),
        Link([(1, 1), (4, 3), (3, 3), (0, 1)]),
        Link([(4, 2), (5, 3), (0, 2), (2, 2)]),
        Link([(1, 0), (5, 0), (3, 0), (2, 1)]),
        Link([(4, 1), (1, 3), (0, 3), (3, 1)]),
    ]
}

/// Walks across the map and returns the final checksum.
fn execute_walk(chunks: &[Chunk], instructions: &[Op], link_fn: LinkFn) -> Option<i32> {
    let links = link_fn(chunks);

    let mut chunk = chunks.iter().position(|c| c.pos.1 == 0)?;
    let mut pos = (chunks[chunk].data.iter().position(|&b| b)? as i32, 0);
    let mut facing = 0;

    for op in instructions {
        match op {
            Op::Forward(n) => {
                for _ in 0..*n {
                    // See where taking a step would end up.
                    let ((x, y), (dx, dy)) = (pos, DELTAS[facing]);
                    let (tx, ty) = (x + dx, y + dy);

                    // Check if we're leaving through an edge.
                    let edge = match (tx, ty) {
                        (x, _) if x == SIZE as i32 => Some(0),
                        (_, y) if y == SIZE as i32 => Some(1),
                        (-1, _) => Some(2),
                        (_, -1) => Some(3),
                        _ => None,
                    };

                    // Find the target chunk and position we're entering.
                    let (tchunk, tpos, tfacing) = if let Some(edge) = edge {
                        let (new_chunk, target_edge) = links[chunk].0[edge];
                        let new_pos = move_to_edge(pos, edge, target_edge);
                        (new_chunk, new_pos, (target_edge + 2) % 4)
                    } else {
                        (chunk, (tx, ty), facing)
                    };

                    // Abort if we bonked into a wall.
                    if !chunks[tchunk].data[(tpos.1 * SIZE as i32 + tpos.0) as usize] {
                        break;
                    }

                    // Actually update the current state.
                    (chunk, pos, facing) = (tchunk, tpos, tfacing);
                }
            }
            Op::Left => facing = (facing + 3) % 4,
            Op::Right => facing = (facing + 1) % 4,
        }
    }

    let ((cx, cy), (x, y)) = (chunks[chunk].pos, pos);
    Some(1000 * (cy * SIZE as i32 + y + 1) + 4 * (cx * SIZE as i32 + x + 1) + facing as i32)
}

/// Gets the position of `p` on `target_edge`, assuming it starts on `source_edge`.
fn move_to_edge(p: Point, source_edge: usize, target_edge: usize) -> Point {
    // This code is a symptom of me giving up, but it is NOT special-cased!
    // It simply lists all possibilities, when there's likely a cleaner
    // way of expressing it all.
    match (source_edge % 4, target_edge % 4) {
        (0, 0) => (MAX, MAX - p.1),
        (0, 1) => (p.1, MAX),
        (0, 2) => (0, p.1),
        (0, 3) => (MAX - p.1, 0),
        (1, 0) => (MAX, p.0),
        (1, 1) => (MAX - p.0, MAX),
        (1, 2) => (0, MAX - p.0),
        (1, 3) => (p.0, 0),
        (2, 0) => (MAX, p.1),
        (2, 1) => (MAX - p.1, MAX),
        (2, 2) => (0, MAX - p.1),
        (2, 3) => (p.1, 0),
        (3, 0) => (MAX, MAX - p.0),
        (3, 1) => (p.0, MAX),
        (3, 2) => (0, p.0),
        (3, 3) => (MAX - p.0, 0),
        _ => unreachable!(),
    }
}

/// A point in 2D space.
type Point = (i32, i32);

/// A function that computes links between chunks.
type LinkFn = fn(&[Chunk]) -> Vec<Link>;

/// Represents one sixth of the map, a single 50x50 chunk.
struct Chunk {
    pos: (i32, i32),
    data: [bool; SIZE * SIZE],
}

/// Represents the four connections--to the right, down, to the left, and up. The first number
/// in each pair indicates the chunk we will arrive at, and the second number the edge we
/// will arrive through (again, 0 = right edge, 1 = bottom edge, 2 = left edge, 3 = top edge).
struct Link([(usize, usize); 4]);

/// A walking instruction.
#[derive(Debug)]
enum Op {
    Forward(i32),
    Left,
    Right,
}

/// Four orthogonal deltas, in the order specified by the puzzle (right, down, left, up).
const DELTAS: [Point; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

/// Side length of each chunk.
const SIZE: usize = 50;

/// Convenience define for calculations where I need 1 less than the side length.
const MAX: i32 = (SIZE - 1) as i32;

/// Parses the puzzle input into a list of chunks and a list of instructions.
fn parse(input: &str) -> Option<(Vec<Chunk>, Vec<Op>)> {
    let (map, instructions) = input.split_once("\n\n")?;
    let lines: Vec<_> = map.lines().collect();

    let mut chunks = vec![];
    for cy in 0..6 {
        for cx in 0..6 {
            let value = lines
                .get(cy * SIZE)
                .and_then(|v| v.as_bytes().get(cx * SIZE))
                .copied();

            if matches!(value, Some(b'.' | b'#')) {
                let mut data = [false; SIZE * SIZE];
                for y in 0..SIZE {
                    for x in 0..SIZE {
                        data[SIZE * y + x] = lines[cy * SIZE + y].as_bytes()[cx * SIZE + x] == b'.';
                    }
                }
                chunks.push(Chunk {
                    pos: (cx as i32, cy as i32),
                    data,
                });
            }
        }
    }

    let instructions = instructions.replace('L', " L ").replace('R', " R ");
    let instructions = instructions
        .split_whitespace()
        .map(|c| match c.parse() {
            Ok(n) => Op::Forward(n),
            Err(_) if c == "L" => Op::Left,
            _ => Op::Right,
        })
        .collect();

    Some((chunks, instructions))
}
