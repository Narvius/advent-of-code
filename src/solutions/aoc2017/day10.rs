use std::fmt::Write;

/// Find the product of the first two numbers after one iteration of the hashing algorithm.
pub fn one(input: &str) -> Result<String, String> {
    let mut h = Hasher::new(input.split(',').filter_map(|n| n.parse::<u8>().ok()));
    h.run_once();
    let hash = h.sparse_hash();
    Ok((hash[0] as usize * hash[1] as usize).to_string())
}

/// Find the actual knot hash, as per puzzle description. In short: run 64 rounds of hashing,
/// find the "dense hash", and write it as a hexadecimal string. Also uses an alternate
/// interpretation of the puzzle input.
pub fn two(input: &str) -> Result<String, String> {
    let mut s = String::with_capacity(32);
    for byte in knot_hash(input.bytes()) {
        write!(s, "{:02x}", byte).expect("failed to write to preallocated string");
    }
    Ok(s)
}

/// Calculates the knot hash for a given sequence of bytes.
pub fn knot_hash(bytes: impl IntoIterator<Item = u8>) -> Vec<u8> {
    let mut hasher = Hasher::new(bytes.into_iter().chain([17, 31, 73, 47, 23]));
    for _ in 0..64 {
        hasher.run_once();
    }
    hasher.dense_hash().collect()
}

/// Holds the state for knot hashing.
struct Hasher {
    /// The array that is being hashed on.
    array: Vec<u8>,
    /// Skip size from the puzzle.
    skip: usize,
    /// In this implementation, `array` is always rotated so that 0 corresponds to the
    /// current cursor position from the puzzle. This stores the difference between the current
    /// and original rotation of `array`.
    offset: usize,
    /// Hashing algorithm input.
    lengths: Vec<u8>,
}

impl Hasher {
    /// Creates a new hasher that uses the provided sequence of reversing lengths for hashing.
    fn new(lengths: impl IntoIterator<Item = u8>) -> Self {
        Self {
            array: (0..=255).collect(),
            skip: 0,
            offset: 0,
            lengths: lengths.into_iter().collect(),
        }
    }

    /// Runs one iteration of the hashing algorithm.
    fn run_once(&mut self) {
        let len = self.array.len();
        for &length in &self.lengths {
            if length > 0 {
                self.array[0..length as usize].reverse();
            }
            self.array.rotate_left((self.skip + length as usize) % len);
            self.offset = (self.offset + self.skip + length as usize) % len;
            self.skip += 1;
        }
    }

    /// Returns the sparse hash produced so far.
    fn sparse_hash(&mut self) -> &[u8] {
        if self.offset > 0 {
            self.array.rotate_right(self.offset);
            self.offset = 0;
        }

        self.array.as_ref()
    }

    /// Returns the "dense hash" for the current state, as described in the puzzle.
    fn dense_hash(&mut self) -> impl Iterator<Item = u8> + '_ {
        self.sparse_hash()
            .chunks(16)
            .map(|chunk| chunk.iter().fold(0u8, |a, &b| a ^ b))
    }
}
