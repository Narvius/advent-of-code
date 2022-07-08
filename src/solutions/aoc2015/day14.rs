/// Finds the distance the furthest reindeer has travelled after 2503 seconds.
pub fn one(input: &str) -> crate::Result<usize> {
    // For a given reindeer, returns how much distance they will have travelled after a given time.
    fn distance_after_time((speed, travel, rest): Reindeer, seconds: usize) -> usize {
        let cycles = seconds / (travel + rest);
        let leftover = seconds % (travel + rest);

        speed * travel * cycles + speed * travel.min(leftover)
    }

    parse(input)
        .map(|r| distance_after_time(r, 2503))
        .max()
        .ok_or_else(|| "no reindeer in input".into())
}

/// Finds the best score achieved by a reindeer after 2503 seconds.
pub fn two(input: &str) -> crate::Result<usize> {
    let reindeer: Vec<_> = parse(input).collect();
    let mut scores = vec![0; reindeer.len()];
    let mut distances = vec![0; reindeer.len()];
    let mut cycle = vec![0; reindeer.len()];
    let mut furthest = 0;

    for _ in 0..2503 {
        for i in 0..reindeer.len() {
            let (speed, travel, rest) = reindeer[i];
            if cycle[i] < travel {
                distances[i] += speed;
                furthest = furthest.max(distances[i]);
            }

            cycle[i] = (cycle[i] + 1) % (travel + rest);
        }

        for i in 0..distances.len() {
            if distances[i] == furthest {
                scores[i] += 1;
            }
        }
    }

    scores.into_iter().max().ok_or_else(|| "no results".into())
}

type Reindeer = (usize, usize, usize);

/// Parses the puzzle input into an enumeration of reindeer.
fn parse(input: &str) -> impl Iterator<Item = Reindeer> + '_ {
    input.lines().filter_map(|line| {
        let line: Vec<_> = line.split(' ').collect();
        Some((
            line[3].parse().ok()?,
            line[6].parse().ok()?,
            line[13].parse().ok()?,
        ))
    })
}
