use std::cmp::Ordering;

/// Find the sum of the indices of pairs in the input that are sorted.
pub fn one(input: &str) -> crate::Result<usize> {
    let lists = parse(input);
    Ok(lists
        .chunks(2)
        .enumerate()
        .filter(|(_, c)| c[0] < c[1])
        .map(|(i, _)| i + 1)
        .sum())
}

/// Sort the list of items, and multiply together the positions of two sentinel values.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut lists = parse(input);
    let div_a = Item::List(vec![Item::List(vec![Item::Value(2)])]);
    let div_b = Item::List(vec![Item::List(vec![Item::Value(6)])]);
    lists.extend([div_a.clone(), div_b.clone()]);
    lists.sort_unstable();

    let pos_a = lists.iter().position(|i| *i == div_a).ok_or("lost div_a")?;
    let pos_b = lists.iter().position(|i| *i == div_b).ok_or("lost div_b")?;

    Ok((pos_a + 1) * (pos_b + 1))
}

/// Parses the puzzle input into a list of [`Item`]s. Filters out empty lines.
fn parse(input: &str) -> Vec<Item> {
    fn parse_line(input: &str) -> Item {
        if input.is_empty() || input == "[]" {
            return Item::List(vec![]);
        }

        Item::List(
            split_on_shallow_commas(&input[1..input.len() - 1])
                .map(|item| match item.parse::<i32>() {
                    Ok(n) => Item::Value(n),
                    _ => parse_line(item),
                })
                .collect(),
        )
    }

    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect()
}

/// Splits a string on commas, but not if those commas are surrounded by square brackets.
fn split_on_shallow_commas(input: &str) -> impl Iterator<Item = &str> + '_ {
    let mut depth = 0;
    input.split(move |c| {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' if depth == 0 => return true,
            _ => {}
        };
        false
    })
}

/// A list of numbers and/or lists, with custom comparison logic that follows the puzzle
/// description.
#[derive(Clone, Debug, Eq, PartialEq)]
enum Item {
    Value(i32),
    List(Vec<Item>),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        use Item::*;
        match (self, other) {
            (Value(lhs), Value(rhs)) => lhs.cmp(rhs),
            (lhs @ Value(_), rhs @ List(_)) => List(vec![lhs.clone()]).cmp(rhs),
            (lhs @ List(_), rhs @ Value(_)) => lhs.cmp(&List(vec![rhs.clone()])),
            (List(lhs), List(rhs)) => {
                for i in 0.. {
                    match (i == lhs.len(), i == rhs.len()) {
                        (true, true) => return Ordering::Equal,
                        (true, false) => return Ordering::Less,
                        (false, true) => return Ordering::Greater,
                        _ => {
                            let cmp = lhs[i].cmp(&rhs[i]);
                            if cmp != Ordering::Equal {
                                return cmp;
                            }
                        }
                    }
                }

                unreachable!()
            }
        }
    }
}
