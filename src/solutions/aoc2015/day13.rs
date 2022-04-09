use std::collections::HashMap;

/// Find the best happiness rating achievable with a seating arrangement.
pub fn one(input: &str) -> Result<String, String> {
    let (people, matrix) = parse(input).ok_or(format!("failed to parse puzzle input"))?;
    Ok(all_arrangement_values(people, matrix)
        .max()
        .ok_or(format!("no people in input"))?
        .to_string())
}

/// Find the best happiness rating achievable with a seating arrangement that includes you.
pub fn two(input: &str) -> Result<String, String> {
    let (people, matrix) = parse(input).ok_or(format!("failed to parse puzzle input"))?;
    Ok(all_arrangement_values(people + 1, matrix)
        .max()
        .ok_or(format!("no people in input"))?
        .to_string())
}

type HappinessMatrix = HashMap<(usize, usize), i32>;

/// Returns the total happiness values for all possible arrangements.
fn all_arrangement_values(people: usize, matrix: HappinessMatrix) -> impl Iterator<Item = i32> {
    // Returns all possible permutations of the numbers in `0..k`, using Heap's algorithm.
    fn permutations(k: usize) -> Vec<Vec<usize>> {
        fn inner(k: usize, values: &mut [usize]) -> Vec<Vec<usize>> {
            let mut result = Vec::new();
            if k <= 1 {
                result.push(Vec::from(values));
            } else {
                result.extend(inner(k - 1, values));
                for i in 0..(k - 1) {
                    if k % 2 == 0 {
                        values.swap(i, k - 1);
                    } else {
                        values.swap(0, k - 1);
                    }
                    result.extend(inner(k - 1, values));
                }
            }
            result
        }

        inner(k, &mut (0..k).collect::<Vec<_>>())
    }

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

    permutations(people)
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
