/// Find the product of two entries that sum to 2020.
pub fn one(input: &str) -> crate::Result<i32> {
    let ns: Vec<i32> = input.lines().filter_map(|line| line.parse().ok()).collect();
    for i in 0..ns.len() {
        for j in (i + 1)..ns.len() {
            if ns[i] + ns[j] == 2020 {
                return Ok(ns[i] * ns[j]);
            }
        }
    }
    Err("no result".into())
}

/// Find the product of three entries that sum to 2020.
pub fn two(input: &str) -> crate::Result<i32> {
    let ns: Vec<i32> = input.lines().filter_map(|line| line.parse().ok()).collect();
    for i in 0..ns.len() {
        for j in (i + 1)..ns.len() {
            for k in (j + 1)..ns.len() {
                if ns[i] + ns[j] + ns[k] == 2020 {
                    return Ok(ns[i] * ns[j] * ns[k]);
                }
            }
        }
    }
    Err("no result".into())
}
