use std::collections::HashMap;

/// Read all the assignment operations, treating masks as modifying the value being written.
/// We can decompose the masks as written into two actual numbers, one used to AND with, the
/// other to OR with, allowing us to succinctly modify all the bits specified without
/// touching any of the other ones.
pub fn one(input: &str) -> crate::Result<u64> {
    let mut values: HashMap<u64, u64> = HashMap::new();
    let mut and_mask = u64::MAX;
    let mut or_mask = u64::MIN;

    for line in input.lines() {
        if let Some(mask) = line.strip_prefix("mask = ") {
            and_mask = u64::from_str_radix(&mask.replace('X', "1"), 2)?;
            or_mask = u64::from_str_radix(&mask.replace('X', "0"), 2)?;
        } else if let Some(line) = line.strip_prefix("mem[") {
            let (addr, val) = line.split_once("] = ").ok_or("invalid mem line")?;
            *values.entry(addr.parse()?).or_default() = val.parse::<u64>()? & and_mask | or_mask
        }
    }

    Ok(values.into_values().sum())
}

pub fn two(input: &str) -> crate::Result<u64> {
    let mut values: HashMap<u64, u64> = HashMap::new();
    let mut or_mask = u64::MIN;
    let mut floats = vec![];

    for line in input.lines() {
        if let Some(mask) = line.strip_prefix("mask = ") {
            or_mask = u64::from_str_radix(&mask.replace('X', "0"), 2)?;
            floats = (0..mask.len())
                .filter(|i| mask.as_bytes()[mask.len() - i - 1] == b'X')
                .collect();
        } else if let Some(line) = line.strip_prefix("mem[") {
            let (addr, val) = line.split_once("] = ").ok_or("invalid mem line")?;
            let mut target = addr.parse::<u64>()? | or_mask;
            let val = val.parse::<u64>()?;
            for i in 0..2usize.pow(floats.len() as u32) {
                for (n, &float) in floats.iter().enumerate() {
                    if i & (1 << n) > 0 {
                        target &= !(1 << float);
                    } else {
                        target |= 1 << float;
                    }
                }
                *values.entry(target).or_default() = val;
            }
        }
    }

    Ok(values.into_values().sum())
}
