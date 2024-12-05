use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

/// Counting only correct updates, find the sum of their middle values.
pub fn one(input: &str) -> crate::Result<i32> {
    let (rules, updates) = parse(input).ok_or("no parse")?;
    Ok(updates
        .filter(|xs| {
            xs.iter()
                .enumerate()
                .all(|(i, &a)| xs[i + 1..].iter().all(|&b| right_order(a, b, &rules)))
        })
        .map(|xs| xs[xs.len() / 2])
        .sum())
}

/// Counting only incorrect updates, fix them and find the sum of their middle values.
pub fn two(input: &str) -> crate::Result<i32> {
    let (rules, updates) = parse(input).ok_or("no parse")?;
    Ok(updates
        .filter_map(|mut xs| {
            let prev = xs.clone();
            xs.sort_unstable_by(|&a, &b| match right_order(a, b, &rules) {
                true => Ordering::Less,
                false => Ordering::Greater,
            });
            (prev != xs).then_some(xs[xs.len() / 2])
        })
        .sum())
}

/// Checks if two numbers are in the right order in accordance with the rule map.
fn right_order(a: i32, b: i32, rules: &RuleMap) -> bool {
    !rules[&b].contains(&a)
}

/// All values for a given key must come after it in an update for it to be valid.
type RuleMap = HashMap<i32, HashSet<i32>>;

/// Parses the puzzle input into a map of rules and a list of updates to check.
fn parse(input: &str) -> Option<(RuleMap, impl Iterator<Item = Vec<i32>> + '_)> {
    let (rules, updates) = input.split_once("\r\n\r\n")?;
    let mut rule_map = RuleMap::new();

    for line in rules.lines() {
        let (pre, post) = line.split_once('|')?;
        let (pre, post) = (pre.parse().ok()?, post.parse().ok()?);
        rule_map.entry(pre).or_default().insert(post);
    }

    let updates = updates.lines().map(|line| {
        line.split(',')
            .filter_map(|t| t.parse::<i32>().ok())
            .collect()
    });

    Some((rule_map, updates))
}
