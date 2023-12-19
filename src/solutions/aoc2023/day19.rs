use std::{cmp::Ordering, collections::HashMap};

/// Calculate a checksum from all input [`Thing`]s that get accepted.
pub fn one(input: &str) -> crate::Result<i64> {
    let (flows, things) = parse(input).ok_or("parse failed")?;

    Ok(things
        .filter_map(|thing| {
            let mut flow = "in";
            let mut accepted;
            loop {
                (flow, accepted) = resolve_flow(&flows, flow, &thing);
                if let Some(v) = accepted {
                    return v.then(|| thing.into_iter().sum::<i64>());
                }
            }
        })
        .sum())
}

/// Count the total number of possible [`Thing`]s that would be accepted.
pub fn two(input: &str) -> crate::Result<i64> {
    let (flows, _) = parse(input).ok_or("parse failed")?;
    Ok(count_accepts(&flows, "in", [(1, 4000); 4]))
}

/// Given a single `flow`, returns the next flow and, optionally, a bool indicating whether the
/// [`Thing`] was accepted (true) or rejected (false). Should technically be an `Either` result,
/// but Rust doesn't have a general-purpose `Either` type, so I just return both.
fn resolve_flow<'a>(flows: &Flows<'a>, flow: &str, thing: &Thing) -> (&'a str, Option<bool>) {
    for &(check, target) in &flows[flow] {
        let target = match check {
            Some((i, order, n)) => {
                if thing[i].cmp(&n) == order {
                    target
                } else {
                    continue;
                }
            }
            None => target,
        };

        return match target {
            "A" => ("", Some(true)),
            "R" => ("", Some(false)),
            target => (target, None),
        };
    }

    unreachable!()
}

/// Counts the number of things with values in the provided `ranges` that are accepted.
fn count_accepts(flows: &Flows, flow: &str, mut ranges: [(i64, i64); 4]) -> i64 {
    let mut result = 0;
    for &(check, target) in &flows[flow] {
        let new_ranges = match check {
            Some((i, order, n)) => {
                let mut sub_ranges = ranges;
                match order {
                    Ordering::Less => {
                        ranges[i].0 = n;
                        sub_ranges[i].1 = n - 1;
                    }
                    Ordering::Greater => {
                        ranges[i].1 = n;
                        sub_ranges[i].0 = n + 1;
                    }
                    Ordering::Equal => unreachable!(),
                }
                sub_ranges
            }
            None => ranges,
        };

        result += match target {
            "A" => new_ranges
                .into_iter()
                .map(|(lo, hi)| 0.max(1 + hi - lo))
                .product(),
            "R" => 0,
            target => count_accepts(flows, target, new_ranges),
        };
    }
    result
}

/// A single branch in a sorting workflow. When the condition is `None`, it is always `true`.
type Branch<'a> = (Option<(usize, Ordering, i64)>, &'a str);

/// A map of thing-sorting workflows.
type Flows<'a> = HashMap<&'a str, Vec<Branch<'a>>>;

/// [x, m, a, s] value for a thing to be sorted.
type Thing = [i64; 4];

/// Parses the puzzle input into a [flow name => flow branches] map and a list of [`Thing`]s.
fn parse(input: &str) -> Option<(Flows, impl Iterator<Item = Thing> + '_)> {
    let (flows, things) = input.split_once("\r\n\r\n")?;
    let mut flow_map = HashMap::new();

    for flow in flows.lines() {
        let (name, spec) = flow.trim_end_matches('}').split_once('{')?;
        flow_map.insert(
            name,
            spec.split(',')
                .filter_map(|cond| {
                    Some(match cond.split_once(':') {
                        Some((cond, target)) => {
                            let src = match cond.as_bytes()[0] {
                                b'x' => 0,
                                b'm' => 1,
                                b'a' => 2,
                                b's' => 3,
                                _ => unreachable!(),
                            };

                            let order = match cond.as_bytes()[1] {
                                b'<' => Ordering::Less,
                                b'>' => Ordering::Greater,
                                _ => unreachable!(),
                            };

                            (Some((src, order, cond[2..].parse().ok()?)), target)
                        }
                        None => (None, cond),
                    })
                })
                .collect(),
        );
    }

    let things = things.lines().filter_map(|line| {
        line.split(&['=', ',', '}'][..])
            .filter_map(|n| n.parse::<i64>().ok())
            .collect::<Vec<_>>()
            .try_into()
            .ok()
    });

    Some((flow_map, things))
}
