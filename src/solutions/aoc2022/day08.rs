use std::cmp::Ordering;

/// Count the number of trees that are (visible)[`visible`] from outside the grid.
pub fn one(input: &str) -> crate::Result<usize> {
    let map = parse(input);
    Ok((0..map.len())
        .flat_map(|y| (0..map[y].len()).map(move |x| (x, y)))
        .filter(|&p| visible(&map, p))
        .count())
}

/// Find highest (scenic score)[`scenic_score`] of any tree on the map.
pub fn two(input: &str) -> crate::Result<i32> {
    let map = parse(input);
    (0..map.len())
        .flat_map(|y| (0..map[y].len()).map(move |x| (x, y)))
        .map(|p| scenic_score(&map, p))
        .max()
        .ok_or_else(|| "no result".into())
}

type Map = Vec<Vec<u8>>;

/// Checks if the tree at the given position is visible from the outside.
fn visible(map: &Map, (x, y): (usize, usize)) -> bool {
    (0..x).all(|vx| map[y][vx] < map[y][x])
        || ((x + 1)..map[y].len()).all(|vx| map[y][vx] < map[y][x])
        || (0..y).all(|vy| map[vy][x] < map[y][x])
        || ((y + 1)..map.len()).all(|vy| map[vy][x] < map[y][x])
}

/// Calculates the scenic score for a given tree, that is, the product of the number of trees
/// seen in each cardinal direction.
fn scenic_score(map: &Map, p: (usize, usize)) -> i32 {
    // Counts the number of trees seen in the given direction.
    fn scan(map: &Map, (x, y): (usize, usize), (dx, dy): (i32, i32)) -> i32 {
        let mut count = 0;
        let (w, h) = (map[0].len() as i32, map.len() as i32);
        let max = map[y][x];
        for n in 1.. {
            let (x, y) = (x as i32 + dx * n, y as i32 + dy * n);
            if !(0..w).contains(&x) || !(0..h).contains(&y) {
                break;
            }

            match map[y as usize][x as usize].cmp(&max) {
                Ordering::Less => count += 1,
                Ordering::Equal => {
                    count += 1;
                    break;
                }
                Ordering::Greater => break,
            }
        }
        count
    }

    [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .map(|d| scan(map, p, d))
        .into_iter()
        .product()
}

/// Parses the puzzle input into a 2D map of tree heights.
fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}
