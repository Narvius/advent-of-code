use crate::common::astar;

pub fn one(input: &str) -> crate::Result<usize> {
    let map: Vec<Vec<_>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    astar::shortest_path_length(Crucible::new(false), &map).ok_or("no result".into())
}

pub fn two(input: &str) -> crate::Result<usize> {
    let map: Vec<Vec<_>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    astar::shortest_path_length(Crucible::new(true), &map).ok_or("no result".into())
}

/// Used as a node in an [A* search](astar).
#[derive(Clone, Eq, Hash, PartialEq)]
struct Crucible {
    position: (i32, i32),
    direction: (i32, i32),
    straights: i32,
    ultra: bool,
}

impl Crucible {
    /// Creates a new default [`Crucible`] of the given type.
    fn new(ultra: bool) -> Self {
        Self {
            position: (0, 0),
            direction: (1, 0),
            straights: 0,
            ultra,
        }
    }

    /// Creates a new [`Crucible`] that is the result of taking a step in the provided
    /// direction.
    fn in_dir(&self, (dx, dy): (i32, i32)) -> Self {
        let straights = 1 + if self.direction == (dx, dy) {
            self.straights
        } else {
            0
        };
        let (x, y) = self.position;
        Self {
            position: (x + dx, y + dy),
            direction: (dx, dy),
            straights,
            ultra: self.ultra,
        }
    }

    /// Checks if this [`Crucible`] is valid (hasn't stepped off the map and not taken too many
    /// steps forward), and if so, returns itself alongside the cost of having done the last
    /// step.
    fn with_cost(self, map: &[Vec<u8>]) -> Option<(Self, usize)> {
        let (Ok(x), Ok(y)) = (
            usize::try_from(self.position.0),
            usize::try_from(self.position.1),
        ) else {
            return None;
        };

        let straight_cap = if self.ultra { 10 } else { 3 };
        if self.straights > straight_cap {
            return None;
        }

        map.get(y)
            .and_then(|r| r.get(x))
            .map(|&cost| (self, (cost - b'0') as usize))
    }
}

impl<'a> astar::Node<'a> for Crucible {
    type Cost = usize;
    type Env = Vec<Vec<u8>>;

    fn next(&self, env: &'a Self::Env) -> Box<dyn Iterator<Item = (Self, Self::Cost)> + 'a> {
        let (dx, dy) = self.direction;
        let left = (dy, -dx);
        let right = (-dy, dx);

        match self.ultra {
            false => Box::new(
                [self.in_dir((dx, dy)), self.in_dir(left), self.in_dir(right)]
                    .into_iter()
                    .filter_map(|s| s.with_cost(env)),
            ),
            true => {
                let s = self.straights;
                Box::new(
                    [
                        (s < 10).then(|| self.in_dir(self.direction)),
                        (4 <= s || self.position == (0, 0)).then(|| self.in_dir(left)),
                        (4 <= s || self.position == (0, 0)).then(|| self.in_dir(right)),
                    ]
                    .into_iter()
                    .flatten()
                    .filter_map(|s| s.with_cost(env)),
                )
            }
        }
    }

    fn heuristic(&self, env: &Self::Env) -> Self::Cost {
        let (x, y) = self.position;
        let (width, height) = (env[0].len(), env.len());

        (x as usize).abs_diff(width) + (y as usize).abs_diff(height)
    }

    fn done(&self, env: &Self::Env) -> bool {
        let (x, y) = self.position;
        let (width, height) = (env[0].len() as i32, env.len() as i32);

        let position_fits = (x + 1) == width && (y + 1) == height;
        let ultra = !self.ultra || self.straights >= 4;

        position_fits && ultra
    }
}
