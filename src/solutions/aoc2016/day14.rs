use std::collections::HashMap;

/// Finds the 64th key index.
pub fn one(input: &str) -> Result<String, String> {
    Ok(find_hash(input, find_64th_key(false)).to_string())
}

/// Finds the 64th key index with hash stretching employed.
pub fn two(input: &str) -> Result<String, String> {
    Ok(find_hash(input, find_64th_key(true)).to_string())
}

/// Returns a predicate for [`find_hash`] which finds the index of the 64th key according to the
/// puzzle description.
fn find_64th_key(stretch: bool) -> impl FnMut(usize, md5::Digest) -> Option<usize> {
    let mut count = 0;
    let mut candidates: HashMap<u8, Vec<usize>> = HashMap::new();

    move |i, d| {
        let mut s = format!("{:x}", d);
        if stretch {
            for _ in 0..2016 {
                s = format!("{:x}", md5::compute(s.as_bytes()));
            }
        }

        for c in find_runs(s.as_bytes(), 5) {
            for ci in candidates.entry(c).or_default().drain(..) {
                if i <= ci + 1000 {
                    count += 1;
                    if count == 64 {
                        return Some(ci);
                    }
                }
            }
        }

        if let Some(c) = find_runs(s.as_bytes(), 3).next() {
            candidates.entry(c).or_default().push(i);
        }

        None
    }
}

/// Finds the lowest number for which `md5(input + that number)` returns `Some(x)`, and returns
/// that `x`. This is a slightly-modified version of the identically-named function from 2015-06.
fn find_hash<T>(input: &str, mut check: impl FnMut(usize, md5::Digest) -> Option<T>) -> T {
    let mut cache = vec![md5::Context::new()];
    cache[0].consume(input.as_bytes());

    let mut i = 1;
    loop {
        let mut context = cache[i / 10].clone();
        context.consume([b'0' + (i % 10) as u8]);
        cache.push(context.clone());

        if let Some(result) = check(i, context.compute()) {
            return result;
        }

        i += 1;
    }
}

/// Finds all characters that appear `length` times in a row; in the order that these runs appear
/// in.
fn find_runs(s: &[u8], length: usize) -> impl Iterator<Item = u8> + '_ {
    s.windows(length)
        .filter(|w| w.iter().all(|&c| w[0] == c))
        .map(|w| w[0])
}
