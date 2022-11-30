/// Find the encryption key for the given pair of public keys.
pub fn one(input: &str) -> crate::Result<i32> {
    let mut lines = input.lines();
    let card_key = lines.next().ok_or("insufficient input")?.parse()?;
    let door_key = lines.next().ok_or("insufficient input")?.parse()?;

    Ok(modular_exponentiation(
        door_key,
        discrete_logarithm(7, card_key, MODULO_FACTOR),
        MODULO_FACTOR,
    ))
}

/// Freebie!
pub fn two(_input: &str) -> crate::Result<&str> {
    Ok("done!")
}

/// Modulo factor given by the puzzle.
const MODULO_FACTOR: i32 = 20201227;

/// Finds the smallest number `n` that satisfies `a^n == 0 (mod m)`, using trial multiplication.
fn discrete_logarithm(a: i32, b: i32, m: i32) -> i32 {
    let mut i = 0;
    let mut k = 1i64;
    while k != b as i64 {
        k = (k * a as i64) % (m as i64);
        i += 1;
    }
    i
}

/// Calculates `x^k (mod m)`, using square-and-multiply.
fn modular_exponentiation(x: i32, k: i32, m: i32) -> i32 {
    let mut r = 1i64;
    let bits = (k as f64).log2().ceil() as i32;

    for i in (0..bits).rev() {
        r = (r * r) % (m as i64);
        if (k & 1 << i) > 0 {
            r = (r * x as i64) % (m as i64);
        }
    }

    r as i32
}
