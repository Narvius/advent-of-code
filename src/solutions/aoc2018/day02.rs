use std::collections::HashMap;

/// Find the number of lines that have a letter pair and the number of lines that have a letter
/// triple and multiply them together.
pub fn one(input: &str) -> crate::Result<i32> {
    let (mut twos, mut threes) = (0, 0);
    for line in input.lines() {
        let mut chars: HashMap<char, i32> = HashMap::new();
        for c in line.chars() {
            *chars.entry(c).or_default() += 1;
        }
        if chars.values().any(|&count| count == 2) {
            twos += 1;
        }
        if chars.values().any(|&count| count == 3) {
            threes += 1;
        }
    }
    Ok(twos * threes)
}

/// There's one pair of input lines that differ by exactly one character. Find the sequence
/// of characters they share.
pub fn two(input: &str) -> crate::Result<String> {
    let mut one = input.lines();
    while let Some(s1) = one.next() {
        for s2 in one.clone().skip(1) {
            if s1.bytes().zip(s2.bytes()).filter(|(a, b)| a == b).count() == s2.len() - 1 {
                let mut s = String::with_capacity(s2.len() - 1);
                for (c1, c2) in s1.chars().zip(s2.chars()) {
                    if c1 == c2 {
                        s.push(c1);
                    }
                }
                return Ok(s);
            }
        }
    }

    Err("no match found".into())
}
