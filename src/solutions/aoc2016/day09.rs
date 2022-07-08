/// Find the length of the string after decompressing it.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(decompressed_length(input.as_bytes(), false))
}

/// Find the length of the string after recursively decompressing it.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(decompressed_length(input.as_bytes(), true))
}

/// Calculates the length of a string after "decompressing" it according to the rules of the
/// puzzle. `recursive` controls whether strings that were duplicated by decompression are further
/// decompressed or not.
fn decompressed_length(bytes: &[u8], recursive: bool) -> usize {
    let (mut i, mut len) = (0, 0);
    while i < bytes.len() {
        match bytes[i] {
            b'(' => {
                let (mut stride, mut mult) = (0, 0);
                i += 1;
                while bytes[i] != b'x' {
                    stride = (stride * 10) + (bytes[i] - b'0') as usize;
                    i += 1;
                }
                i += 1;
                while bytes[i] != b')' {
                    mult = (mult * 10) + (bytes[i] - b'0') as usize;
                    i += 1;
                }

                len += if recursive {
                    mult * decompressed_length(&bytes[i + 1..i + stride + 1], true)
                } else {
                    mult * stride
                };

                i += stride;
            }
            c if c.is_ascii_alphanumeric() => len += 1,
            _ => {}
        }
        i += 1;
    }
    len
}
