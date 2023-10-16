use std::collections::HashMap;

/// Find the best happiness rating achievable with a seating arrangement.
pub fn one(input: &str) -> crate::Result<i32> {
    let (people, matrix) = parse(input).ok_or_else(|| "failed to parse puzzle input".to_owned())?;
    all_arrangement_values(people, matrix)
        .max()
        .ok_or_else(|| "no people in input".into())
}

/// Find the best happiness rating achievable with a seating arrangement that includes you.
pub fn two(input: &str) -> crate::Result<i32> {
    let (people, matrix) = parse(input).ok_or_else(|| "failed to parse puzzle input".to_owned())?;
    all_arrangement_values(people + 1, matrix)
        .max()
        .ok_or_else(|| "no people in input".into())
}

type HappinessMatrix = HashMap<(usize, usize), i32>;

/// Returns the total happiness values for all possible arrangements.
fn all_arrangement_values(people: usize, matrix: HappinessMatrix) -> impl Iterator<Item = i32> {
    // Calculates the total happines for a seating arrangement.
    fn arrangement_value(order: &[usize], matrix: &HappinessMatrix) -> i32 {
        order
            .windows(2)
            .chain([[order[order.len() - 1], order[0]].as_slice()])
            .map(|w| {
                *matrix.get(&(w[0], w[1])).unwrap_or(&0) + *matrix.get(&(w[1], w[0])).unwrap_or(&0)
            })
            .sum()
    }

    crate::common::permutations(people)
        .into_iter()
        .map(move |v| arrangement_value(&v[..], &matrix))
}

/// Parses the puzzle input into an amount of people and a map of happiness deltas.
fn parse(input: &str) -> Option<(usize, HappinessMatrix)> {
    let mut people = HashMap::new();
    let mut matrix = HashMap::new();

    for line in input.lines().map(|s| &s[0..s.len() - 1]) {
        let line: Vec<_> = line.split(' ').collect();
        for person in [line[0], line[10]] {
            if !people.contains_key(person) {
                people.insert(person, people.len());
            }
        }

        let v = line[3].parse::<i32>().ok()?
            * match line[2] {
                "gain" => 1,
                "lose" => -1,
                _ => 0,
            };
        matrix.insert((people[line[0]], people[line[10]]), v);
    }

    Some((people.len(), matrix))
}
