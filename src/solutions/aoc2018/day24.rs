/// Resolve the fight, count survivors.
pub fn one(input: &str) -> crate::Result<usize> {
    let (a, b) = parse(input).ok_or("failed to parse input")?;
    Ok(resolve(a, b).1)
}

/// Find the smallest possible number added to the attack of every unit on the immune system side
/// that leads to a win; count survivors.
///
/// Simply does a binary search starting from a value that is high enough that it will guarantee
/// a win.
pub fn two(input: &str) -> crate::Result<usize> {
    let (a, b) = parse(input).ok_or("failed to parse puzzle input")?;

    let mut boost = 1024;
    let mut diff = 512;

    loop {
        let (mut a, b) = (a.clone(), b.clone());
        for a in a.iter_mut() {
            a.power += boost;
        }

        match resolve(a, b) {
            // Boost is too weak
            (0, _) => boost += diff,

            // Boost is either too strong or exactly strong enough
            (a, 0) if diff == 1 => break Ok(a),
            (_, 0) => boost -= diff,

            // Fight stalemated; the correct result is higher
            _ => boost += diff,
        }

        diff /= 2;
    }
}

/// Resolves a fight between two sides. Only performs up to 10000 rounds, to avoid looping
/// forever on stalemates.
fn resolve(mut a: Vec<Squad>, mut b: Vec<Squad>) -> (usize, usize) {
    let mut turns = 10000;
    while turns > 0 && !a.is_empty() && !b.is_empty() {
        pick_targets(&mut a, &mut b);
        do_attacks(&mut a, &mut b);

        a.retain(|s| s.count > 0);
        b.retain(|s| s.count > 0);
        turns -= 1;
    }

    (
        a.iter().map(|s| s.count).sum(),
        b.iter().map(|s| s.count).sum(),
    )
}

/// Does target resolution as described in the puzzle.
fn pick_targets(a: &mut [Squad], b: &mut [Squad]) {
    pick(a, b);
    pick(b, a);

    fn pick(a: &mut [Squad], b: &mut [Squad]) {
        for u in a.iter_mut() {
            u.chose_target = false;
        }
        for u in b.iter_mut() {
            u.available_to_target = true;
        }

        while let Some(source) = a
            .iter_mut()
            .filter(|u| !u.chose_target)
            .max_by_key(|u| (u.effectiveness(), u.initiative))
        {
            let target = b
                .iter_mut()
                .enumerate()
                .filter(|u| u.1.available_to_target)
                .max_by_key(|u| (source.damage_to(u.1), u.1.effectiveness(), u.1.initiative));

            source.chose_target = true;
            source.target = target.and_then(|(n, u)| {
                (source.damage_to(u) > 0).then(|| {
                    u.available_to_target = false;
                    n
                })
            });
        }
    }
}

/// Performs attacks as described in the puzzle.
fn do_attacks(a: &mut [Squad], b: &mut [Squad]) {
    loop {
        let (a_initiative, a_index) = fastest(a);
        let (b_initiative, b_index) = fastest(b);

        if a_initiative.is_none() && b_initiative.is_none() {
            break;
        }

        if a_initiative > b_initiative {
            attack(&mut a[a_index], b);
        } else {
            attack(&mut b[b_index], a);
        }
    }

    fn attack(source: &mut Squad, targets: &mut [Squad]) {
        if let Some(target) = source.target.take() {
            let target = &mut targets[target];
            let damage = source.damage_to(target);
            target.count = target.count.saturating_sub(damage / target.hp);
        }
    }
}

/// Finds the highest-initiative squad that still has a target in the list; and returns its
/// initiative (as an [`Option`]) and index.
///
/// If the initiative is [`None`], the index will be meaningless.
fn fastest(squads: &mut [Squad]) -> (Option<usize>, usize) {
    let fastest = squads
        .iter()
        .enumerate()
        .filter(|v| v.1.target.is_some())
        .max_by_key(|v| v.1.initiative);

    (
        fastest.map(|v| v.1.initiative),
        fastest.map(|v| v.0).unwrap_or(0),
    )
}

/// A single group of units, corresponding to one line of puzzle input.
#[derive(Clone)]
struct Squad<'a> {
    count: usize,
    hp: usize,
    power: usize,
    initiative: usize,

    hit_type: &'a str,
    mods: Vec<(&'a str, usize)>,

    chose_target: bool,
    target: Option<usize>,
    available_to_target: bool,
}

impl Squad<'_> {
    /// Called "effective power" in the puzzle; used as a basis for various calculations.
    fn effectiveness(&self) -> usize {
        self.count * self.power
    }

    /// The damage this [`Squad`] would deal to the `other` one. Takes immunities and
    /// resistances into account.
    fn damage_to(&self, other: &Squad) -> usize {
        let modval = other.mods.iter().find(|v| v.0 == self.hit_type);
        self.effectiveness() * modval.map(|v| v.1).unwrap_or(1)
    }
}

/// Parses a line of puzzle input into a [`Squad`].
fn parse_squad(line: &str) -> Option<Squad> {
    let tokens: Vec<_> = line.split_whitespace().collect();
    let mods = {
        if let Some((_, mods)) = line.split_once(" (") {
            let mods = mods.split_once(") ")?.0;
            let (immunities, weaknesses) = match mods.split_once("; ") {
                Some((l, r)) if l.starts_with("weak") => {
                    (r.strip_prefix("immune to ")?, l.strip_prefix("weak to ")?)
                }
                Some((l, r)) if r.starts_with("weak") => {
                    (l.strip_prefix("immune to ")?, r.strip_prefix("weak to ")?)
                }
                None if mods.starts_with("weak") => ("", mods.strip_prefix("weak to ")?),
                _ => (mods.strip_prefix("immune to ")?, ""),
            };

            immunities
                .split(", ")
                .map(|t| (t, 0))
                .chain(weaknesses.split(", ").map(|t| (t, 2)))
                .filter(|s| !s.0.is_empty())
                .collect()
        } else {
            vec![]
        }
    };

    let len = tokens.len();
    Some(Squad {
        count: tokens[0].parse().ok()?,
        hp: tokens[4].parse().ok()?,
        power: tokens[len - 6].parse().ok()?,
        initiative: tokens[len - 1].parse().ok()?,
        hit_type: tokens[len - 5],
        mods,
        chose_target: false,
        target: None,
        available_to_target: true,
    })
}

/// Parses the puzzle input into the two sides of a fight.
fn parse(input: &str) -> Option<(Vec<Squad>, Vec<Squad>)> {
    let (left, right) = input.split_once("\n\n")?;
    Some((
        left.lines()
            .skip(1)
            .map(parse_squad)
            .collect::<Option<Vec<_>>>()?,
        right
            .lines()
            .skip(1)
            .map(parse_squad)
            .collect::<Option<Vec<_>>>()?,
    ))
}
