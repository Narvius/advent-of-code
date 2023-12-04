/// Using scoring rules, find the total score of all scratchcards.
pub fn one(input: &str) -> crate::Result<u32> {
    Ok(parse(input)
        .filter(|&c| c > 0)
        .map(|c| 2u32.pow(c as u32 - 1))
        .sum())
}

/// Using recursive scratchcard rules, find the total number of scratchcards gained.
pub fn two(input: &str) -> crate::Result<usize> {
    let scores: Vec<_> = parse(input).collect();
    let mut counts = vec![1; scores.len()];

    for i in 0..scores.len() {
        for n in 1..=scores[i] {
            counts[i + n] += counts[i];
        }
    }

    Ok(counts.into_iter().sum())
}

/// For each line of input, returns the "score" (count of winning numbers) for that line.
fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    fn to_numbers(s: &str) -> impl Iterator<Item = i32> + '_ {
        s.split_whitespace().filter_map(|n| n.parse().ok())
    }

    input.lines().filter_map(|line| {
        let (_, numbers) = line.split_once(": ")?;
        let (card, draws) = numbers.split_once(" | ")?;

        let card: Vec<_> = to_numbers(card).collect();
        Some(to_numbers(draws).filter(|n| card.contains(n)).count())
    })
}
