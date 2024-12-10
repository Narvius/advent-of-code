/// Compact all files, fragmenting them, and find the resulting filesystem checksum.
pub fn one(input: &str) -> crate::Result<usize> {
    let p = input.trim().as_bytes();
    let (mut start, mut end) = (1, p.len() - 1);
    let (mut space, mut data) = (digit(p[start]), digit(p[end]));
    let mut position = digit(p[0]);
    let mut checksum = 0;

    while start < end {
        // Main routine. Keep using up space and data to increase checksum and position.
        while space > 0 && data > 0 {
            space -= 1;
            data -= 1;
            checksum += position * end / 2;
            position += 1;
        }

        // Once we're out of space, advance the left pointer, adding passed data packets into the
        // checksum.
        while space == 0 && (start + 1) < end {
            for _ in 0..digit(p[start + 1]) {
                checksum += position * (start + 1) / 2;
                position += 1;
            }
            start += 2;
            space = digit(p[start]);
        }

        // Once we're out of data to move, advance the right pointer.
        while data == 0 {
            end -= 2;
            data = digit(p[end]);
        }

        // Special case: If we haven't been able to acquire new space (because the start pointer is
        // too close to the end pointer), manually resolve the last remaining data.
        if space == 0 && data > 0 {
            for _ in 0..data {
                checksum += position * end / 2;
                position += 1;
            }
            end -= 2;
        }
    }

    Ok(checksum)
}

/// Compact all files, without fragmenting them, and find the resulting filesystem checksum.
pub fn two(input: &str) -> crate::Result<usize> {
    let (mut files, mut spaces) = (vec![], vec![]);
    let mut position = 0;
    let mut checksum = 0;

    // "Parse"--index all files and spaces.
    for (i, c) in input.trim().as_bytes().chunks(2).enumerate() {
        let file = digit(c[0]);
        files.push((position, file, i));
        if c.len() == 2 {
            let space = digit(c[1]);
            spaces.push((position + file, space));
            position += file + space;
        }
    }

    // Going from the right, try to place each file.
    for (p_file, size, id) in files.into_iter().rev() {
        let p = if let Some((p_space, free)) = spaces.iter_mut().find(|(_, free)| *free >= size) {
            // If we find a suitable spot, shrink the remaining free space, and return the position
            // we inserted at.
            *free -= size;
            *p_space += size;
            *p_space - size
        } else {
            // If we don't find a spot, just keep the file where it is.
            p_file
        };

        for i in 0..size {
            checksum += (p + i) * id;
        }
        spaces.pop(); // File is done, so last space is now to the right of all remaining files.
    }

    Ok(checksum)
}

/// Given an ASCII digit, returns it as an `usize`.
fn digit(b: u8) -> usize {
    (b - b'0') as usize
}
