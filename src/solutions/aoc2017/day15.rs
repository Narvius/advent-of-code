/// Count how many of the first 40 million pairs match.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(count_matches(input, 40000000, false))
}

/// Count how many of the first 5 million sieved pairs match.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(count_matches(input, 5000000, true))
}

/// Counts for how many of the first `count` generated pair the lowest 16 bits match.
fn count_matches(input: &str, count: usize, sieving: bool) -> usize {
    let factors: Vec<_> = input
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .last()
                .and_then(|s| s.parse::<usize>().ok())
        })
        .collect();

    Generator::a(factors[0], sieving)
        .zip(Generator::b(factors[1], sieving))
        .take(count)
        .filter(|&(a, b)| (a ^ b) % (1 << 16) == 0)
        .count()
}

/// An iterator that generates numbers as given in the puzzle description. It would be
/// more efficient to split up the sieving and non-sieving paths, but it doesn't really
/// matter.
struct Generator {
    factor: usize,
    last: usize,
    sieve: Option<usize>,
}

impl Generator {
    /// Creates a new generator with the given parameters.
    fn new(seed: usize, factor: usize, sieve: Option<usize>) -> Self {
        Self {
            factor,
            last: seed,
            sieve,
        }
    }

    /// Creates a generator with the parameters for generator A.
    fn a(seed: usize, sieving: bool) -> Self {
        Self::new(seed, 16807, sieving.then_some(4))
    }

    /// Creates a generator with the parameters for generator B.
    fn b(seed: usize, sieving: bool) -> Self {
        Self::new(seed, 48271, sieving.then_some(8))
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.last = (self.last * self.factor) % 2147483647;
            if self.sieve.map(|s| self.last % s == 0).unwrap_or(true) {
                break;
            }
        }
        Some(self.last)
    }
}
