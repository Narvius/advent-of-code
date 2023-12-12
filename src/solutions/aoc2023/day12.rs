use std::{borrow::Cow, collections::HashMap};

/// Find the number of possible group arrangements.
pub fn one(input: &str) -> crate::Result<usize> {
    run(input, false)
}

/// Find the number of possible group arrangements with expanded data.
pub fn two(input: &str) -> crate::Result<usize> {
    run(input, true)
}

type Cache = HashMap<(usize, usize, usize), usize>;

/// Shared code for both parts. `extended` decides if the input is expanded as per part 2.
fn run(input: &str, extended: bool) -> crate::Result<usize> {
    Ok(input
        .lines()
        .filter_map(|line| {
            let mut cache = HashMap::new();
            let (pattern, groups) = line.split_once(' ')?;

            let pattern = match extended {
                false => Cow::Borrowed(pattern),
                true => Cow::Owned({
                    let mut s = String::with_capacity(pattern.len() * 5 + 4);
                    s.push_str(pattern);
                    for _ in 1..5 {
                        s.push('?');
                        s.push_str(pattern);
                    }
                    s
                }),
            };

            let mut groups: Vec<usize> = groups.split(',').filter_map(|n| n.parse().ok()).collect();
            if extended {
                groups.reserve(groups.len() * 4);
                let len = groups.len();
                for _ in 1..5 {
                    for i in 0..len {
                        groups.push(groups[i]);
                    }
                }
            }

            Some(arrangements(pattern.as_bytes(), 0, &groups, &mut cache))
        })
        .sum())
}

/// Counts the number of possible arrangements of `groups` in the provided `s`tring, assuming we
/// previously parsed `run` consecutive #s. `cache` memoizes previous results, indexed by
/// (remaining string length, run length, remaining group count).
fn arrangements(s: &[u8], run: usize, groups: &[usize], cache: &mut Cache) -> usize {
    let cache_key = (s.len(), run, groups.len());

    if s.is_empty() {
        // String is empty. The current run must empty or equal to the last remaining group.
        usize::from((run == 0 && groups.is_empty()) || groups == [run])
    } else if run > *groups.first().unwrap_or(&0) {
        // Our current run is too large for the current group. The whole branch can be discarded.
        0
    } else if let Some(&cached_result) = cache.get(&cache_key) {
        cached_result
    } else {
        let count = match (s[0], run) {
            (b'.', 0) => arrangements(&s[1..], 0, groups, cache),
            (b'.', n) => match n == groups[0] {
                true => arrangements(&s[1..], 0, &groups[1..], cache),
                false => 0,
            },

            (b'#', n) => arrangements(&s[1..], n + 1, groups, cache),

            // If we're not in a run, a ? could be either. Just count both options.
            (b'?', 0) => {
                arrangements(&s[1..], 1, groups, cache) + arrangements(&s[1..], 0, groups, cache)
            }
            // Already in a run. Whether we assume a # or . fully depends on the current group.
            (b'?', n) => match n == groups[0] {
                true => arrangements(&s[1..], 0, &groups[1..], cache),
                false => arrangements(&s[1..], n + 1, groups, cache),
            },

            _ => unreachable!(),
        };

        cache.insert(cache_key, count);
        count
    }
}
