/// Find the order after the dance.
pub fn one(input: &str) -> crate::Result<String> {
    stringize(dance(input, false)?)
}

/// Find the order after one billion iterations of the dance.
///
/// For performance reason, we are of course not doing one billion dances. Instead, we have
/// three observations:
/// - the "named swap" (p) instructions can be ignored, as they will cancel out at an even
///   amount of dances;
/// - if we ignore (p), and call the result from calling [`dance`] once "`mapping`", then
///   `mapping[i]` is the position of the `i`th character after a second dance. Thus, we can
///   stack dances without actually running [`dance`] again.
/// - if we use a "stacked" dance as `mapping`, then we can apply that many dances at once.
/// From there, it's simply a matter of applying the dance 10 times to reach a power of 10,
/// use that as a new `mapping` to apply 10 times more dances per iteration; and do that
/// entire thing 9 times to reach 10 billion iterations.
pub fn two(input: &str) -> crate::Result<String> {
    let mut mapping = dance(input, true)?;

    for _ in 0..9 {
        let mut order: Vec<u8> = (0..16).collect();
        let mut prev = vec![0; 16];

        for _ in 0..10 {
            std::mem::swap(&mut order, &mut prev);
            for i in 0..order.len() {
                order[i] = prev[mapping[i] as usize];
            }
        }

        mapping = order;
    }

    stringize(mapping)
}

/// Converts an ordering into the format expected of the answer.
fn stringize(order: Vec<u8>) -> crate::Result<String> {
    String::from_utf8(order.into_iter().map(|c| c + b'a').collect())
        .map_err(|_| "produced invalid string".into())
}

/// Performs the dance from the input, returning the resulting scrambled ordering. See
/// [`two`] for more information on the `ignore_p` parameter.
fn dance(input: &str, ignore_p: bool) -> Result<Vec<u8>, String> {
    let mut order: Vec<_> = (0..16).collect();
    for step in input.split(',') {
        match &step[0..1] {
            "s" => {
                let size = step[1..].parse().map_err(|_| "failed parse".to_owned())?;
                order.rotate_right(size);
            }
            "x" => {
                if let Some((l, r)) = step[1..].split_once('/') {
                    let l = l.parse().map_err(|_| "failed parse".to_owned())?;
                    let r = r.parse().map_err(|_| "failed parse".to_owned())?;
                    order.swap(l, r);
                }
            }
            "p" if !ignore_p => {
                if let Some((l, r)) = step[1..].split_once('/') {
                    let (l, r) = (l.as_bytes()[0] - b'a', r.as_bytes()[0] - b'a');
                    for b in &mut order {
                        *b = match *b {
                            b if b == l => r,
                            b if b == r => l,
                            b => b,
                        };
                    }
                }
            }
            _ => {}
        }
    }
    Ok(order)
}
