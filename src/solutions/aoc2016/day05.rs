// Running this solution takes about 73 seconds on my machine. I'm glad to be past this for now,
// and I really don't like the "puzzles" that are just about calculating md5 real good, but I might
// try to find a more efficient way in the future. There probably is a trick to this.

/// Find eight hashes starting with five zeroes, and construct the password from their sixth
/// bytes.
pub fn one(input: &str) -> crate::Result<String> {
    Ok(Hashes::new(input)
        .take(8)
        .map(|(c, _)| format!("{:x}", c))
        .collect())
}

/// Construct the password in a more complicated fashion using hashes starting with five zeroes.
/// See puzzle description for more details.
pub fn two(input: &str) -> crate::Result<String> {
    let mut cs = vec![None; 8];
    let mut found = 0;
    for (i, c) in Hashes::new(input) {
        if i < 8 && cs[i] == None {
            cs[i] = Some(format!("{:x}", c));
            found += 1;
            if found == 8 {
                return cs
                    .into_iter()
                    .collect::<Option<String>>()
                    .ok_or_else(|| "failed to construct output string".into());
            }
        }
    }
    Err("ran out of hashes".into())
}

/// An iterator that, for a given seed, calculates `md5(seed + i)`, where i as a number starting at
/// 10 and growing each attempt; returning the sixth and seventh byte of the hash if it starts with
/// five zeroes.
struct Hashes {
    cache: Vec<md5::Context>,
    i: usize,
}

impl Hashes {
    fn new(seed: &str) -> Hashes {
        Hashes {
            cache: {
                let mut cache = vec![md5::Context::new()];
                cache[0].consume(seed.as_bytes());
                cache
            },
            i: 0,
        }
    }
}

impl Iterator for Hashes {
    type Item = (usize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.i += 1;

            let mut context = self.cache[self.i / 10].clone();
            context.consume([b'0' + (self.i % 10) as u8]);
            self.cache.push(context.clone());
            let d = context.compute();

            if d[0] == 0 && d[1] == 0 && d[2] < 16 {
                return Some((d[2] as usize, (d[3] >> 4) as u8));
            }
        }
    }
}
