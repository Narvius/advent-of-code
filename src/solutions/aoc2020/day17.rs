/// Run a 3D cellular automaton for six cycles; count the live cells.
pub fn one(input: &str) -> crate::Result<usize> {
    fn neighbours(data: &Space3, [x, y, z]: [usize; 3]) -> usize {
        product(product(-1..=1, -1..=1), -1..=1)
            .filter(|&p| p != ((0, 0), 0))
            .filter_map(|((dx, dy), dz)| {
                data.get((x as i32 + dx) as usize)?
                    .get((y as i32 + dy) as usize)?
                    .get((z as i32 + dz) as usize)?
                    .then_some(())
            })
            .count()
    }

    let (width, height, ps) = parse(input);
    let [sx, sy, sz] = [width, height, 1].map(|i| i + 2 * CYCLES);
    let mut space = vec![vec![vec![false; sz]; sy]; sx];
    let mut buffer = space.clone();

    for (x, y) in ps {
        space[CYCLES + x][CYCLES + y][CYCLES] = true;
    }

    for _ in 0..CYCLES {
        for x in 0..space.len() {
            for y in 0..space[x].len() {
                for z in 0..space[x][y].len() {
                    buffer[x][y][z] = matches!(
                        (space[x][y][z], neighbours(&space, [x, y, z])),
                        (true, 2) | (_, 3)
                    );
                }
            }
        }
        std::mem::swap(&mut space, &mut buffer);
    }

    let cells = space.into_iter().flatten().flatten();
    Ok(cells.filter(|&b| b).count())
}

/// Run a 4D cellular automaton for six cycles; count the live cells. Note that this is
/// just a natural extension of the 3D version; just add an extra dimension everywhere.
pub fn two(input: &str) -> crate::Result<usize> {
    fn neighbours(data: &Space4, [x, y, z, w]: [usize; 4]) -> usize {
        product(product(product(-1..=1, -1..=1), -1..=1), -1..=1)
            .filter(|&p| p != (((0, 0), 0), 0))
            .filter_map(|(((dx, dy), dz), dw)| {
                data.get((x as i32 + dx) as usize)?
                    .get((y as i32 + dy) as usize)?
                    .get((z as i32 + dz) as usize)?
                    .get((w as i32 + dw) as usize)?
                    .then_some(())
            })
            .count()
    }

    let (width, height, ps) = parse(input);
    let [sx, sy, sz, sw] = [width, height, 1, 1].map(|i| i + 2 * CYCLES);
    let mut space = vec![vec![vec![vec![false; sw]; sz]; sy]; sx];
    let mut buffer = space.clone();

    for (x, y) in ps {
        space[CYCLES + x][CYCLES + y][CYCLES][CYCLES] = true;
    }

    for _ in 0..CYCLES {
        for x in 0..space.len() {
            for y in 0..space[x].len() {
                for z in 0..space[x][y].len() {
                    for w in 0..space[x][y][z].len() {
                        buffer[x][y][z][w] = matches!(
                            (space[x][y][z][w], neighbours(&space, [x, y, z, w])),
                            (true, 2) | (_, 3)
                        );
                    }
                }
            }
        }
        std::mem::swap(&mut space, &mut buffer);
    }

    let cells = space.into_iter().flatten().flatten().flatten();
    Ok(cells.filter(|&b| b).count())
}

const CYCLES: usize = 6;

type Space3 = Vec<Vec<Vec<bool>>>;
type Space4 = Vec<Vec<Vec<Vec<bool>>>>;

/// Parses the input into a list of coordinates that are initially true.
fn parse(input: &str) -> (usize, usize, impl Iterator<Item = (usize, usize)> + '_) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let cells = input.lines().enumerate().flat_map(|(y, line)| {
        line.char_indices()
            .filter_map(move |(x, c)| (c == '#').then_some((x, y)))
    });

    (width, height, cells)
}

/// Returns the carthesian product of two iterators.
fn product<A: Clone, B>(
    a: impl Iterator<Item = A>,
    b: impl Iterator<Item = B> + Clone,
) -> impl Iterator<Item = (A, B)> {
    a.flat_map(move |i| b.clone().map(move |j| (i.clone(), j)))
}
