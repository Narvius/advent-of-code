pub fn one(input: &str) -> crate::Result<usize> {
    let mut total = 0;
    let mut intervals = vec![];
    for line in input.lines() {
        let (pattern, groups) = line.split_once(' ').ok_or("invalid line")?;
        let groups: Vec<usize> = groups.split(',').filter_map(|n| n.parse().ok()).collect();

        intervals.clear();
        intervals.push((0, groups[0] - 1));
        for i in 1..groups.len() {
            let start = intervals[i - 1].1 + 2;
            intervals.push((start, start + groups[i] - 1));
        }

        loop {
            if valid_arrangement(pattern.as_bytes(), &intervals) {
                total += 1;
            }

            if !next_arrangement(intervals.as_mut_slice(), pattern.as_bytes().len()) {
                break;
            }
        }
    }
    Ok(total)
}

pub fn two(input: &str) -> crate::Result<&str> {
    Err("unimplemented".into())
}

fn valid_arrangement(pattern: &[u8], intervals: &[(usize, usize)]) -> bool {
    let mut i = 0;

    for &(lo, hi) in intervals {
        while i < lo {
            if pattern[i] == b'#' {
                return false;
            }
            i += 1;
        }

        while i <= hi {
            if pattern[i] == b'.' {
                return false;
            }
            i += 1;
        }
    }

    while i < pattern.len() {
        if pattern[i] == b'#' {
            return false;
        }
        i += 1;
    }

    true
}

/// Modifies `group` to be the next arrangement, and returns whether it has done so successfully.
/// When it returns `false`, all arrangements have been checked.
fn next_arrangement(groups: &mut [(usize, usize)], length: usize) -> bool {
    for i in (0..groups.len()).rev() {
        // Check if there's space to the right.
        let end = match groups.get(i + 1) {
            Some(&(s, _)) => s - 2,
            None => length - 1,
        };

        // If so, move current group one space to the right.
        if groups[i].1 < end {
            groups[i].0 += 1;
            groups[i].1 += 1;

            // Now we have to left-align all blocks after self.
            for j in (i + 1)..groups.len() {
                let new_start = groups[j - 1].1 + 2;
                let length = groups[j].1 - groups[j].0;
                groups[j] = (new_start, new_start + length);
            }

            return true;
        }
    }

    false
}
