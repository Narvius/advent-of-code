/// Using scoring rules, find the total score of all scratchcards.
pub fn one(input: &str) -> crate::Result<u32> {
    Ok(parse(input)
        .filter(|&c| c > 0)
        .map(|c| 2u32.pow(c as u32 - 1))
        .sum())
}

/// Using recursive scratchcard rules, find the total number of scratchcards gained.
pub fn two(input: &str) -> crate::Result<usize> {
    let cards: Vec<_> = parse(input).collect();
    let mut counts = vec![1; cards.len()];

    for i in 0..cards.len() {
        for n in 1..=cards[i] {
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
        let (cards, draws) = numbers.split_once(" | ")?;
        let cards: Vec<_> = to_numbers(cards).collect();

        Some(to_numbers(draws).filter(|n| cards.contains(n)).count())
    })
}
