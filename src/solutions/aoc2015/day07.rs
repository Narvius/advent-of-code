use std::collections::{HashMap, VecDeque};

/// Run the circuit once, get the value of wire "a".
pub fn one(input: &str) -> crate::Result<String> {
    Ok(run_circuit(input, []).to_string())
}

/// Run the circuit twice; feeding the first wire "a" into the second wire "b", then get the value
/// of the second wire "a".
pub fn two(input: &str) -> crate::Result<String> {
    let a = run_circuit(input, []);
    Ok(run_circuit(input, [("b", a)]).to_string())
}

/// Runs the circuit describes by the puzzle input, and returns the value of wire "a".
fn run_circuit(input: &str, initial: impl IntoIterator<Item = (&'static str, usize)>) -> usize {
    // Resolves a Val to a value.
    fn resolve(memory: &mut HashMap<usize, usize>, val: Val) -> Option<usize> {
        Some(match val {
            Val::Const(c) => c,
            Val::Named(n) => memory.get(&n).copied()?,
        })
    }

    // Runs a single instruction if possible, and returns Some(()) if successful, None otherwise.
    // Does not overwrite values if they're already in memory under the same name.
    fn execute(memory: &mut HashMap<usize, usize>, instruction: Instruction) -> Option<()> {
        let (command, v1, v2, t) = instruction;
        let (v1, v2) = (resolve(memory, v1)?, resolve(memory, v2)?);
        memory.entry(t).or_insert(match command {
            "->" => v1,
            "NOT" => !v1,
            "AND" => v1 & v2,
            "OR" => v1 | v2,
            "LSHIFT" => v1 << v2,
            "RSHIFT" => v1 >> v2,
            _ => unreachable!(),
        });

        Some(())
    }

    let mut pending: VecDeque<_> = input.lines().filter_map(parse).collect();
    let mut memory = HashMap::from_iter(
        initial
            .into_iter()
            .filter_map(|(s, v)| Some((as_number(s)?, v))),
    );

    while !pending.is_empty() {
        let instruction = pending.pop_front().unwrap();
        if execute(&mut memory, instruction).is_none() {
            pending.push_back(instruction);
        }
    }

    memory.get(&as_number("a").unwrap()).copied().unwrap_or(0)
}

type Instruction<'a> = (&'a str, Val, Val, usize);

/// A value for use in instructions.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Val {
    Const(usize),
    Named(usize),
}

/// Parses a line of puzzle input.
fn parse(line: &str) -> Option<Instruction> {
    fn val(s: &str) -> Option<Val> {
        Some(match as_number(s) {
            Some(name) => Val::Named(name),
            None => Val::Const(s.parse().ok()?),
        })
    }

    let line: Vec<_> = line.split(' ').collect();
    Some(match line.len() {
        3 => (line[1], val(line[0])?, Val::Const(0), as_number(line[2])?),
        4 => (line[0], val(line[1])?, Val::Const(0), as_number(line[3])?),
        5 => (line[1], val(line[0])?, val(line[2])?, as_number(line[4])?),
        _ => None?,
    })
}

/// Converts a variable name from the input into a numeric representation.
fn as_number(s: &str) -> Option<usize> {
    Some(if s.bytes().all(|b| (b'a'..=b'z').contains(&b)) {
        s.bytes().fold(0, |acc, b| (acc << 8) + b as usize)
    } else {
        None?
    })
}
