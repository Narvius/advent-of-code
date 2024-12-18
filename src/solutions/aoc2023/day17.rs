use crate::common::{astar, Dir, Grid};

pub fn one(input: &str) -> crate::Result<usize> {
    let map = Grid::from_input(input);
    astar::shortest_path_length(Crucible::new(false), &map).ok_or("no result".into())
}

pub fn two(input: &str) -> crate::Result<usize> {
    let map = Grid::from_input(input);
    astar::shortest_path_length(Crucible::new(true), &map).ok_or("no result".into())
}

/// Used as a node in an [A* search](astar).
#[derive(Clone, Eq, Hash, PartialEq)]
struct Crucible {
    position: (i32, i32),
    heading: Dir,
    straights: i32,
    ultra: bool,
}

impl Crucible {
    /// Creates a new default [`Crucible`] of the given type.
    fn new(ultra: bool) -> Self {
        Self {
            position: (0, 0),
            heading: Dir::E,
            straights: 0,
            ultra,
        }
    }

    /// Creates a new [`Crucible`] that is the result of taking a step in the provided
    /// direction.
    fn in_dir(&self, heading: Dir) -> Self {
        let straights = 1 + match self.heading == heading {
            true => self.straights,
            _ => 0,
        };

        Self {
            position: self.position + heading,
            heading,
            straights,
            ultra: self.ultra,
        }
    }

    /// Checks if this [`Crucible`] is valid (hasn't stepped off the map and not taken too many
    /// steps forward), and if so, returns itself alongside the cost of having done the last
    /// step.
    fn with_cost(self, grid: &Grid<u8>) -> Option<(Self, usize)> {
        let straight_cap = if self.ultra { 10 } else { 3 };
        if self.straights > straight_cap {
            return None;
        }

        grid.get(self.position)
            .map(|cost| (self, (cost - b'0') as usize))
    }
}

impl<'a> astar::Node<'a> for Crucible {
    type Cost = usize;
    type Env = Grid<'a, u8>;

    fn next(&self, env: &'a Self::Env) -> Box<dyn Iterator<Item = (Self, Self::Cost)> + 'a> {
        match self.ultra {
            false => Box::new(
                [self.heading, self.heading.left(), self.heading.right()]
                    .map(|d| self.in_dir(d))
                    .into_iter()
                    .filter_map(|node| node.with_cost(env)),
            ),
            true => {
                let s = self.straights;
                Box::new(
                    [
                        (s < 10).then_some(self.heading),
                        (4 <= s || self.position == (0, 0)).then(|| self.heading.left()),
                        (4 <= s || self.position == (0, 0)).then(|| self.heading.right()),
                    ]
                    .map(|d| d.map(|d| self.in_dir(d)))
                    .into_iter()
                    .flatten()
                    .filter_map(move |node| node.with_cost(env)),
                )
            }
        }
    }

    fn heuristic(&self, env: &Self::Env) -> Self::Cost {
        let (x, y) = self.position;
        (x as usize).abs_diff(env.width()) + (y as usize).abs_diff(env.height())
    }

    fn done(&self, env: &Self::Env) -> bool {
        let (x, y) = (self.position.0 as usize, self.position.1 as usize);

        let reached_end = x == env.width() - 1 && y == env.height() - 1;
        let ultra = !self.ultra || self.straights >= 4;

        reached_end && ultra
    }
}
