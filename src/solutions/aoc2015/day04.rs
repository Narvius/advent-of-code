/// Find an md5 hash with five leading zeroes.
pub fn one(input: &str) -> Result<String, String> {
    Ok(find_hash(input, |d| d[0] == 0 && d[1] == 0 && d[2] < 16).to_string())
}

/// Find an md5 hash with six leading zeroes.
pub fn two(input: &str) -> Result<String, String> {
    Ok(find_hash(input, |d| d[0] == 0 && d[1] == 0 && d[2] == 0).to_string())
}

/// Finds the lowest number for which `md5(input + that number)` passes the `check`.
fn find_hash(input: &str, check: fn(md5::Digest) -> bool) -> usize {
    let mut cache = vec![md5::Context::new()];
    cache[0].consume(input.as_bytes());

    for i in 1..10 {
        cache.push(cache[0].clone());
        cache[i].consume([b'0' + i as u8]);
    }

    let mut i = 10;
    loop {
        let mut context = cache[i / 10].clone();
        context.consume([b'0' + (i % 10) as u8]);
        cache.push(context.clone());

        if check(context.compute()) {
            return i;
        }

        i += 1;
    }
}
