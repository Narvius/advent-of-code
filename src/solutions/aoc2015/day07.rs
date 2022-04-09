use std::collections::{HashMap, VecDeque};

/// Run the circuit once, get the value of wire "a".
pub fn one(input: &str) -> Result<String, String> {
    Ok(run_circuit(input, []).to_string())
}

/// Run the circuit twice; feeding the first wire "a" into the second wire "b", then get the value
/// of the second wire "a".
pub fn two(input: &str) -> Result<String, String> {
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
    let mut memory = HashMap::new();

    for (s, v) in initial {
        if let Some(s) = from_name(s) {
            memory.insert(s, v);
        }
    }

    while !pending.is_empty() {
        let instruction = pending.pop_front().unwrap();
        if execute(&mut memory, instruction).is_none() {
            pending.push_back(instruction);
        }
    }

    from_name("a")
        .and_then(|n| memory.get(&n).copied())
        .unwrap_or(0)
}

type Instruction<'a> = (&'a str, Val, Val, usize);

/// A value for use in instructions.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Val {
    Const(usize),
    Named(usize),
}

/// Parses a line of puzzle input.
fn parse<'a>(line: &str) -> Option<Instruction> {
    fn get_val(s: &str) -> Option<Val> {
        Some(if let Some(name) = from_name(s) {
            Val::Named(name)
        } else {
            Val::Const(s.parse().ok()?)
        })
    }

    let tokens: Vec<_> = line.split(' ').collect();
    Some(match tokens.len() {
        3 => (
            tokens[1],
            get_val(tokens[0])?,
            Val::Const(0),
            from_name(tokens[2])?,
        ),
        4 => (
            tokens[0],
            get_val(tokens[1])?,
            Val::Const(0),
            from_name(tokens[3])?,
        ),
        5 => (
            tokens[1],
            get_val(tokens[0])?,
            get_val(tokens[2])?,
            from_name(tokens[4])?,
        ),
        _ => None?,
    })
}

/// Converts a variable name from the input into a numeric representation.
fn from_name(s: &str) -> Option<usize> {
    Some(if s.bytes().all(|b| b'a' <= b && b <= b'z') {
        s.bytes().fold(0, |acc, b| (acc << 8) + b as usize)
    } else {
        None?
    })
}
