/// Find the sum of all `mul` instructions.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(sum_muls(input))
}

/// Find the sum of all enabled `mul` instructions.
pub fn two(input: &str) -> crate::Result<i32> {
    // Find all the stretches of input data during which `mul`s are enabled.
    let (leading_enabled, offs) = input.split_once("don't()").unwrap();
    let enabled_parts = offs
        .split("don't()")
        .filter_map(|part| Some(part.split_once("do()")?.1));

    // Sum across all those parts.
    Ok(std::iter::once(leading_enabled)
        .chain(enabled_parts)
        .map(sum_muls)
        .sum::<i32>())
}

/// Sums the results of all valid `mul` instructions within `input`. Does not consider `do()` or
/// `don't()` instructions at all and simply assumes all contained `mul`s are enabled.
fn sum_muls(input: &str) -> i32 {
    input
        .split("mul(")
        .filter_map(|t| {
            let (t, _) = t.split_once(')')?;
            let (a, b) = t.split_once(',')?;
            let (a, b) = (a.parse::<i32>().ok()?, b.parse::<i32>().ok()?);
            Some(a * b)
        })
        .sum()
}
