use std::collections::{HashMap, HashSet};

use crate::common::intcode::v2::*;

/// Run the program until it pauses, count the number of block-type tiles on screen.
pub fn one(input: &str) -> crate::Result<usize> {
    let mut screen = HashMap::new();
    let mut p = Program::with_capacity(input, 3000)?;

    p.run()?;
    while !p.output.is_empty() {
        let x = p.output.pop_front().ok_or("wrong amount of outputs")?;
        let y = p.output.pop_front().ok_or("wrong amount of outputs")?;
        let cell = p.output.pop_front().ok_or("wrong amount of outputs")?;

        *screen.entry((x as i32, y as i32)).or_default() = cell as u8;
    }

    Ok(screen.into_values().filter(|&v| v == 2).count())
}

/// Win the simulated pong game, return the score.
pub fn two(input: &str) -> crate::Result<Int> {
    let mut blocks = HashSet::new();
    let (mut paddle, mut ball, mut score) = (0, 0, 0);
    let mut p = Program::with_capacity(input, 3000)?;
    p.code[0] = 2;

    loop {
        p.run()?;
        while !p.output.is_empty() {
            let x = p.output.pop_front().ok_or("wrong amount of outputs")?;
            let y = p.output.pop_front().ok_or("wrong amount of outputs")?;
            let cell = p.output.pop_front().ok_or("wrong amount of outputs")?;

            match cell {
                0 => {
                    blocks.remove(&(x, y));
                }
                2 => {
                    blocks.insert((x, y));
                }
                3 => paddle = x,
                4 => ball = x,
                _ if (x, y) == (-1, 0) => score = cell,
                _ => {}
            }
        }
        if !blocks.is_empty() {
            p.input.push_back((ball - paddle).signum());
        } else {
            return Ok(score);
        }
    }
}
