// Predict the next value for each line; find the sum.
pub fn one(input: &str) -> crate::Result<i64> {
    Ok(parse(input)
        .filter_map(|mut series| predict(&mut series))
        .sum())
}

// Extrapolate the previous value for each line; find the sum.
pub fn two(input: &str) -> crate::Result<i64> {
    Ok(parse(input)
        .filter_map(|mut series| extrapolate(&mut series))
        .sum())
}

/// Predicts the next value for a given series.
///
/// If we look at the example provided:
///
/// ```text
/// 10  13  16  21  30  45 -> 68
///    3   3   5   9  15 -> 23
///      0   2   4   6 -> 8
///        2   2   2 -> 2
///          0   0 -> 0
/// ```
///
/// Notice that the final result (68) is equal to 45 + 15 + 6 + 2, ie. the last value of each level
/// of summed. This is no coincidence; it is actually how the final result is defined. So, we
/// trivially do this in place in one array. Simply leave the last element untouched in every
/// iteration. This will "leave behind" the last values for each row. For this example, looks like
/// this after every step:
///
/// ```text
/// [10, 13, 16, 21, 30, 45] => calculate differences for the first 5 elements
/// [3, 3, 5, 9, 15, 45] => calculate differences for the first 4 elements
/// [0, 2, 4, 6, 15, 45] => calculate differences for the first 3 elements
/// [2, 2, 2, 6, 15, 45] => calculate differences for the first 2 elements
/// [0, 0, 2, 6, 15, 45] => first two elements are zeroes, we are done
/// ```
///
/// Now, the array only contains zeroes and the "last values" we seek; so we can simply sum the
/// array to find the result.
fn predict(series: &mut [i64]) -> Option<i64> {
    for n in (1..series.len()).rev() {
        for i in 0..n {
            series[i] = series[i + 1] - series[i];
        }
        if series[..=n].iter().all(|n| *n == 0) {
            return Some(series[n..].iter().copied().sum());
        }
    }
    None
}

/// Extrapolates the previous value for a given series.
///
/// This is a modified version of [`predict`]. This time, we are interested in the first column of
/// values:
///
/// ```text
/// 5 <- 10  13  16  21  30  45
///   5 <-  3   3   5   9  15
///    -2 <-  0   2   4   6
///       2 <-  2   2   2
///         0 <-  0   0
/// ```
///
/// We want to arrive at the array [10, 3, 0, 2, 0, 0]. We do that kinda similar to the previous
/// part, but going from the end:
///
/// ```text
/// [10, 13, 16, 21, 30, 35] => calculate differences for the last 5 elements
/// [10, 3, 3, 5, 9, 15] => calculate differences for the last 4 elements
/// [10, 3, 0, 2, 4, 6] => calculate differences for the last 3 elements
/// [10, 3, 0, 2, 2, 2] => calculate differences for the last 2 elements
/// [10, 3, 0, 2, 0, 0] => last two elements are zeroes, we are done
/// ```
///
/// Now, by definition, going from the bottom up, the extrapolated value one layer up is equal to
/// the leftmost value on that layer minus the previous interpolated value. So, in the example
/// given, we have:
///
/// ```text
/// bottom layer:     0
/// layer one up:     2 - 0 = 2
/// layer two up:     0 - 2 = -2
/// layer three up:   3 - (-2) = 5
/// top layer:        10 - 5 = 5
/// ```
///
/// Thus, the final result is 5.
fn extrapolate(series: &mut [i64]) -> Option<i64> {
    for n in (1..series.len()).rev() {
        for i in (series.len() - n..series.len()).rev() {
            series[i] -= series[i - 1];
        }
        if series[series.len() - n..].iter().all(|n| *n == 0) {
            let values = series.iter().take(series.len() - n).rev();
            return Some(values.fold(0, |val, step| step - val));
        }
    }
    None
}

/// Parses the puzzle input into a list of lines per line.
fn parse(input: &str) -> impl Iterator<Item = Vec<i64>> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect()
    })
}
