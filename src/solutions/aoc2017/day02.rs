/// For each line, find the difference between the highest and lowest number, then sum it all.
pub fn one(input: &str) -> crate::Result<String> {
    let mut result = 0;
    for line in input.lines() {
        let mut min = i32::MAX;
        let mut max = 0;
        for x in line.split_whitespace() {
            let x = x.parse().map_err(|_| "failed to parse num".to_owned())?;
            min = min.min(x);
            max = max.max(x);
        }
        result += max - min;
    }
    Ok(result.to_string())
}

/// For each line, find the pair of numbers that is evenly divisble, then sum the quotients.
pub fn two(input: &str) -> crate::Result<String> {
    let mut result = 0;
    for line in input.lines() {
        let xs = line
            .split_whitespace()
            .map(|x| x.parse().map_err(|_| "failed to parse num".to_owned()))
            .collect::<Result<Vec<i32>, String>>()?;
        'line: for i in 0..xs.len() {
            for j in 0..xs.len() {
                if i != j && xs[i] % xs[j] == 0 {
                    result += xs[i] / xs[j];
                    break 'line;
                }
            }
        }
    }
    Ok(result.to_string())
}
