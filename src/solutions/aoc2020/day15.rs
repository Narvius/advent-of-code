/// Find the 2020th number in the memory game.
pub fn one(input: &str) -> crate::Result<usize> {
    run_memory_game(input.trim(), 2020)
}

/// Find the thirty millionth number in the memory game.
pub fn two(input: &str) -> crate::Result<usize> {
    run_memory_game(input.trim(), 30_000_000)
}

/// Runs the memory game as described in the puzzle, returning the `target`th number.
fn run_memory_game(input: &str, target: usize) -> crate::Result<usize> {
    let mut vals = input.split(',').filter_map(|v| v.parse::<usize>().ok());

    // `occurences[n]` correspond to the turn that `n` last showed up on. The size is
    // probably too large, but finding a more accurate  size is too much effort to be worth it.
    let mut occurences = vec![None; target];
    // `item` is the last number that was called.
    // `occurence` is the turn that `item` last showed up on.
    let (mut item, mut occurence) = (0, None);

    for turn in 0..target {
        // Take inputs as long as there are any; otherwise use the calculation described.
        item = vals
            .next()
            .unwrap_or_else(|| occurence.map(|n| turn - n - 1).unwrap_or(0));
        occurence = occurences[item].replace(turn);
    }

    Ok(item)
}
