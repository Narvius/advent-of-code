use std::io::{stdin, stdout, Write};

use crate::common::intcode::v2::*;

// Transcript from me doing it interactively.
// ...the first one where I didn't die, anyway, lmao.
//
// For me, the solution was:
// - food ration
// - antenna
// - semiconductor
// - monolith
const SCRIPT: &str = include_str!("../../data/aoc2019/day25_record.txt");

pub fn one(input: &str) -> crate::Result<String> {
    let mut p = Program::with_capacity(input, 5200)?;
    for line in SCRIPT.lines() {
        p.output.clear();
        p.run_with(line.bytes().map(Int::from).chain(Some(10)))?;
    }

    Ok(p.output
        .into_iter()
        .map(|c| c as u8 as char)
        .filter(|c| c.is_numeric())
        .collect())
}

/// Freebie!
pub fn two(_input: &str) -> crate::Result<&str> {
    Ok("done!")
}

/// Exploring this text adventure manually was definitely the right call!
#[allow(unused)]
fn interactive(input: &str) -> crate::Result<()> {
    let mut p = Program::with_capacity(input, 5200)?;
    p.run()?;
    let mut record = vec![];
    let mut line = String::new();

    loop {
        for byte in p.output.drain(..) {
            print!("{}", byte as u8 as char);
        }

        print!("> ");
        stdout().flush()?;
        line.clear();
        stdin().read_line(&mut line)?;

        if line.starts_with("exit") {
            let text = record.join("\n");
            std::fs::write("record.txt", text)?;
            return Ok(());
        }

        p.run_with(line.trim().bytes().map(Int::from).chain(Some(10)))?;
        record.push(line.trim().to_string());
    }
}
