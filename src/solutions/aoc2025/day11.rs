use std::collections::{HashMap, HashSet};

/// Find the number of paths between `you` and `out`.
pub fn one(input: &str) -> crate::Result<usize> {
    let graph: HashMap<_, Vec<_>> = input
        .lines()
        .map(|line| {
            let (src, tgts) = line.split_once(": ").unwrap();
            (src, tgts.split_whitespace().collect())
        })
        .collect();

    let mut stack = vec!["you"];

    let mut paths = 0;
    while let Some(src) = stack.pop() {
        if src == "out" {
            paths += 1;
        } else {
            stack.extend(graph[src].iter().copied());
        }
    }
    Ok(paths)
}

/// Find the number of paths from `svr` that go through `fft` and `dac` and end at `out`.
pub fn two(input: &str) -> crate::Result<usize> {
    // I know from checking that in ALL counted routes, `fft` comes before `dac`, for my input at
    // least. So I scan *backwards* from `fft`, then forwards from `dac` (marking all nodes after
    // `dac` as "forbidden"), and then finally forward between `fft` and `dac` but stopping on
    // forbidden nodes. Those three path counts can be multiplied together to find the final
    // result.

    let graph: HashMap<_, Vec<_>> = input
        .lines()
        .map(|line| {
            let (src, tgts) = line.split_once(": ").unwrap();
            (src, tgts.split_whitespace().collect())
        })
        .collect();

    let mut inverted = HashMap::new();
    for (key, vals) in &graph {
        for &val in vals {
            inverted.entry(val).or_insert(vec![]).push(*key);
        }
    }

    let mut forbidden = HashSet::new();

    let mut paths_to_fft = 0;
    let mut stack = vec!["fft"];
    while let Some(src) = stack.pop() {
        if src == "svr" {
            paths_to_fft += 1;
        } else {
            stack.extend(inverted[src].iter().copied());
        }
    }

    let mut paths_from_dac = 0;
    let mut stack = vec!["dac"];
    while let Some(src) = stack.pop() {
        if src == "out" {
            paths_from_dac += 1;
        } else {
            forbidden.extend(graph[src].iter().copied());
            stack.extend(graph[src].iter().copied());
        }
    }

    let mut paths_between = 0;
    let mut stack = vec!["fft"];
    while let Some(src) = stack.pop() {
        if src == "dac" {
            paths_between += 1;
        } else {
            stack.extend(
                graph[src]
                    .iter()
                    .copied()
                    .filter(|s| !forbidden.contains(s)),
            );
        }
    }

    Ok(paths_to_fft * paths_between * paths_from_dac)
}
