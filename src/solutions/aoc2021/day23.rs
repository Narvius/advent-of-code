use crate::common::astar::{self, AStarNode};

/// Sort amphipods as efficiently as possible, return used energy.
pub fn one(input: &str) -> crate::Result<i32> {
    let s = parse(input);

    astar::shortest_path_length(s, &()).ok_or("no result".into())
}

/// Sort *more* amphipods as efficiently as possible, return used energy.
pub fn two(input: &str) -> crate::Result<i32> {
    let s = parse_extended(input);

    astar::shortest_path_length(s, &()).ok_or("no result".into())
}

/// A (compacted) state of the map. `rest` are rest spots outside the columns,
/// `cols` are the columns.
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
struct State<const COL_LEN: usize> {
    rest: [u8; 7],
    cols: [heapless::Vec<u8, COL_LEN>; 4],
}

impl<const COL_LEN: usize> AStarNode for State<COL_LEN> {
    type Cost = i32;
    type Env = ();

    /// Produces all possible next states.
    fn next(&self, _: &Self::Env) -> Box<dyn Iterator<Item = (Self, Self::Cost)> + '_> {
        Box::new(self.moves().map(move |(m, cost)| {
            let mut next = self.clone();
            match (m.0, m.1) {
                (Index::Column(s), Index::Column(t)) => {
                    let val = next.cols[s].pop().unwrap();
                    next.cols[t].push(val).unwrap();
                }
                (Index::Column(s), Index::Rest(r)) => {
                    let val = next.cols[s].pop().unwrap();
                    next.rest[r] = val;
                }
                (Index::Rest(r), Index::Column(t)) => {
                    let val = next.rest[r];
                    next.rest[r] = 0;
                    next.cols[t].push(val).unwrap();
                }
                _ => unreachable!(),
            }
            (next, cost)
        }))
    }

    /// Returns what it would cost if every missing amphipod was 1 step away
    /// from home.
    fn heuristic(&self, _: &Self::Env) -> Self::Cost {
        (0..4)
            .map(|n| {
                let correct = self.cols[n].iter().take_while(|&&b| b == (n as u8 + b'A'));
                ((COL_LEN - correct.count()) * 10usize.pow(n as u32)) as i32
            })
            .sum()
    }

    /// Checks if all amphipods are home.
    fn done(&self, _: &Self::Env) -> bool {
        self.cols
            .iter()
            .enumerate()
            .all(|(i, col)| col.is_full() && col.iter().all(|&b| b == (i as u8 + b'A')))
    }
}

impl<const COL_LEN: usize> State<COL_LEN> {
    /// A list of every possible move for the current state, alongside the cost to reach it.
    fn moves(&self) -> impl Iterator<Item = (Move, i32)> + '_ {
        let c2r = (0..4).flat_map(|c| (0..7).map(move |r| Move(Index::Column(c), Index::Rest(r))));
        let r2c = (0..7).flat_map(|r| (0..4).map(move |c| Move(Index::Rest(r), Index::Column(c))));
        let c2c = (0..4).flat_map(|c| {
            (0..4)
                .filter(move |&d| c != d)
                .map(move |d| Move(Index::Column(c), Index::Column(d)))
        });

        c2r.chain(r2c)
            .chain(c2c)
            .filter_map(|m| self.cost(m).map(|cost| (m, cost as i32)))
    }

    /// Computes the energy cost of a move. Returns `None` if the move is not
    /// possible.
    fn cost(&self, Move(source, target): Move) -> Option<usize> {
        // Moves between rest spots are disallowed.
        if let (Index::Rest(_), Index::Rest(_)) = (source, target) {
            return None;
        }

        // Moves to taken spots are disallowed.
        match target {
            Index::Column(c) if self.cols[c].is_full() => return None,
            Index::Rest(r) if self.rest[r] != 0 => return None,
            _ => {}
        }

        // Retrieve the critter that actually moves.
        let mover = match source {
            Index::Column(c) => self.cols[c].last().copied(),
            Index::Rest(n) => Some(self.rest[n]),
        }?;

        // Get the energy cost multiplier; doubles as a check that there's
        // something actually moving.
        let multiplier = match mover {
            b'A' => 1,
            b'B' => 10,
            b'C' => 100,
            b'D' => 1000,
            _ => return None,
        };

        // Moves to columns other than sorted home are disallowed.
        let home_col = (mover - b'A') as usize;
        if let Index::Column(n) = target {
            if home_col != n || !self.cols[home_col].iter().all(|&m| m == mover) {
                return None;
            }
        }

        // Blocked moves are disallowed.
        let mut passed_spots = match (source, target) {
            (Index::Column(s), Index::Column(t)) => 2 + s.min(t)..2 + s.max(t),
            (Index::Column(s), Index::Rest(r)) => (r + 1).min(s + 2)..r.max(s + 2),
            (Index::Rest(r), Index::Column(t)) => (r + 1).min(t + 2)..r.max(t + 2),
            _ => return None,
        };
        if passed_spots.any(|n| self.rest[n] != 0) {
            return None;
        }

        // Calculate distance of move.
        const COL_TO_X: [usize; 4] = [2, 4, 6, 8];
        const REST_TO_X: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

        let distance = match (source, target) {
            (Index::Column(s), Index::Column(t)) => {
                let leave_start = 1 + COL_LEN - self.cols[s].len();
                let enter_end = COL_LEN - self.cols[t].len();
                let go_between = 2 * s.abs_diff(t);
                leave_start + go_between + enter_end
            }
            (Index::Column(s), Index::Rest(r)) => {
                let leave_start = 1 + COL_LEN - self.cols[s].len();
                let x_move = COL_TO_X[s].abs_diff(REST_TO_X[r]);
                leave_start + x_move
            }
            (Index::Rest(r), Index::Column(t)) => {
                let x_move = REST_TO_X[r].abs_diff(COL_TO_X[t]);
                let enter_end = COL_LEN - self.cols[t].len();
                x_move + enter_end
            }
            _ => return None,
        };

        Some(distance * multiplier)
    }
}

/// An index into a [`State`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Index {
    Column(usize),
    Rest(usize),
}

/// A move which transforms a [`State`] into a new one.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Move(Index, Index);

/// Parses the puzzle input.
fn parse(input: &str) -> State<2> {
    let text: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();

    let mut state = State::default();

    for x in 0..4 {
        state.cols[x].extend([text[3][3 + 2 * x], text[2][3 + 2 * x]]);
    }

    state
}

/// Parses the puzzle input using the extended rows from the puzzle description.
fn parse_extended(input: &str) -> State<4> {
    let text: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();

    let mut state = State::default();

    let e1 = b"DBAC";
    let e2 = b"DCBA";

    for x in 0..4 {
        state.cols[x].extend([text[3][3 + 2 * x], e1[x], e2[x], text[2][3 + 2 * x]]);
    }

    state
}
