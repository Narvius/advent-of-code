use std::collections::{HashMap, VecDeque};

/// Find the ID of the bot responsible for sorting 17 and 61 chips.
pub fn one(input: &str) -> Result<String, String> {
    let (held, _) = run_program(input);

    let pair = held
        .into_iter()
        .find(|(_, (a, b))| matches!([*a, *b], [Some(61), Some(17)] | [Some(17), Some(61)]));

    if let Some((id, _)) = pair {
        Ok(id.to_string())
    } else {
        Err("no matching bot found".into())
    }
}

/// Find the product of the first three outputs.
pub fn two(input: &str) -> Result<String, String> {
    let (_, output) = run_program(input);

    if let (Some(a), Some(b), Some(c)) = (output.get(&0), output.get(&1), output.get(&2)) {
        Ok((a * b * c).to_string())
    } else {
        Err("no values on required outputs".into())
    }
}

/// Maps bot IDs to held chips.
type HeldMap = HashMap<usize, (Option<usize>, Option<usize>)>;
/// Maps output IDs to values put in them.
type OutputMap = HashMap<usize, usize>;

/// Executes the instructions from the puzzle input and returns the final state.
fn run_program(input: &str) -> (HeldMap, OutputMap) {
    let mut queue = VecDeque::from_iter(parse(input));
    let mut held = HashMap::new();
    let mut output = HashMap::new();

    fn give(held: &mut HeldMap, output: &mut OutputMap, target: Target, value: usize) {
        match target {
            Target::Bot(bot) => {
                let entry = held.entry(bot).or_insert((None, None));
                *entry = (Some(value), entry.0);
            }
            Target::Output(slot) => {
                output.insert(slot, value);
            }
        }
    }

    while let Some(op) = queue.pop_front() {
        match op {
            Op::Value(v, n) => give(&mut held, &mut output, n, v),
            Op::Split(s, a, b) => {
                if let Some(&(Some(v1), Some(v2))) = held.get(&s) {
                    give(&mut held, &mut output, a, v1.min(v2));
                    give(&mut held, &mut output, b, v1.max(v2));
                } else {
                    queue.push_back(op);
                }
            }
        }
    }

    (held, output)
}

/// Parses the puzzle input into a sequence of instructions.
fn parse(input: &str) -> impl Iterator<Item = Op> + '_ {
    // Parses the `index`th and `index + 1`th entries into a `Target`.
    fn target(tokens: &[&str], index: usize) -> Option<Target> {
        let n = tokens[index + 1].parse().ok()?;

        Some(match tokens[index] {
            "bot" => Target::Bot(n),
            "output" => Target::Output(n),
            _ => None?,
        })
    }

    // Parses a single line.
    fn convert(line: &str) -> Option<Op> {
        let tokens: Vec<_> = line.split(' ').collect();
        Some(match tokens.len() {
            6 => Op::Value(tokens[1].parse().ok()?, target(&tokens, 4)?),
            12 => Op::Split(
                tokens[1].parse().ok()?,
                target(&tokens, 5)?,
                target(&tokens, 10)?,
            ),
            _ => None?,
        })
    }

    input.lines().filter_map(convert)
}

/// A single instruction from the puzzle input.
enum Op {
    Value(usize, Target),
    Split(usize, Target, Target),
}

/// A target to place a value into, as per the puzzle instructions.
#[derive(Copy, Clone)]
enum Target {
    Bot(usize),
    Output(usize),
}
