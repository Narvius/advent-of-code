/// Going horizontally, count the number of valid triangles.
pub fn one(input: &str) -> Result<String, String> {
    let sides = input
        .lines()
        .flat_map(|line| line.split(' ').filter_map(|t| t.parse().ok()));
    Ok(count_triangles(sides).to_string())
}

/// Going vertically, count the number of valid triangles.
pub fn two(input: &str) -> Result<String, String> {
    let (mut v1, mut v2, mut v3) = (vec![], vec![], vec![]);
    for line in input.lines() {
        let mut sides = line.split(' ').filter_map(|t| t.parse().ok());
        if let (Some(a), Some(b), Some(c)) = (sides.next(), sides.next(), sides.next()) {
            v1.push(a);
            v2.push(b);
            v3.push(c);
        }
    }
    Ok(count_triangles([v1, v2, v3].into_iter().flat_map(|v| v)).to_string())
}

/// Given a stream of numbers, counts how many valid triangles it forms.
fn count_triangles(mut ls: impl Iterator<Item = i32>) -> i32 {
    let mut count = 0;
    while let (Some(a), Some(b), Some(c)) = (ls.next(), ls.next(), ls.next()) {
        if a + b > c && b + c > a && c + a > b {
            count += 1;
        }
    }
    count
}
