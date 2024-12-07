/// Find the sum of the values of all expressions that can be formed using only addition and
/// multiplication.
pub fn one(input: &str) -> crate::Result<i64> {
    sum_formable_expressions(input, false)
}

/// Find the sum of the values of all expressions that can be formed using only addition,
/// multiplication, and concatenation.
pub fn two(input: &str) -> crate::Result<i64> {
    sum_formable_expressions(input, true)
}

/// Sums all formable expressions in the input. Uses only addition and multiplication when
/// `with_concat` is false; also uses concatenation when it is `true`.
fn sum_formable_expressions(input: &str, with_concat: bool) -> crate::Result<i64> {
    let step = if with_concat { step_with_concat } else { step };
    let can_produce = |(total, numbers): &(i64, Vec<i64>)| {
        numbers
            .iter()
            .skip(1)
            .fold(vec![numbers[0]], step)
            .iter()
            .any(|x| x == total)
    };

    Ok(parse(input).filter(can_produce).map(|p| p.0).sum())
}

/// Given a list of partial results and the next number in the sequence, produces all possible
/// partial results after incorporating this next number; such that after calling this with the
/// entire sequence we end up with a list of ALL possible results for all combinations of
/// operators.
fn step(mut options: Vec<i64>, next: &i64) -> Vec<i64> {
    options.reserve(options.len());
    for i in 0..options.len() {
        options.push(options[i] * next);
        options[i] += next;
    }
    options
}

/// Given a list of partial results and the next number in the sequences, produces all possible
/// partial results after incorporating this next number. Compared to [`step`], also considers
/// concatenation.
fn step_with_concat(mut options: Vec<i64>, next: &i64) -> Vec<i64> {
    options.reserve(options.len() * 2);
    for i in 0..options.len() {
        options.push(options[i] * next);
        options.push(options[i] * 10i64.pow((f64::log10(*next as f64) + 1.0) as u32) + next);
        options[i] += next;
    }
    options
}

/// Parses the puzzle input into (expected result, numbers in expression) pairs.
fn parse(input: &str) -> impl Iterator<Item = (i64, Vec<i64>)> + '_ {
    input.lines().filter_map(|line| {
        let (total, numbers) = line.split_once(": ")?;
        Some((
            total.parse().ok()?,
            numbers
                .split_ascii_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect(),
        ))
    })
}
