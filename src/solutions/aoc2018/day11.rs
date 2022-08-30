/// Find the optimal 3x3 square in the grid.
pub fn one(input: &str) -> crate::Result<String> {
    let (x, y, _) = pick_optimal_square(input.parse()?, Some(3));
    Ok(format!("{},{}", x, y))
}

/// Find the optimal square in the grid.
pub fn two(input: &str) -> crate::Result<String> {
    let (x, y, size) = pick_optimal_square(input.parse()?, None);
    Ok(format!("{},{},{}", x, y, size))
}

/// Returns the optimal square based on a grid serial number, as per the puzzle description.
/// If `fixed_size` is provided, only squares of that size are searched.
fn pick_optimal_square(seed: i32, fixed_size: Option<usize>) -> (usize, usize, usize) {
    const SIZE: usize = 300;

    let sums = make_summed_area_table(seed, SIZE);
    let sums: Vec<_> = sums.as_slice().chunks(SIZE + 1).collect();

    let sizes = if let Some(size) = fixed_size {
        size..=size
    } else {
        1..=SIZE
    };

    let (mut best, mut x, mut y, mut s) = (0, 0, 0, 0);

    for size in sizes {
        for cy in size..=SIZE {
            for cx in size..=SIZE {
                let sum = sums[cy][cx] + sums[cy - size][cx - size] - sums[cy - size][cx] - sums[cy][cx - size];
                if sum > best {
                    (best, x, y, s) = (sum, cx - size + 1, cy - size + 1, size);
                }
            }
        }
    }

    (x, y, s)
}

/// Generates the power cell grid as described in the puzzle, and creates a
/// [summed area table](https://en.wikipedia.org/wiki/Summed-area_table) for it.
/// Note that the returned table is one larger in either direction than `size`.
fn make_summed_area_table(seed: i32, size: usize) -> Vec<i32> {
    let mut sums = vec![0; (size + 1).pow(2)];
    let mut view: Vec<_> = sums.as_mut_slice().chunks_mut(size + 1).collect();

    for y in 1..=size {
        for x in 1..=size {
            let rack_id = x as i32 + 10;
            let v =(((rack_id * y as i32 + seed) * rack_id) / 100) % 10 - 5;
            view[y][x] = v + view[y - 1][x] + view[y][x - 1] - view[y - 1][x - 1];
        }
    }

    sums
}
