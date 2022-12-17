/// Find the height of the tower after dropping 2022 rocks.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(tower_height(input, 2022))
}

/// Find the height of the tower after dropping 1000000000000 rocks.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(tower_height(input, 1000000000000))
}

/// Returns the height of the rock tower after the given amount of `steps`.
fn tower_height(input: &str, mut steps: usize) -> usize {
    let mut state = State::new(input);

    // Find the "cycle length". After one cycle length of rocks, both inputs (shapes and jets)
    // line up again exactly like in the beginning.
    let cycle = match state.jets.len() % SHAPES.len() {
        0 => state.jets.len(),
        _ => state.jets.len() * SHAPES.len(),
    };

    // If there's few steps, we can just run them and be done with it. Otherwise, continue on
    // to more complicated processing.
    if steps < cycle {
        state.drop_rocks(steps);
        return state.map.len();
    }

    // Now, we keep dropping `cycle` rocks, and remembering the height/height gain after each
    // time. Since we know that the inputs (shapes / jets) align at each of those points, if
    // the situation of the tower also aligns between two of them, we know that these two
    // can be repeated infinitely without actually running the simulation.
    let mut tops: Vec<(usize, usize)> = vec![];
    let (steps_per_repeat, height_per_repeat) = loop {
        state.drop_rocks(cycle);
        steps -= cycle;
        let position = state.map.len();
        let gain = position - tops.last().map(|(p, _)| *p).unwrap_or(0);
        let repeat = tops
            .iter()
            .position(|&(p, g)| state.map[position - gain..position] == state.map[p - g..p]);

        if let Some(p) = repeat {
            break (cycle * (tops.len() - p), position - tops[p].0);
        } else {
            tops.push((position, gain));
        }
    };

    // We've found a repeat; so we can just skip actually dropping rocks as long as there's
    // space for full cycles.
    let simulated_height = height_per_repeat * (steps / steps_per_repeat);

    // Drop the few actual rocks needed to finish.
    state.drop_rocks(steps % steps_per_repeat);

    state.map.len() + simulated_height
}

/// Width of the shaft.
const WIDTH: i32 = 7;

type Map = Vec<[bool; WIDTH as usize]>;
type Shape = (&'static [(i32, i32)], i32);

/// Represents the state
struct State {
    map: Map,
    shape_index: usize,
    jet_index: usize,
    jets: Vec<i32>,
}

impl State {
    /// Creates a fresh [`State`] from the puzzle input.
    fn new(input: &str) -> Self {
        let jets = input.trim().chars().map(|c| if c == '<' { -1 } else { 1 });
        Self {
            map: vec![],
            shape_index: 0,
            jet_index: 0,
            jets: jets.collect(),
        }
    }

    /// Drops the next rock, as per the puzzle description.
    fn drop_rock(&mut self) {
        let shape = SHAPES[self.shape_index];
        let (mut x, mut y) = (2, 3 + self.map.len() as i32);
        self.shape_index = (self.shape_index + 1) % SHAPES.len();
        loop {
            let jet = self.jets[self.jet_index];
            self.jet_index = (self.jet_index + 1) % self.jets.len();
            if !collides(&self.map, shape, (x + jet, y)) {
                x += jet;
            }

            if collides(&self.map, shape, (x, y - 1)) {
                place(&mut self.map, shape, (x, y));
                return;
            }

            y -= 1;
        }
    }

    /// Calls [`drop_rock`](self::drop_rock) multiple `times`.
    fn drop_rocks(&mut self, times: usize) {
        for _ in 0..times {
            self.drop_rock();
        }
    }
}

/// Edits the given `map` to include the shape at the specified position.
fn place(map: &mut Map, (shape, _): Shape, (x, y): (i32, i32)) {
    for &(dx, dy) in shape {
        let (x, y) = ((x + dx) as usize, (y + dy) as usize);
        if map.len() <= y {
            map.push([false; WIDTH as usize]);
        }
        map[y][x] = true;
    }
}

/// Checks whether a rock at the given position would collide with something.
fn collides(map: &Map, (shape, w): Shape, (x, y): (i32, i32)) -> bool {
    // If it intersects with walls or floor, it collides.
    if !(0..=WIDTH - w).contains(&x) || y < 0 {
        return true;
    }

    // If its above any of the map data, it definitely doesn't collide.
    if y >= map.len() as i32 {
        return false;
    }

    shape.iter().any(|(dx, dy)| {
        map.get((y + dy) as usize)
            .map(|line| line[(x + dx) as usize])
            .unwrap_or(false)
    })
}

/// Possible rock shapes, defined by a set of points and their total width.
static SHAPES: [Shape; 5] = [
    // Flat long
    (&[(0, 0), (1, 0), (2, 0), (3, 0)], 4),
    // Cross
    (&[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], 3),
    // Bottom right corner
    (&[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], 3),
    // Standing long
    (&[(0, 0), (0, 1), (0, 2), (0, 3)], 1),
    // Square
    (&[(0, 0), (1, 0), (0, 1), (1, 1)], 2),
];
