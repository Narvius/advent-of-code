/// Scramble a given password.
pub fn one(input: &str) -> Result<String, String> {
    let mut s = b"abcdefgh".to_vec();
    scramble(s.as_mut(), input.lines(), false);
    Ok(String::from_utf8(s).unwrap())
}

/// Unscramble a given password.
pub fn two(input: &str) -> Result<String, String> {
    let mut s = b"fbgdceah".to_vec();
    scramble(s.as_mut(), input.lines().rev(), true);
    Ok(String::from_utf8(s).unwrap())
}

/// The amount of right shifts required to scramble a password, given by the formula
/// `(1 + i + (i >= 4) as usize) % 8`. This maps
const ROTMAP: [usize; 8] = [1, 2, 3, 4, 6, 7, 0, 1];
/// The mapping `ROTMAP` logically inverted, for unscrambling a password. It can be derived by
/// noting that `i + ROTMAP[i]` is the position `k` you will be at; so it is merely a matter of
/// finding an `f(k)` such that it gives the amount of left shifts required to cancel out the
/// right shift by ROTMAP\[i\].
const REV_ROTMAP: [usize; 8] = [1, 1, 6, 2, 7, 3, 0, 4];

/// Applies the scrambling operations as described in the puzzle description to a given string
/// (as slice of bytes). If `reversed` is set, the operations will be ran in unscrambling mode
/// instead; scrambling then unscrambling would be effectively a no-op.
fn scramble<'a, I>(text: &mut [u8], instructions: I, reversed: bool)
where
    I: Iterator<Item = &'a str>,
{
    for line in instructions {
        let tokens: Vec<_> = line.split(' ').collect();
        match *tokens.as_slice() {
            // Swapping remains the same in either mode.
            ["swap", "position", x, "with", "position", y] => {
                text.swap(x.parse().unwrap(), y.parse().unwrap());
            }
            ["swap", "letter", x, "with", "letter", y] => {
                for i in 0..text.len() {
                    if text[i] == x.as_bytes()[0] {
                        text[i] = y.as_bytes()[0];
                    } else if text[i] == y.as_bytes()[0] {
                        text[i] = x.as_bytes()[0]
                    }
                }
            }
            // When `reversed`, just rotate the opposite direction.
            ["rotate", dir, x, _step] => {
                if (dir == "right") ^ reversed {
                    text.rotate_right(x.parse().unwrap());
                } else {
                    text.rotate_left(x.parse().unwrap());
                }
            }
            // When `reversed`, rotate left by the amount of letters such that the only possible
            // original configuration is restored.
            ["rotate", "based", "on", "position", "of", "letter", x] => {
                for i in 0..text.len() {
                    if text[i] == x.as_bytes()[0] {
                        if !reversed {
                            text.rotate_right(ROTMAP[i]);
                        } else {
                            text.rotate_left(REV_ROTMAP[i]);
                        }
                        break;
                    }
                }
            }
            // Reversing remains the same in either mode.
            ["reverse", "positions", x, "through", y] => {
                text[x.parse().unwrap()..=y.parse().unwrap()].reverse();
            }
            // Since moving between positions is implemented as rotating a sub-slice, simply
            // rotate the opposite direction when `reversed`.
            ["move", "position", x, "to", "position", y] => {
                let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                match (x < y, reversed) {
                    (true, false) => text[x..=y].rotate_left(1),
                    (true, true) => text[x..=y].rotate_right(1),
                    (false, false) => text[y..=x].rotate_right(1),
                    (false, true) => text[y..=x].rotate_left(1),
                }
            }
            _ => {}
        }
    }
}
