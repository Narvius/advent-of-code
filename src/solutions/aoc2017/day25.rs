use std::collections::HashSet;

/// Run the state machine as per the specs in the puzzle input, and return the "diagnostic
/// checksum" (number of 1 values of the tape after execution).
pub fn one(input: &str) -> crate::Result<usize> {
    parse(input)
        .ok_or_else(|| "failed parse".into())
        .map(|m| m.get_checksum())
}

/// Freebie!
pub fn two(_input: &str) -> crate::Result<&'static str> {
    Ok("done!")
}

/// Whether to write 1 or 0; whether to turn right or left; and next state, respectively.
type Instruction = (bool, bool, usize);

/// Specifications for a state machine as described in the puzzle input.
struct Machine {
    states: Vec<(Instruction, Instruction)>,
    starting_state: usize,
    steps_for_diagnostic: usize,
}

impl Machine {
    /// Runs the state machine until completion and gets the checksum (total amount of 1s
    /// on the tape).
    fn get_checksum(&self) -> usize {
        let mut set = HashSet::new();
        let mut position = 0;
        let mut state = self.starting_state;

        for _ in 0..self.steps_for_diagnostic {
            let (write, dir, next) = if set.contains(&position) {
                self.states[state].1
            } else {
                self.states[state].0
            };

            if write {
                set.insert(position);
            } else {
                set.remove(&position);
            }

            position += if dir { 1 } else { -1 };
            state = next;
        }

        set.len()
    }
}

/// Parses the puzzle input into specifications for a state machine.
fn parse(s: &str) -> Option<Machine> {
    let mut lines = s.lines();

    let starting_state = (lines.next()?.split(' ').last()?.as_bytes()[0] - b'A') as usize;
    let steps_for_diagnostic = lines
        .next()?
        .split(' ')
        .filter_map(|t| t.parse::<usize>().ok())
        .next()?;
    let mut states = vec![];

    while let Some(_) = lines.next() {
        lines.next();
        lines.next();
        let write0 = lines.next()?.split(' ').last()? == "1.";
        let dir0 = lines.next()?.split(' ').last()? == "right.";
        let next0 = lines.next()?.split(' ').last()?.as_bytes()[0];
        lines.next();
        let write1 = lines.next()?.split(' ').last()? == "1.";
        let dir1 = lines.next()?.split(' ').last()? == "right.";
        let next1 = lines.next()?.split(' ').last()?.as_bytes()[0];
        states.push((
            (write0, dir0, (next0 - b'A') as usize),
            (write1, dir1, (next1 - b'A') as usize),
        ));
    }

    Some(Machine {
        states,
        starting_state,
        steps_for_diagnostic,
    })
}
