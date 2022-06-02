/// Find the element after the last inserted one in a small spinlock list.
pub fn one(input: &str) -> Result<String, String> {
    let skip: usize = input.parse().map_err(|_| "failed parse".to_owned())?;
    let mut list: Vec<usize> = vec![0; 2017 + 1];
    let mut position = 0;
    for i in 1..list.len() {
        position = (position + skip + 1) % i;
        list[position..i].rotate_right(1);
        list[position] = i;
    }
    Ok(list[(position + 1) % list.len()].to_string())
}

/// Find the element after 0 in a huge spinlock list.
/// 
/// Unlike [`one`], this does not require us to actually construct the list. We only need to
/// keep track of which element was inserted at position 1 last, since 0 will forever remain
/// the leading element.
pub fn two(input: &str) -> Result<String, String> {
    let skip: usize = input.parse().map_err(|_| "failed parse".to_owned())?;
    let mut last = 0;
    let mut position = 0;
    for i in 1..50000000 + 1 {
        // Position formula differs slightly from the equivalent in `one`, because there we
        // cared about being a valid index (and it wouldn't be on the final iteration), here we
        // care about maintaining zero as lead.
        position = (position + skip) % i + 1;
        if position == 1 {
            last = i;
        }
    }
    Ok(last.to_string())
}
