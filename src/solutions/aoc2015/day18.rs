/// Run a cellular automaton for 100 steps, count the live cells.
pub fn one(input: &str) -> Result<String, String> {
    Ok(run(&mut parse(input), |_, b, n| (b && n == 2) || n == 3).to_string())
}

pub fn two(input: &str) -> Result<String, String> {
    let mut grid = parse(input);
    let rules = |i, b, n| [0, 99, 9900, 9999].contains(&i) || (b && n == 2) || n == 3;
    for i in [0, 99, 9900, 9999] {
        grid[i] = true;
    }
    Ok(run(&mut grid, rules).to_string())
}

/// Run a cellular automaton for 100 steps, count the live cells. The corners of the field are
/// permanently alive.
fn run(grid: &mut [bool], rules: fn(usize, bool, u8) -> bool) -> usize {
    // Count the number of alive neighbours for a given cell.
    fn neighbours(grid: &[bool], i: usize) -> u8 {
        let mut indices = vec![-100, 100];
        if i % 100 != 0 {
            indices.extend([-101, -1, 99]);
        }
        if i % 100 != 99 {
            indices.extend([-99, 1, 101]);
        }

        indices
            .into_iter()
            .filter_map(|d| (d + i as i32).try_into().ok())
            .filter(|&n: &usize| *grid.get(n).unwrap_or(&false))
            .count() as u8
    }

    // Writes the next step into `target`, based on `source` and `rules`.
    fn step(source: &[bool], target: &mut [bool], rules: fn(usize, bool, u8) -> bool) {
        for i in 0..target.len() {
            target[i] = rules(i, source[i], neighbours(&source, i));
        }
    }

    let mut buffer = vec![false; grid.len()];

    for _ in 0..50 {
        step(&grid, &mut buffer, rules);
        step(&buffer, grid, rules);
    }

    grid.iter().copied().filter(|&b| b).count()
}

/// Parses the puzzle input into a grid of cellular automaton cells.
fn parse(input: &str) -> Vec<bool> {
    let mut result = vec![false; 10_000];
    for (i, c) in input.lines().flat_map(|l| l.chars()).enumerate() {
        if c == '#' {
            result[i] = true;
        }
    }
    result
}
