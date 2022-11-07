use std::collections::{HashMap, VecDeque};

/// Find the number of highly-ambiguous samples (matching 3 or more operations).
pub fn one(input: &str) -> crate::Result<usize> {
    // Checks whether a sample is highly ambiguous (3 or more operations match).
    fn ambiguous(sample: &Sample) -> bool {
        OPS.iter().filter(|&&op| matches(op, *sample)).count() >= 3
    }

    let (runs, _) = parse(input).ok_or("failed parse")?;
    Ok(runs.into_iter().filter(ambiguous).count())
}

/// Identify all opcodes, run the test program, and get the output.
pub fn two(input: &str) -> crate::Result<i32> {
    let (samples, program) = parse(input).ok_or("failed parse")?;

    let mapping = discover_opcodes(samples);
    let mut reg = [0; 4];

    for command in program {
        eval(mapping[command[0] as usize], &command, &mut reg);
    }

    Ok(reg[0])
}

type Sample = ([i32; 4], [i32; 4], [i32; 4]);
type Command = [i32; 4];
type Op = fn(i32, i32, i32, i32) -> i32;

/// Produces a re-sorted operation table, such that they match the actual opcodes.
fn discover_opcodes(samples: Vec<Sample>) -> [Op; 16] {
    // Run all combinations of ops and samples, producing a map of (opcode => outcomes).
    let mut match_tables = HashMap::new();
    for sample in samples {
        let mut op_matches = vec![];
        for op in OPS.iter() {
            op_matches.push(matches(*op, sample));
        }
        match_tables
            .entry(sample.1[0])
            .or_insert_with(Vec::new)
            .push(op_matches);
    }

    // For each remaining unmapped operation, find all opcodes that it could match. If it
    // only matchdes one, we've found a mapping; if it matches multiple, we try a different
    // operation. Keep doing that until done.
    let mut result = [OPS[0]; 16];
    let mut ops: VecDeque<_> = (0..16).collect();

    while let Some(i) = ops.pop_front() {
        let matching_tables: Vec<_> = match_tables
            .iter()
            .filter(|table| table.1.iter().all(|t| t[i]))
            .collect();

        if matching_tables.len() == 1 {
            let key = *matching_tables[0].0;
            result[key as usize] = OPS[i];
            match_tables.remove(&key);
        } else {
            ops.push_back(i);
        }
    }

    result
}

/// Checks whether an operation behaves like a given sample.
fn matches(op: Op, (mut before, args, after): Sample) -> bool {
    eval(op, &args, &mut before);
    before == after
}

/// Executes an operation with the given arguments and
fn eval(op: Op, args: &[i32], reg: &mut [i32]) {
    let ra = reg.get(args[1] as usize).copied().unwrap_or(i32::MAX);
    let rb = reg.get(args[2] as usize).copied().unwrap_or(i32::MAX);

    reg[args[3] as usize] = op(ra, args[1], rb, args[2]);
}

/// Parses the puzzle input into a list of samples and the test program.
fn parse(input: &str) -> Option<(Vec<Sample>, Vec<Command>)> {
    let lines: Vec<_> = input.lines().collect();

    fn as_array(numbers: &str, separator: &str) -> Option<[i32; 4]> {
        numbers
            .split(separator)
            .filter_map(|v| v.parse().ok())
            .collect::<Vec<_>>()
            .try_into()
            .ok()
    }

    let mut i = 0;
    let mut samples = vec![];
    while lines[i].starts_with("Before: ") {
        let before = &lines[i][9..19];
        let after = &lines[i + 2][9..19];
        samples.push((
            as_array(before, ", ")?,
            as_array(lines[i + 1], " ")?,
            as_array(after, ", ")?,
        ));
        i += 4;
    }

    let commands = lines[i + 2..]
        .iter()
        .filter_map(|v| as_array(v, " "))
        .collect();

    Some((samples, commands))
}

/// A list of all available operations. Each operation takes all four arguments (in the order of
/// Register A, Value A, Register B, Value B), but only uses the ones that are relevant to it.
const OPS: [Op; 16] = [
    |ra, _, rb, _| ra + rb,
    |ra, _, _, vb| ra + vb,
    |ra, _, rb, _| ra * rb,
    |ra, _, _, vb| ra * vb,
    |ra, _, rb, _| ra & rb,
    |ra, _, _, vb| ra & vb,
    |ra, _, rb, _| ra | rb,
    |ra, _, _, vb| ra | vb,
    |ra, _, _, _| ra,
    |_, va, _, _| va,
    |_, va, rb, _| if va > rb { 1 } else { 0 },
    |ra, _, _, vb| if ra > vb { 1 } else { 0 },
    |ra, _, rb, _| if ra > rb { 1 } else { 0 },
    |_, va, rb, _| if va == rb { 1 } else { 0 },
    |ra, _, _, vb| if ra == vb { 1 } else { 0 },
    |ra, _, rb, _| if ra == rb { 1 } else { 0 },
];
