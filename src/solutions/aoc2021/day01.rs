/// In the input file, count the number of lines that have a larger value than the preceding line.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(parse(input)?.windows(2).filter(|w| w[1] > w[0]).count())
}

/// Consider all possible width-3 windows in the input file. Count the number of windows that have
/// a larger sum than the window starting one line prior.
pub fn two(input: &str) -> crate::Result<usize> {
    let sums: Vec<i32> = parse(input)?.windows(3).map(|w| w.iter().sum()).collect();
    Ok(sums.windows(2).filter(|w| w[1] > w[0]).count())
}

/// Parses the puzzle input into a list of numbers.
fn parse(input: &str) -> crate::Result<Vec<i32>> {
    Ok(input.lines().map(|s| s.parse()).collect::<Result<_, _>>()?)
}
