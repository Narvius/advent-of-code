/// Find the sum of the results of all operations, going column-by-column.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(cephalopod_math(input, |set, i| {
        set.get(i).map(|n| n.trim().parse::<usize>().unwrap())
    }))
}

/// Find the sum of the results of all operations, going column-by-column, and also reading numbers
/// column-wise.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(cephalopod_math(input, |set, i| {
        (i < set[0].len()).then(|| {
            set.iter().fold(0, |acc, line| {
                let c = line.as_bytes()[i];
                if c.is_ascii_digit() {
                    10 * acc + (c - b'0') as usize
                } else {
                    acc
                }
            })
        })
    }))
}

/// Performs "cephalopod math" (folding over all numbers in a column with the operation at the
/// bottom of it). `read_num` reads the `i`th number for the column of operations.
fn cephalopod_math(input: &str, read_num: fn(&[&str], usize) -> Option<usize>) -> usize {
    // Detect all clean-break columns (while splitting on whitespace is sufficient for part 1, we
    // need the actual break columns in order for things to line up in part 2).
    let mut lines: Vec<_> = input.lines().collect();
    let mut breaks = (0..lines[0].len())
        .filter(|&i| lines.iter().all(|line| line.as_bytes()[i] == b' '))
        .collect::<Vec<_>>();
    breaks.push(lines[0].len());

    let ops: Vec<_> = lines.pop().unwrap().split_whitespace().collect();

    // Since we already did the work of finding the breaks, we might as well convert the input into
    // a more user-friendly problem set-based format.
    let mut sets = vec![];
    let mut prev = 0;
    for b in breaks {
        sets.push(lines.iter().map(|line| &line[prev..b]).collect::<Vec<_>>());
        prev = b + 1;
    }

    // Perform the actual folding for each column.
    (ops.into_iter().zip(sets))
        .map(|(op, set)| {
            let mut r = if op == "+" { 0 } else { 1 };
            for i in 0.. {
                if let Some(n) = read_num(&set, i) {
                    r = if op == "+" { r + n } else { r * n };
                } else {
                    break;
                }
            }
            r
        })
        .sum()
}
