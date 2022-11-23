/// Find the Bingo board that wins first, and find its score at the time of the win.
pub fn one(input: &str) -> crate::Result<i32> {
    let mut bingo = Bingo::from_input(input)?;
    while let Some(number) = bingo.draw_number() {
        if let Some(board) = bingo.take_finished_board() {
            return Ok(board.iter().filter_map(|&f| f).sum::<i32>() * number);
        }
    }

    Err("unreachable".into())
}

/// Find the Bingo board that wins last, and find its score at the time of the win.
pub fn two(input: &str) -> crate::Result<i32> {
    let mut bingo = Bingo::from_input(input)?;
    while let Some(number) = bingo.draw_number() {
        while let Some(board) = bingo.take_finished_board() {
            if bingo.cells.is_empty() {
                return Ok(board.iter().filter_map(|&f| f).sum::<i32>() * number);
            }
        }
    }

    Err("unreachable".into())
}

/// Represents the whole Bingo game described by `input`.
struct Bingo {
    draws: Vec<i32>,
    cells: Vec<Option<i32>>,
}

impl Bingo {
    /// Parses `input` into a `Bingo` game.
    fn from_input(input: &str) -> crate::Result<Bingo> {
        let mut tokens = input.split_whitespace();
        let draws = tokens.next().ok_or("no input")?;

        Ok(Bingo {
            draws: draws
                .split(',')
                .filter_map(|v| v.parse().ok())
                .rev()
                .collect(),
            cells: tokens.map(|v| v.parse().ok()).collect(),
        })
    }

    /// Draws a number, marks it off on all boards, and returns it. If there are no more numbers to
    /// be drawn, returns `None`.
    fn draw_number(&mut self) -> Option<i32> {
        self.draws.pop().map(|number| {
            for cell in &mut self.cells {
                if *cell == Some(number) {
                    *cell = None;
                }
            }

            number
        })
    }

    /// Returns the contents of a finished board, and removes it from the list of boards.
    fn take_finished_board(&mut self) -> Option<Vec<Option<i32>>> {
        (0..self.cells.len() / 25)
            .position(|board| {
                let marked = |n: usize| self.cells[25 * board + n].is_none();

                (0..5).any(|d| {
                    let horizontal = (0..5).map(|i| i + 5 * d).all(marked);
                    let vertical = (0..5).map(|i| 5 * i + d).all(marked);
                    horizontal || vertical
                })
            })
            .map(|board| self.cells.drain((board * 25)..(25 + board * 25)).collect())
    }
}
