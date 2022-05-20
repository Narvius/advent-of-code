use serde_json::Value;

/// Finds the sum of all numbers in the JSON blob.
pub fn one(input: &str) -> Result<String, String> {
    Ok(regex::Regex::new(r"-?\d+")
        .unwrap()
        .find_iter(input)
        .filter_map(|n| n.as_str().parse::<i32>().ok())
        .sum::<i32>()
        .to_string())
}

/// Find the sum of all numbers in the JSON blob, excluding red objects.
pub fn two(input: &str) -> Result<String, String> {
    let tree = serde_json::from_str(input).map_err(|_| "failed to parse input".to_owned())?;
    Ok(non_red_sum(tree).to_string())
}

/// Finds the sum of all numbers in the JSON blob, excluding ones contained within objects with
/// a property with the value "red".
fn non_red_sum(node: Value) -> i64 {
    match node {
        Value::Number(n) => n.as_i64().unwrap_or(0),
        Value::Array(v) => v.into_iter().map(non_red_sum).sum(),
        Value::Object(o) => {
            if o.values().any(|v| v.as_str() == Some("red")) {
                0
            } else {
                o.into_iter().map(|(_, v)| non_red_sum(v)).sum()
            }
        }
        _ => 0,
    }
}
