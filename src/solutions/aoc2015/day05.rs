use std::collections::HashSet;

/// Counts how many strings are nice, according to some arbitrary rules.
pub fn one(input: &str) -> Result<String, String> {
    const BADS: [&[u8]; 4] = [b"ab", b"cd", b"pq", b"xy"];

    fn nice(line: &&str) -> bool {
        let vowels = line.chars().filter(|c| "aiueo".contains(*c)).count() >= 3;
        let pair = line.as_bytes().windows(2).any(|w| w[0] == w[1]);
        let no_bad = line.as_bytes().windows(2).all(|w| !BADS.contains(&w));

        vowels && pair && no_bad
    }

    Ok(input.lines().filter(nice).count().to_string())
}

/// Counts how many strings are nice, according to some other arbitrary rules.
pub fn two(input: &str) -> Result<String, String> {
    fn nice(line: &&str) -> bool {
        let double_pair = {
            let mut found = HashSet::new();
            let mut last = None;
            let mut result = false;

            for w in line.as_bytes().windows(2).map(|w| (w[0], w[1])) {
                if last.take().map(|l| l != w).unwrap_or(true) {
                    if found.contains(&w) {
                        result = true;
                        break;
                    } else {
                        found.insert(w);
                        last = Some(w);
                    }
                }
            }

            result
        };
        let bracket = line.as_bytes().windows(3).any(|w| w[0] == w[2]);

        double_pair && bracket
    }

    Ok(input.lines().filter(nice).count().to_string())
}
