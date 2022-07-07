use std::collections::HashMap;

/// Find the least amount of steps required to move 5 pairs up to the fourth floor.
pub fn one(input: &str) -> crate::Result<String> {
    least_steps_to_move(input, false)
}

/// Find the least amount of steps required to move 7 pairs up to the fourth floor.
pub fn two(input: &str) -> crate::Result<String> {
    least_steps_to_move(input, true)
}

/// Finds the least amount of steps required to move all chips and generators up to the fourth
/// floor, and returns it as a String.
fn least_steps_to_move(input: &str, include_extras: bool) -> crate::Result<String> {
    let mut prev: HashMap<(usize, [(usize, usize); 7]), usize> = HashMap::new();
    let mut states = vec![parse(input, include_extras)];
    let mut best = 1000;

    let pairs = if include_extras { 7 } else { 5 };
    let item_count = pairs * 2;

    let final_key = (3, [(3, 3); 7]);

    while let Some(s) = states.pop() {
        // Prune all branches that:
        // - take longer than the current best result
        // - arrived at a previously-found state slower than before

        if s.steps > best {
            continue;
        }

        if (s.floor, s.items) == final_key {
            best = best.min(s.steps);
        }

        if let Some(v) = prev.get_mut(&(s.floor, s.items)) {
            if s.steps < *v {
                *v = s.steps;
            } else {
                continue;
            }
        }

        prev.insert((s.floor, s.items), s.steps);

        // Try all possible moves of two items up, and of one item down. If i == j, we move one
        // item, hence the `i != j` to determine if we're going up or down.
        let len = states.len();
        for i in 0..item_count {
            for j in i..item_count {
                if let Some(s) = s.next(pairs, i, j, i != j) {
                    states.push(s);
                }
            }
        }

        if len == states.len() {
            // There weren't any valid moves using two up or one down. Also allow all moves with
            // one item up or two items down.
            for i in 0..item_count {
                for j in i..item_count {
                    if let Some(s) = s.next(pairs, i, j, i == j) {
                        states.push(s);
                    }
                }
            }
        }
    }

    if let Some(steps) = prev.get(&final_key) {
        Ok(steps.to_string())
    } else {
        Err("no end state found".into())
    }
}

/// Parses an initial state from the puzzle input.
fn parse(input: &str, include_extras: bool) -> State {
    let mut items: HashMap<&str, (usize, usize)> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let tokens: Vec<_> = line.split(' ').collect();
        for w in tokens.windows(2) {
            if w[1].starts_with("generator") {
                items.entry(w[0]).or_default().1 = i;
            } else if w[1].starts_with("microchip") {
                if let Some((material, _)) = w[0].split_once('-') {
                    items.entry(material).or_default().0 = i;
                }
            }
        }
    }

    let items: Vec<_> = items.values().collect();
    let mut state = State {
        floor: 0,
        steps: 0,
        items: [
            *items[0],
            *items[1],
            *items[2],
            *items[3],
            *items[4],
            (3, 3),
            (3, 3),
        ],
    };

    if include_extras {
        state.items[5] = (0, 0);
        state.items[6] = (0, 0);
    }
    state.items.sort_unstable();

    state
}

/// The amount of floors in the building.
const FLOORS: usize = 4;

/// A possible state.
#[derive(Clone)]
struct State {
    /// The floor that each item is at. Each entry corresponds to a linked chip and generator.
    items: [(usize, usize); 7],
    /// The floor the elevator is at.
    floor: usize,
    /// The number of steps taken to reach this state.
    steps: usize,
}

impl State {
    /// Produces a new state given three inputs: The two items to move, and the direction to move.
    /// If `item1` and `item2` are the same, only one item is moved. If the result would be an
    /// illegal state, [`None`] is returned.
    fn next(&self, pairs: usize, item1: usize, item2: usize, up: bool) -> Option<State> {
        // Can only take items that are on the same floor.
        if self.get(item1) != self.floor || self.get(item2) != self.floor {
            return None;
        }

        // Cannot take a generator and chip of different materials at the same time.
        if (item1 % 2 != item2 % 2) && (item1 / 2 != item2 / 2) {
            return None;
        }

        let lowest_floor = (0..10).map(|i| self.get(i)).min().unwrap();

        let mut state = State {
            items: self.items,
            floor: match up {
                true if self.floor < FLOORS - 1 => self.floor + 1,
                false if self.floor > lowest_floor => self.floor - 1,
                _ => return None,
            },
            steps: self.steps + 1,
        };

        *state.get_mut(item1) = state.floor;
        *state.get_mut(item2) = state.floor;
        state.items[0..pairs].sort_unstable();

        // Every floor must have either zero generators, or zero unpaired chips.
        for floor in 0..FLOORS {
            // If there are no generators, the floor is safe.
            let has_generators = (0..pairs).any(|i| state.get(2 * i + 1) == floor);
            if !has_generators {
                continue;
            }

            // If there are unpaired chips on this floor, it's unsafe.
            let has_unpaired_chips = (0..pairs).any(|i| {
                let (chip, generator) = state.items[i];
                chip == floor && chip != generator
            });
            if has_unpaired_chips {
                return None;
            }
        }

        Some(state)
    }

    /// Gets a mutable reference to the floor either a chip (even index) or generator (odd index)
    /// is at.
    fn get_mut(&mut self, index: usize) -> &mut usize {
        match index % 2 == 0 {
            true => &mut self.items[index / 2].0,
            false => &mut self.items[index / 2].1,
        }
    }

    /// Gets the floor that a chip (even index) or generator (odd index) is at.
    fn get(&self, index: usize) -> usize {
        match index % 2 == 0 {
            true => self.items[index / 2].0,
            false => self.items[index / 2].1,
        }
    }
}
