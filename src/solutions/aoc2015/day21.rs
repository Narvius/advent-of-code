/// Find the cost of the cheapest possible loadout to win with.
pub fn one(input: &str) -> Result<String, String> {
    Ok(all_outcomes(parse(input)?).0.to_string())
}

/// Find the cost of the most expensive possible loadout to lose with.
pub fn two(input: &str) -> Result<String, String> {
    Ok(all_outcomes(parse(input)?).1.to_string())
}

type Stat = (i32, i32, i32);
type Loadout = (usize, usize, usize, usize);

/// Simulates a fight between you and the boss, returning `true` if you won, `false` otherwise.
fn fight(mut you: Stat, mut enemy: Stat) -> bool {
    loop {
        enemy.0 -= 1.max(you.1 - enemy.2);
        if enemy.0 <= 0 {
            return true;
        }
        you.0 -= 1.max(enemy.1 - you.2);
        if you.0 <= 0 {
            return false;
        }
    }
}

/// Tries the fight with all possible loadouts, and records the cheapest win and most expensive
/// loss.
fn all_outcomes(boss: Stat) -> (i32, i32) {
    let (mut cheapest_win, mut most_expensive_loss) = (i32::MAX, i32::MIN);
    for w in 0..W.len() {
        for a in 0..=A.len() {
            for r1 in 0..=R.len() {
                for r2 in 0..=R.len() {
                    if let Some((stats, gold)) = resolve_loadout((w, a, r1, r2)) {
                        if fight(stats, boss) {
                            cheapest_win = cheapest_win.min(gold);
                        } else {
                            most_expensive_loss = most_expensive_loss.max(gold);
                        }
                    }
                }
            }
        }
    }
    (cheapest_win, most_expensive_loss)
}

/// Given a loadout, calculates the resulting stats and cost.
fn resolve_loadout((w, a, r1, r2): Loadout) -> Option<(Stat, i32)> {
    if r1 < R.len() && r1 == r2 {
        None?;
    }

    let (mut gold, mut atk, mut def) = (0, 0, 0);
    for v in [W.get(w), A.get(a), R.get(r1), R.get(r2)] {
        if let Some(&(g, a, d)) = v {
            gold += g;
            atk += a;
            def += d;
        }
    }
    Some(((100, atk, def), gold))
}

// The stats for purchaseable gear. The first value in each pair is the gold cost, the second value
// is the stat chance--a positive value is attack, a negative value is defense.
const W: [Stat; 5] = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
const A: [Stat; 5] = [(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
const R: [Stat; 6] = [
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

/// Parses the puzzle input into the boss' stats.
fn parse(input: &str) -> Result<Stat, String> {
    let tokens: Vec<_> = input
        .lines()
        .filter_map(|line| line.split(' ').last())
        .filter_map(|n| n.parse().ok())
        .collect();
    if let &[hp, atk, def] = tokens.as_slice() {
        Ok((hp, atk, def))
    } else {
        Err(format!("failed to parse input"))
    }
}
