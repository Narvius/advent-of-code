use std::{
    collections::{HashSet, VecDeque},
    hash::{Hash, Hasher},
};

/// Run a game of Combat to completion and find the winner's score.
pub fn one(input: &str) -> crate::Result<i32> {
    let (mut p1, mut p2) = parse(input);

    while !p1.is_empty() && !p2.is_empty() {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2 {
            p1.extend([c1, c2]);
        } else {
            p2.extend([c2, c1]);
        }
    }

    Ok(score(p1) + score(p2))
}

/// Run a game of Recursive Combat to completion and find the winner's score.
pub fn two(input: &str) -> crate::Result<i32> {
    let (mut p1, mut p2) = parse(input);
    recursive(&mut p1, &mut p2);
    Ok(score(p1) + score(p2))
}

/// Runs a game of Recursive Combat; returns `true` if player 1 won, false otherwise.
fn recursive(p1: &mut VecDeque<i32>, p2: &mut VecDeque<i32>) -> bool {
    let mut history = HashSet::new();
    while !p1.is_empty() && !p2.is_empty() {
        let hash = hashed(&p1, &p2);
        if !history.insert(hash) {
            return true;
        }

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        let p1won = if c1 as usize <= p1.len() && c2 as usize <= p2.len() {
            let mut p1 = p1.iter().copied().take(c1 as usize).collect();
            let mut p2 = p2.iter().copied().take(c2 as usize).collect();
            recursive(&mut p1, &mut p2)
        } else {
            c1 > c2
        };

        if p1won {
            p1.extend([c1, c2]);
        } else {
            p2.extend([c2, c1]);
        }
    }
    p2.is_empty()
}

/// Hashes a game state; used to check if a game state repeats.
fn hashed(p1: &VecDeque<i32>, p2: &VecDeque<i32>) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    (p1, p2).hash(&mut hasher);
    hasher.finish()
}

/// Calculates the score of a deck.
fn score(p: VecDeque<i32>) -> i32 {
    p.into_iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i as i32 + 1) * v)
        .sum()
}

/// Parses the puzzle input into the two decks used.
fn parse(input: &str) -> (VecDeque<i32>, VecDeque<i32>) {
    let mut lines = input.lines();
    let (mut p1, mut p2) = (VecDeque::new(), VecDeque::new());

    lines.next();
    while let Some(n) = lines.next().and_then(|v| v.parse().ok()) {
        p1.push_back(n);
    }
    lines.next();
    while let Some(n) = lines.next().and_then(|v| v.parse().ok()) {
        p2.push_back(n);
    }

    (p1, p2)
}
