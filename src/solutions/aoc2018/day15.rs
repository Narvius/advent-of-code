use std::collections::HashSet;

/// Run the battle, get the outcome.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(Battle::from_input(input).run_to_completion().0)
}

/// Buff elves so they barely win without casualties; run the battle, get the outcome.
pub fn two(input: &str) -> crate::Result<i32> {
    let battle = Battle::from_input(input);
    for i in 4.. {
        if let (outcome, true) = battle.with_elf_power(i).run_to_completion() {
            return Ok(outcome);
        }
    }

    Err("unreachable".into())
}

/// A battle. Can be indexed with a `(i32, i32)` to get whether the selected tile is open.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Battle {
    /// All units on the battlefield.
    units: Vec<Unit>,
    /// The map that is being fought on.
    map: Vec<Vec<bool>>,
    /// The number of elves (initially).
    elf_count: usize,
    /// Combat power of elves.
    elf_power: i32,
    /// The amount of full turns that elapsed.
    turns: usize,
}

impl Battle {
    /// Parses the puzzle input into a ready-to-run battle.
    fn from_input(input: &str) -> Self {
        let mut units = vec![];
        let mut map = vec![];

        for (y, line) in input.lines().enumerate() {
            let mut xs = vec![];
            for (x, c) in line.char_indices() {
                xs.push(match c {
                    'G' | 'E' => {
                        units.push(Unit::new((x as i32, y as i32), c == 'E'));
                        false
                    }
                    '.' => true,
                    _ => false,
                });
            }
            map.push(xs);
        }

        let elf_count = units.iter().filter(|u| u.elf).count();
        Self {
            units,
            map,
            elf_count,
            elf_power: 3,
            turns: 0,
        }
    }

    /// Clones this battle, setting elf_power to the provided value in the clone.
    fn with_elf_power(&self, elf_power: i32) -> Self {
        let mut result = self.clone();
        result.elf_power = elf_power;
        result
    }

    /// Runs the fight to completion, and returns the outcome (as described in the puzzle),
    /// as well as a bool saying whether the elves won without casualties.
    fn run_to_completion(mut self) -> (i32, bool) {
        loop {
            self.units.sort_unstable_by_key(|u| key(u.pos, 0));
            for i in 0..self.units.len() {
                if !self.execute_unit_turn(i) {
                    self.units.retain(|u| u.hp > 0);
                    let elves = self.units[0].elf && self.elf_count == self.units.len();
                    return (
                        self.turns as i32 * self.units.into_iter().map(|u| u.hp).sum::<i32>(),
                        elves,
                    );
                }
            }
            self.units.retain(|u| u.hp > 0);
            self.turns += 1;
        }
    }
}

impl Battle {
    /// Executes the turn of a single unit. Returns whether the battle continues.
    fn execute_unit_turn(&mut self, unit: usize) -> bool {
        let Unit { pos, hp, elf } = self.units[unit];

        // Step 0: Dead units can't act.
        if hp <= 0 {
            return true;
        }

        // Step 1: Check if there are targets. If not, stop the fight.
        if !self.units.iter().any(|u| u.hp > 0 && u.elf != elf) {
            return false;
        }

        // Step 2: Find potential target squares. If there are none, end the turn.
        let targets: HashSet<_> = self
            .units
            .iter()
            .filter(|u| u.hp >= 0 && u.elf != elf)
            .flat_map(|u| splat(u.pos))
            .filter(|&p| self[p] || p == pos)
            .collect();
        if targets.is_empty() {
            return true;
        }

        // Step 3: Search for shortest paths to target squares, and pick the first one in
        //         reading order; if one was found, move to it.
        let path = {
            // Breadth-first search, taking steps in reading order. We must take one step in all
            // directions at the same time, because we have to find all paths that terminate
            // at the same length (in order to pick the final one based on other criteria).
            let mut visited = HashSet::from([pos]);
            let mut heads = vec![(pos, None)];

            while !heads.is_empty() && !heads.iter().any(|(p, _)| targets.contains(p)) {
                let mut next = vec![];
                for (s, dir) in heads {
                    for p in splat(s) {
                        if visited.insert(p) && self[p] {
                            next.push((p, dir.or_else(|| Some(sub(p, s)))));
                        }
                    }
                }
                heads = next;
            }

            heads
                .into_iter()
                .filter(|(p, _)| targets.contains(p))
                .min_by_key(|(p, _)| key(*p, 0))
        };
        if let Some((_, Some(dir))) = path {
            self.units[unit].pos = add(pos, dir);
            self[pos] = true;
            self[add(pos, dir)] = false;
        }

        // Step 4: Find an adjacent foe, and strike it.
        let pos = self.units[unit].pos;
        let target = self
            .units
            .iter_mut()
            .filter(|u| u.hp > 0 && u.elf != elf && splat(pos).contains(&u.pos))
            .min_by_key(|u| key(u.pos, u.hp));
        if let Some(target) = target {
            target.hp -= if elf { self.elf_power } else { 3 };
            if target.hp <= 0 {
                let pos = target.pos;
                self[pos] = true;
            }
        }

        true
    }
}

impl std::ops::Index<(i32, i32)> for Battle {
    type Output = bool;

    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        &self.map[y as usize][x as usize]
    }
}

impl std::ops::IndexMut<(i32, i32)> for Battle {
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
        &mut self.map[y as usize][x as usize]
    }
}

/// A single unit participating in the fight.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Unit {
    pos: (i32, i32),
    hp: i32,
    elf: bool,
}

impl Unit {
    /// Constructs a new unit at the given location.
    fn new(pos: (i32, i32), elf: bool) -> Self {
        Self { pos, hp: 200, elf }
    }
}

// Creates a sorting key that is equivalent to sorting by, in order, `hp`, `y` and `x`.
fn key((x, y): (i32, i32), hp: i32) -> i32 {
    (hp << 16) + (y << 8) + x
}

// Adds two points.
fn add((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> (i32, i32) {
    (ax + bx, ay + by)
}

// Subtracts two points.
fn sub((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> (i32, i32) {
    (ax - bx, ay - by)
}

// A list of points adjacent to the provided one, in reading order.
fn splat((x, y): (i32, i32)) -> [(i32, i32); 4] {
    [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
}
