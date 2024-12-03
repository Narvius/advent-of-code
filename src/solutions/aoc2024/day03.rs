/// Find the sum of all `mul` instructions.
pub fn one(input: &str) -> crate::Result<i32> {
    sum_muls(input, false)
}

/// Find the sum of all enabled `mul` instructions.
pub fn two(input: &str) -> crate::Result<i32> {
    sum_muls(input, true)
}

/// Sums the results of all `mul` instructions in the input. If `consider_donts` is true, also
/// considers `do()` and `don't()` instructions, enabling and disabling upcoming `mul`s
/// respectively.
fn sum_muls(mut input: &str, consider_donts: bool) -> crate::Result<i32> {
    let mut sum = 0;
    let mut enabled = true;
    while !input.is_empty() {
        if input.starts_with("do()") {
            enabled = true;
            input = &input["do()".len()..];
        } else if consider_donts && input.starts_with("don't()") {
            enabled = false;
            input = &input["don't()".len()..];
        } else if let (Some((a, b)), rest) = parse_mul(input, enabled) {
            sum += a * b;
            input = rest;
        } else {
            input = &input[1..];
        }
    }
    Ok(sum)
}

/// Parse a leading token of the form "mul(`a`, `b`)", where `a` and `b` are integers; returns the
/// parsed pair (if any), and the unparsed remainder of the input string.
///
/// If `enabled` is false, always fails, returning no numbers alongside the full input string. I
/// would have preferred to write "if enabled && let (...) = parse_mul(input)" at the callsite, but
/// unfortunately let-matching in conditionals like that isn't in stable Rust as of time of writing.
fn parse_mul(input: &str, enabled: bool) -> (Option<(i32, i32)>, &str) {
    if !enabled || !input.starts_with("mul(") {
        return (None, input);
    }

    let rest = &input["mul(".len()..];
    let a_len = rest.chars().take_while(char::is_ascii_digit).count();
    let (a, rest) = rest.split_at(a_len);
    if let Some(rest) = rest.strip_prefix(',') {
        let b_len = rest.chars().take_while(char::is_ascii_digit).count();
        let (b, rest) = rest.split_at(b_len);
        if let Some(rest) = rest.strip_prefix(')') {
            return (
                Some((a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())),
                rest,
            );
        }
    }
    (None, input)
}
