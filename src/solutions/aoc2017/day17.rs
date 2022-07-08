/// Find the element after the last inserted one in a small spinlock list.
pub fn one(input: &str) -> crate::Result<usize> {
    let skip: usize = input.parse().map_err(|_| "failed parse".to_owned())?;
    let mut list: Vec<usize> = vec![0; 2017 + 1];
    let mut position = 0;
    for i in 1..list.len() {
        position = (position + skip + 1) % i;
        list[position..i].rotate_right(1);
        list[position] = i;
    }
    Ok(list[(position + 1) % list.len()])
}

/// Find the element after 0 in a huge spinlock list.
///
/// Unlike [`one`], this does not require us to actually construct the list. We only need to
/// keep track of which element was inserted at position 0 last, since with the way we
/// calculate insert positions, 0 will always be at the end of the list (and since the list
/// is circular, the element at *index* 0 is the one after the *value* 0).
pub fn two(input: &str) -> crate::Result<usize> {
    let skip: usize = input.parse().map_err(|_| "failed parse".to_owned())?;
    let mut last = 0;
    let mut position = 0;
    for i in 1..50000000 + 1 {
        position = (position + skip + 1) % i;
        if position == 0 {
            last = i;
        }
    }
    Ok(last)
}
