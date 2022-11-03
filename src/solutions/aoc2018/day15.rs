use std::collections::HashSet;

/// Run the battle, get the outcome.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(Battle::from_input(input, 3).run_to_completion().0)
}

/// Buff elves so they barely win without casualties; run the battle, get the outcome.
pub fn two(input: &str) -> crate::Result<i32> {
    for i in 4.. {
        if let (outcome, true) = Battle::from_input(input, i).run_to_completion() {
            return Ok(outcome);
        }
    }

    Err("unreachable".into())
}

/// A battle.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Battle {
    /// All units on the battlefield.
    units: Vec<Unit>,
    /// The map that is being fought on.
    map: Vec<Vec<bool>>,
    /// The number of elves (initially).
    elf_count: usize,
    /// The amount of full turns that elapsed.
    turns: usize,
}

impl Battle {
    /// Parses the puzzle input into a ready-to-run battle.
    fn from_input(input: &str, elf_power: i32) -> Self {
        let mut units = vec![];
        let mut map = vec![];

        for (y, line) in input.lines().enumerate() {
            let mut xs = vec![];
            for (x, c) in line.char_indices() {
                xs.push(match c {
                    'G' | 'E' => {
                        units.push(Unit::new((x as i32, y as i32), c == 'E', elf_power));
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
            turns: 0,
        }
    }

    /// Runs the fight to completion, and returns the outcome (as described in the puzzle),
    /// as well as a bool saying whether the elves won without casualties.
    fn run_to_completion(mut self) -> (i32, bool) {
        loop {
            self.units.sort_unstable_by_key(|u| u.priority(false));
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
    /// Executes the turn of a single unit.
    fn execute_unit_turn(&mut self, unit: usize) -> bool {
        let Unit {
            pos: (x, y),
            hp,
            elf,
            power,
        } = self.units[unit];

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
            .flat_map(|u| DELTAS.map(|(dx, dy)| (u.pos.0 + dx, u.pos.1 + dy)))
            .filter(|&(tx, ty)| self.open(tx, ty) || (tx, ty) == (x, y))
            .collect();
        if targets.is_empty() {
            return true;
        }

        // Step 3: Pick a direction to move in, and do.
        if let Some((_, (dx, dy))) = self
            .fewest_reachable_tiles(unit, &targets)
            .into_iter()
            .min_by_key(|((x, y), _)| y * 1000 + x)
        {
            self.units[unit].pos = (x + dx, y + dy);
            self.map[y as usize][x as usize] = true;
            self.map[(y + dy) as usize][(x + dx) as usize] = false;
        }

        // Step 4: Strike an adjacent foe.
        if let Some(target) = self.get_target(unit) {
            self.units[target].hp -= power;
            let Unit {
                pos: (x, y), hp, ..
            } = self.units[target];
            if hp <= 0 {
                self.map[y as usize][x as usize] = true;
            }
        }

        true
    }

    /// Checks whether a given tile is free or not.
    fn open(&self, x: impl TryInto<usize>, y: impl TryInto<usize>) -> bool {
        if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
            self.map[y][x]
        } else {
            false
        }
    }

    /// Returns the target for a unit (adjacent foe with lowest HP), if any.
    fn get_target(&self, unit: usize) -> Option<usize> {
        let Unit {
            pos: (x, y),
            hp,
            elf,
            ..
        } = self.units[unit];
        if hp <= 0 {
            return None;
        }

        self.units
            .iter()
            .enumerate()
            .filter(|(_, u)| {
                let (ux, uy) = u.pos;
                u.hp > 0 && u.elf != elf && DELTAS.contains(&(ux - x, uy - y))
            })
            .min_by_key(|(_, u)| u.priority(true))
            .map(|p| p.0)
    }

    /// Returns a list of all tiles that are reachable in the fewest number of steps, alongside
    /// the direction to go in in order to reach it.
    fn fewest_reachable_tiles(
        &self,
        unit: usize,
        targets: &HashSet<(i32, i32)>,
    ) -> Vec<((i32, i32), (i32, i32))> {
        let Unit {
            pos: (x, y), hp, ..
        } = self.units[unit];

        if hp <= 0 || targets.contains(&(x, y)) {
            return vec![];
        }

        // Breadth-first search, taking steps in reading order. Always perform a full depth step
        // of the breadth first search, in order to catch cases where there are multiple target
        // squares reached.
        let mut visited = HashSet::from([(x, y)]);
        let mut heads = vec![(x, y, None)];

        while !heads.is_empty() && !heads.iter().any(|&(x, y, _)| targets.contains(&(x, y))) {
            let mut next = vec![];
            for (x, y, dir) in heads {
                for (dx, dy) in DELTAS {
                    let (x, y) = (x + dx, y + dy);

                    if visited.contains(&(x, y)) {
                        continue;
                    }

                    visited.insert((x, y));
                    if self.open(x, y) {
                        next.push((x, y, dir.or(Some((dx, dy)))));
                    }
                }
            }
            heads = next;
        }

        heads
            .into_iter()
            .filter_map(|(x, y, dir)| {
                targets
                    .contains(&(x, y))
                    .then(|| ((x, y), dir.unwrap_or((0, 0))))
            })
            .collect()
    }
}

/// A single unit participating in the fight.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Unit {
    pos: (i32, i32),
    hp: i32,
    power: i32,
    elf: bool,
}

impl Unit {
    /// Constructs a new unit at the given location. If the unit is an elf, `elf_power` is
    /// used for `power`, instead of the default value.
    fn new(pos: (i32, i32), elf: bool, elf_power: i32) -> Self {
        Self {
            pos,
            hp: 200,
            power: if elf { elf_power } else { 3 },
            elf,
        }
    }

    /// Sorting priority (lower = earlier). `include_hp` is for sorting when selecting
    /// a target to strike.
    fn priority(&self, include_hp: bool) -> i32 {
        (if include_hp { self.hp * 1000000 } else { 0 }) + self.pos.1 * 1000 + self.pos.0
    }
}

/// Four directions, in reading order (up, left, right, down).
static DELTAS: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
