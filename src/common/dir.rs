//! Eight cardinal and intercardinal directions.

/// Eight cardinal and intercardinal directions.
///
/// Can be added to `(i32, i32)` points to take a step in that direction.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Dir {
    W,
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
}

/// The four cardinal directions.
pub const CARDINAL: [Dir; 4] = [Dir::W, Dir::N, Dir::E, Dir::S];

#[rustfmt::skip]
/// All eight directions.
pub const ALL_DIRS: [Dir; 8] = [Dir::W, Dir::NW, Dir::N, Dir::NE, Dir::E, Dir::SE, Dir::S, Dir::SW];

impl Dir {
    /// Rotates by the given amount of steps. `steps` should be a number between 0 and 7
    /// (inclusive), representing the amount of 45 degree steps to the right to rotate.
    fn rotate_steps(self, steps: isize) -> Self {
        ((self as isize + steps) % 8).try_into().unwrap()
    }

    /// Rotated to the left by 45 degrees.
    pub fn halfleft(self) -> Self {
        self.rotate_steps(7)
    }

    /// Rotated to the left by 90 degrees.
    pub fn left(self) -> Self {
        self.rotate_steps(6)
    }

    /// Rotated to the right by 45 degrees.
    pub fn halfright(self) -> Self {
        self.rotate_steps(1)
    }

    /// Rotated to the right by 90 degrees.
    pub fn right(self) -> Self {
        self.rotate_steps(2)
    }

    /// Rotated by 180 degrees.
    pub fn opposite(self) -> Self {
        self.rotate_steps(4)
    }
}

impl std::ops::Add<Dir> for (i32, i32) {
    type Output = (i32, i32);

    fn add(self, rhs: Dir) -> Self::Output {
        let (dx, dy) = <(i32, i32)>::from(rhs);
        (self.0 + dx, self.1 + dy)
    }
}

impl std::ops::Add<&Dir> for (i32, i32) {
    type Output = (i32, i32);

    fn add(self, rhs: &Dir) -> Self::Output {
        self + *rhs
    }
}

impl From<Dir> for (i32, i32) {
    fn from(value: Dir) -> Self {
        match value {
            Dir::W => (-1, 0),
            Dir::NW => (-1, -1),
            Dir::N => (0, -1),
            Dir::NE => (1, -1),
            Dir::E => (1, 0),
            Dir::SE => (1, 1),
            Dir::S => (0, 1),
            Dir::SW => (-1, 1),
        }
    }
}

impl TryFrom<isize> for Dir {
    type Error = ();

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Dir::W,
            1 => Dir::NW,
            2 => Dir::N,
            3 => Dir::NE,
            4 => Dir::E,
            5 => Dir::SE,
            6 => Dir::S,
            7 => Dir::SW,
            _ => return Err(()),
        })
    }
}
