use std::collections::HashMap;

pub fn one(input: &str) -> crate::Result<usize> {
    parse(input)
        .into_iter()
        .max_by_key(|(_, sleeps)| sleeps.iter().map(|sleep| sleep.1 - sleep.0).sum::<usize>())
        .map(|(id, sleeps)| id * sleep_stack(&sleeps).0)
        .ok_or_else(|| "failed to find sleepiest guard".into())
}

pub fn two(input: &str) -> crate::Result<usize> {
    let (id, sleeps) = parse(input)
        .into_iter()
        .max_by_key(|(_, sleeps)| sleep_stack(sleeps).1)
        .ok_or("failed to find most consistently sleep guard")?;
    
    Ok(id * sleep_stack(&sleeps).0)
}

/// Returns the most slept minute for the given timetable, and how often it was slept in.
fn sleep_stack(sleeps: &[(usize, usize)]) -> (usize, usize) {
    let mut minutes = [0; 60];
    for &(start, end) in sleeps {
        for slot in minutes[start..end].iter_mut() {
            *slot += 1;
        }
    }
    
    minutes
        .into_iter()
        .enumerate()
        .max_by_key(|(_, c)| *c)
        .unwrap_or((0, 0))
}

type Timetables = HashMap<usize, Vec<(usize, usize)>>;

fn parse(input: &str) -> Timetables {
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort_unstable_by_key(|line| &line[0..18]);

    let mut result: Timetables = HashMap::new();
    let mut guard = 0;
    let mut start = 0;

    for line in lines {
        if line.ends_with("shift") {
            if let Some(id) = line
                .split_ascii_whitespace()
                .nth(3)
                .and_then(|s| s[1..].parse().ok())
            {
                guard = id;
            }
        } else if line.ends_with("asleep") {
            if let Some(min) = line.split_once(':').and_then(|s| s.1[0..2].parse().ok()) {
                start = min;
            }
        } else if line.ends_with("up") {
            if let Some(end) = line.split_once(':').and_then(|s| s.1[0..2].parse().ok()) {
                result.entry(guard).or_default().push((start, end));
            }
        }
    }

    result
}
