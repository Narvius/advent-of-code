/// Count the number of occurences of the digits 1, 4, 7, 8 in outputs.
pub fn one(input: &str) -> crate::Result<usize> {
    run(input, |(_, output)| {
        output
            .split_whitespace()
            .filter(|s| [2, 3, 4, 7].contains(&s.len()))
            .count()
    })
}

/// Identify the output numbers, and sum them.
pub fn two(input: &str) -> crate::Result<usize> {
    run(input, |(input, output)| {
        let mapping = deduce_mapping(input);
        output
            .split_whitespace()
            .filter_map(|c| digit_from_mapping(c.as_bytes(), &mapping))
            .fold(0, |a, d| 10 * a + d)
    })
}

/// Splits all input lines into (input, output) pairs, applies `f` to all of them and sums the
/// results.
fn run(input: &str, f: fn((&str, &str)) -> usize) -> crate::Result<usize> {
    let pairs = input.lines().filter_map(|line| line.split_once(" | "));
    Ok(pairs.map(f).sum())
}

/// Given the "input" portion of a line of puzzle input, returns a mapping from the scrambled wire
/// letter, to the canonical wire letter (as used in [`PATTERNS`]).
fn deduce_mapping(input: &str) -> Vec<u8> {
    let digits: Vec<_> = input.split_whitespace().map(|s| s.as_bytes()).collect();
    let mut candidates = vec![Vec::from(b"abcdefg".as_ref()); 7];

    // First pass.
    // The digits 1, 4, 7 and 8 are always uniquely identifiable by string length. 8 doesn't
    // constrain our candidate list though, so we only use 1, 4 and 7 here.
    for &digit in &digits {
        let unique_digit = match digit.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            _ => None,
        };

        if let Some(n) = unique_digit {
            // For each candidate set `C` for wire `b`: if `digit` contains `b`,
            // then `C = C & PATTERN[n]`. Otherwise, `C = C \ PATTERN[n]` (where A & B is set
            // intersection, and A \ B is set difference).
            for b in b'a'..=b'g' {
                let keep = digit.contains(&b);
                candidates[(b - b'a') as usize].retain(|c| !(keep ^ PATTERNS[n].contains(c)));
            }
        }
    }

    // Second pass.
    // By now we have so massively constrained the search space that we can just brute force the
    // rest of the way.
    for mapping in Product(candidates, 0) {
        if digits
            .iter()
            .all(|s| digit_from_mapping(s, &mapping).is_some())
        {
            return mapping;
        }
    }

    unreachable!()
}

/// Given a 7 segment string and a [`mapping`](deduce_mapping), returns the digit represented by
/// the encoded 7 segment string.
fn digit_from_mapping(digit: &[u8], mapping: &[u8]) -> Option<usize> {
    let mut wires: Vec<_> = digit.iter().map(|c| mapping[(c - b'a') as usize]).collect();
    wires.sort();
    PATTERNS.iter().position(|s| s.eq(&wires))
}

/// An iterator that returns the cartesian product of a list of lists.
struct Product<T>(Vec<Vec<T>>, usize);

impl<T: Copy> Iterator for Product<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let Product(vs, i) = self;
        let mut n = *i;
        *i += 1;

        let mut result = vec![];
        for v in vs {
            result.push(*v.get(n % v.len())?);
            n /= v.len();
        }
        Some(result)
    }
}

/// `PATTERNS[n]` is the canonical wires for digit `n`.
static PATTERNS: &[&[u8]] = &[
    b"abcefg", b"cf", b"acdeg", b"acdfg", b"bcdf", b"abdfg", b"abdefg", b"acf", b"abcdefg",
    b"abcdfg",
];
