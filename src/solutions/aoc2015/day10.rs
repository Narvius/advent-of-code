/// Performs 40 steps of the look-and-say sequence and gets the final length.
pub fn one(input: &str) -> crate::Result<usize> {
    let (mut v1, mut v2) = (input.bytes().map(|b| b - b'0').collect(), vec![]);
    for _ in 0..20 {
        look_and_say(&mut v1, &mut v2);
        look_and_say(&mut v2, &mut v1);
    }
    Ok(v1.len())
}

/// Performs 50 steps of the look-and-say sequence and gets the final length.
pub fn two(input: &str) -> crate::Result<usize> {
    let (mut v1, mut v2) = (input.bytes().map(|b| b - b'0').collect(), vec![]);
    for _ in 0..25 {
        look_and_say(&mut v1, &mut v2);
        look_and_say(&mut v2, &mut v1);
    }
    Ok(v1.len())
}

/// Performs one step of the look-and-say sequence, removing elements from `source`, and writing
/// them to `target`.
fn look_and_say(source: &mut Vec<u8>, target: &mut Vec<u8>) {
    if source.is_empty() {
        return;
    }

    let (mut run, mut last) = (1, source[0]);
    for v in source.drain(..).skip(1) {
        if v == last {
            run += 1;
        } else {
            target.extend([run, last]);
            (run, last) = (1, v);
        }
    }
    target.extend([run, last]);
}
