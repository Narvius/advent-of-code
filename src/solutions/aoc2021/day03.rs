use std::cmp::Ordering;

/// Find the gamma rate (most common bit in each position in the input) and the epsilon rate (the
/// inverse of the gamma rate), and multiply them together.
pub fn one(input: &str) -> crate::Result<u32> {
    let data: Vec<_> = input.lines().collect();
    let mut gamma = 0u32;

    for i in 0..data[0].len() {
        gamma *= 2;
        let ones = data.iter().filter(|s| s.as_bytes()[i] == b'1').count();
        if ones * 2 >= data.len() {
            gamma += 1;
        }
    }
    // This is basically equivalent to `!gamma`, but constrained to `input[0].len` binary digits.
    let epsilon = (1 << data[0].len() as u32) - 1 - gamma;

    Ok(gamma * epsilon)
}

/// Find the two ratings according to some convoluted bit-based filtering mechanism, and multiply
/// them together.
pub fn two(input: &str) -> crate::Result<u32> {
    let oxygen = find_rating(input, Ordering::Greater, b'1');
    let scrubber = find_rating(input, Ordering::Less, b'0');

    Ok(oxygen * scrubber)
}

/// Filters the input data until only one value remains, then returns that value. `partition`
/// determines which bit to keep (most or least common), and `tiebreaker` is the bit that gets used
/// when both bits are equally common.
fn find_rating(data: &str, partition: Ordering, tiebreaker: u8) -> u32 {
    let mut data: Vec<_> = data.lines().collect();

    for i in 0..data[0].len() {
        // split the candidates list into those for which the relevant bit is '0', and for which
        // the relevant bit is '1'.
        data.sort_unstable_by_key(|v| v.as_bytes()[i]);
        let zero_count = data.iter().take_while(|v| v.as_bytes()[i] == b'0').count();
        let one_partition = data.split_off(zero_count);

        // at this point, 'candidates' is the 'zero' partition, and 'one_partition' is the, well,
        // 'one' partition. Now, decide which of these to keep, and store it in 'candidates'. The
        // other partition is dropped and deallocated.
        match zero_count.cmp(&one_partition.len()) {
            Ordering::Equal => {
                if tiebreaker == b'1' {
                    data = one_partition;
                }
            }
            ordering => {
                if ordering != partition {
                    data = one_partition;
                }
            }
        }

        if data.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(data[0], 2).unwrap()
}
