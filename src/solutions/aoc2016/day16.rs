/// Fill a small disk, then calculate the checksum.
pub fn one(input: &str) -> Result<String, String> {
    reduce_to_checksum(fill_disk(input, 272))
}

/// Fill a large disk, then calculate the checksum.
pub fn two(input: &str) -> Result<String, String> {
    reduce_to_checksum(fill_disk(input, 35651584))
}

/// Fills a disk to the given size from the seed, and returns it.
fn fill_disk(input: &str, length: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(length);
    v.extend_from_slice(&Vec::from(input.as_bytes()));
    while v.len() < length {
        v.push(b'0');
        for i in (0..(v.len() - 1)).rev().take(length - v.len()) {
            v.push(b'0' + b'1' - v[i]);
        }
    }
    v
}

/// Reduces a given disk to a checksum and returns it.
fn reduce_to_checksum(mut disk: Vec<u8>) -> Result<String, String> {
    while disk.len() % 2 == 0 {
        for i in 0..disk.len() / 2 {
            disk[i] = b'0' + (disk[2 * i] == disk[2 * i + 1]) as u8;
        }
        disk.truncate(disk.len() / 2);
    }
    String::from_utf8(disk).map_err(|_| "failed to construct checksum".to_owned())
}
