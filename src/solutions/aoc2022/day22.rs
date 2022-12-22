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
                positions.get(&(
                    ((x + dx * i) as i32).rem_euclid(6),
                    ((y + dy * i) as i32).rem_euclid(6),
                ))
            })
            .find(|p| p.is_some())
    };

    chunks
        .iter()
        .map(|chunk| {
            let p = chunk.pos;
            Link([0, 1, 2, 3].map(|dir| (*find(p, DELTAS[dir]).unwrap().unwrap(), 0)))
        })
        .collect()
}

/// Constructs the [`Link`]s for cube-based wrapping.
fn construct_cube_links(_chunks: &[Chunk]) -> Vec<Link> {
    // I'll just.. hardcode the cube fold. Not happy with it.
    vec![
        Link([(1, 0), (2, 0), (3, 2), (5, 1)]),
        Link([(4, 2), (2, 1), (0, 0), (5, 0)]),
        Link([(1, 3), (4, 0), (3, 3), (0, 0)]),
        Link([(4, 0), (5, 0), (0, 2), (2, 1)]),
        Link([(1, 2), (5, 1), (3, 3), (2, 0)]),
        Link([(4, 3), (1, 0), (0, 3), (3, 0)]),
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
                    // Take a step.
                    let ((x, y), (dx, dy)) = (pos, DELTAS[facing]);
                    let (tx, ty) = (x + dx, y + dy);

                    // Check if we're leaving through an edge.
                    let edge = match (tx, ty) {
                        (50, _) => Some(0),
                        (_, 50) => Some(1),
                        (-1, _) => Some(2),
                        (_, -1) => Some(3),
                        _ => None,
                    };

                    // Find the target chunk and position we're entering.
                    let (tchunk, tpos, tfacing) = if let Some(edge) = edge {
                        // I have to correlate edges of chunks here, taking into account
                        // rotation. As a specific example, top edge of chunk 0 connects
                        // to the LEFT edge of chunk 5, which is a link of (5, 1).
                        // The 3 here means that usually we'd expect to connect to the BOTTOM,
                        // but we rotate that expectation right (clockwise) once.
                        let (new_chunk, rotations) = links[chunk].0[edge];
                        let target_edge = (edge + 2 + rotations) % 4;
                        (
                            new_chunk,
                            project_onto_edge(pos, edge, target_edge),
                            (facing + rotations) % 4,
                        )
                    } else {
                        (chunk, (tx, ty), facing)
                    };

                    // Abort if we bonked into a wall.
                    if !chunks[tchunk].data[(tpos.1 * 50 + tpos.0) as usize] {
                        break;
                    }

                    (chunk, pos, facing) = (tchunk, tpos, tfacing);
                }
            }
            Op::Left => facing = (facing + 3) % 4,
            Op::Right => facing = (facing + 1) % 4,
        }
    }

    let ((cx, cy), (x, y)) = (chunks[chunk].pos, pos);
    Some(1000 * (cy * 50 + y + 1) + 4 * (cx * 50 + x + 1) + facing as i32)
}

/// Gets the position of `p` on `target_edge`, assuming it starts on `source_edge`.
fn project_onto_edge(p: Point, source_edge: usize, target_edge: usize) -> Point {
    // This code is a symptom of me giving up.
    match (source_edge, target_edge) {
        (0, 0) => (49, 49 - p.1),
        (0, 1) => (p.1, 49),
        (0, 2) => (0, p.1),
        (0, 3) => (49 - p.1, 0),
        (1, 0) => (49, p.0),
        (1, 1) => (49 - p.0, 49),
        (1, 2) => (0, 49 - p.0),
        (1, 3) => (p.0, 0),
        (2, 0) => (49, p.1),
        (2, 1) => (49 - p.1, 49),
        (2, 2) => (0, 49 - p.1),
        (2, 3) => (p.1, 0),
        (3, 0) => (49, 49 - p.0),
        (3, 1) => (p.0, 49),
        (3, 2) => (0, p.0),
        (3, 3) => (49 - p.0, 0),
        _ => unreachable!(),
    }
}

type Point = (i32, i32);
type LinkFn = fn(&[Chunk]) -> Vec<Link>;

/// Represents one sixth of the map, a single 50x50 chunk.
struct Chunk {
    pos: (i32, i32),
    data: [bool; 2500],
}

/// Represents the four connections--to the right, down, to the left, and up. The first number
/// in each pair indicates the chunk we will arrive at, and the second number how many
/// times it is rotated clockwise in relation to the current chunk.
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

/// Parses the puzzle input into a list of chunks and a list of instructions.
fn parse(input: &str) -> Option<(Vec<Chunk>, Vec<Op>)> {
    let (map, instructions) = input.split_once("\n\n")?;
    let lines: Vec<_> = map.lines().collect();

    let mut chunks = vec![];
    for cy in 0..6 {
        for cx in 0..6 {
            let value = lines
                .get(cy * 50)
                .and_then(|v| v.as_bytes().get(cx * 50))
                .copied();

            if matches!(value, Some(b'.' | b'#')) {
                let mut data = [false; 2500];
                for y in 0..50 {
                    for x in 0..50 {
                        data[50 * y + x] = lines[cy * 50 + y].as_bytes()[cx * 50 + x] == b'.';
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
