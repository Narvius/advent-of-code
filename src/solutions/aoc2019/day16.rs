/// Perform 100 steps of FFT on the input, return the first eight digits.
pub fn one(input: &str) -> crate::Result<String> {
    // We simply prepend a 0 to the input, so the pattern lines up, instead of implementing
    // the "skip one" rule.
    let mut signal: Vec<_> = std::iter::once(0)
        .chain(input.trim().bytes().map(|b| (b - b'0') as i32))
        .collect();
    let mut buffer = vec![0; signal.len()];

    for _ in 0..100 {
        phase(&mut signal, &mut buffer);
        std::mem::swap(&mut signal, &mut buffer);
    }

    Ok(signal[1..]
        .iter()
        .take(8)
        .map(|&b| (b as u8 + b'0') as char)
        .collect())
}

/// Perform 100 steps of FFT on the ten thousand times expanded input, return a substring of
/// eight digits indexed by the first seven digits of the input.
pub fn two(input: &str) -> crate::Result<String> {
    // Key insight #1: When stepping, numbers are only affected by themselves or later digits.
    // Thus we can skip calculating a vast majority--we only need to calculate from the target
    // location upwards!
    let skip: usize = input[0..7].parse()?;
    let period = input.trim().len();
    let cycles = 10000 - skip / period;
    let final_skip = skip - (skip / period) * period;

    let mut signal = std::iter::repeat(input.trim().bytes().map(|b| b - b'0'))
        .take(cycles)
        .flatten()
        .skip(final_skip)
        .collect::<Vec<_>>();

    // Key insight #2: We are *way* past the middle of the full signal. Once you are on indices
    // past the middle of the signal, each cell is simply the sum of all cells after itself plus
    // itself.
    for _ in 0..100 {
        // Implemented here as going backwards, so we only need to iterate over the entire list
        // once to build the sum and assign it to every cell.
        let mut sum = 0;
        for cell in signal.iter_mut().rev() {
            sum = (sum + *cell) % 10;
            *cell = sum;
        }
    }

    Ok(signal.iter().take(8).map(|&b| (b + b'0') as char).collect())
}

/// Naive implementation of FFT as described in the puzzle.
fn phase(input: &mut [i32], output: &mut [i32]) {
    for (period, cell) in output.iter_mut().enumerate().skip(1) {
        *cell = 0;
        for i in 0..period {
            for n in 0..=input.len() / (4 * period) {
                *cell += *input.get(period + i + 4 * n * period).unwrap_or(&0)
                    - *input.get(3 * period + i + 4 * n * period).unwrap_or(&0);
            }
        }
        *cell = cell.abs() % 10;
    }
}
