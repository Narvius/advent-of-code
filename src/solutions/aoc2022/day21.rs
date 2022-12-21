use std::collections::{HashMap, VecDeque};

/// Resolve the tree of monkey math operations and return the number computed by `"root"`.
pub fn one(input: &str) -> crate::Result<i128> {
    let mut queue = VecDeque::new();
    let mut shouted = HashMap::new();
    let mut monkeys = parse(input);

    while let Some((name, op)) = monkeys.next().or_else(|| queue.pop_front()) {
        match op {
            Op::Const(n) => {
                shouted.insert(name, n);
            }
            Op::Bin(a, f, b) => {
                if let (Some(&lhs), Some(&rhs)) = (shouted.get(a), shouted.get(b)) {
                    shouted.insert(name, (get_op(f))?(lhs, rhs));
                } else {
                    queue.push_back((name, op));
                }
            }
        }
    }

    Ok(shouted["root"])
}

/// Resolve the tree of monkey math, but treat `"humn"` as a variable in an equation, and
/// `"root"`s operation as a comparison. Solve for the variable.
pub fn two(input: &str) -> crate::Result<i128> {
    // The first block is similar to part 1, but `shouted` now contains a second value; a bool
    // saying whether this value has the "humn" variable in it. If something does, we save
    // the operation being done to it. In the end, `operations` contains a list of operations
    // done to "humn" to arrive at the value it is being compared to.

    let mut queue = VecDeque::new();
    let mut shouted = HashMap::new();
    let mut monkeys = parse(input);
    let mut operations = vec![];
    let mut target = 0;

    // Find the list of operations we need to do to `x`, and the target number.
    while let Some((name, op)) = monkeys.next().or_else(|| queue.pop_front()) {
        match op {
            Op::Const(n) => {
                shouted.insert(name, (n, name == "humn"));
            }
            Op::Bin(a, f, b) => match (shouted.get(a), shouted.get(b)) {
                (Some(&lhs), Some(&rhs)) if name == "root" => {
                    target = if lhs.1 { rhs.0 } else { lhs.0 };
                }
                (Some(&lhs), Some(&rhs)) => {
                    if lhs.1 || rhs.1 {
                        operations.push(((!lhs.1).then_some(lhs.0), f, (!rhs.1).then_some(rhs.0)));
                    }
                    shouted.insert(name, ((get_op(f))?(lhs.0, rhs.0), lhs.1 || rhs.1));
                }
                _ => queue.push_back((name, op)),
            },
        }
    }

    // Apply inverse operations to the target number in reverse order, to solve for the
    // variable.
    Ok(operations
        .into_iter()
        .rev()
        .fold(target, |acc, op| match op {
            (None, "+", Some(b)) => acc - b,
            (Some(a), "+", None) => acc - a,
            (None, "-", Some(b)) => acc + b,
            (Some(a), "-", None) => a - acc,
            (None, "*", Some(b)) => acc / b,
            (Some(a), "*", None) => acc / a,
            (None, "/", Some(b)) => acc * b,
            _ => unreachable!(),
        }))
}

/// Converts a single-character string into the corresponding basic math operation.
fn get_op(op: &str) -> crate::Result<fn(i128, i128) -> i128> {
    Ok(match op {
        "+" => <i128 as std::ops::Add>::add,
        "-" => <i128 as std::ops::Sub>::sub,
        "*" => <i128 as std::ops::Mul>::mul,
        "/" => <i128 as std::ops::Div>::div,
        _ => Err("unknown operation")?,
    })
}

/// A single monkey from the input, composed of a name and a mathematical expression.
type Monkey<'a> = (&'a str, Op<'a>);

/// Represents a mathematical expression that can be held by a monkey. It is either a constant,
/// or the result of applying a basic mathematical operation to the results from two other
/// monkeys.
enum Op<'a> {
    Const(i128),
    Bin(&'a str, &'a str, &'a str),
}

/// Parses the puzzle input into a list of monkeys.
fn parse(input: &str) -> impl Iterator<Item = Monkey> + '_ {
    input.lines().filter_map(|line| {
        let (name, op) = line.split_once(": ")?;
        let op: Vec<_> = op.split_ascii_whitespace().collect();
        Some(if op.len() == 1 {
            (name, Op::Const(op[0].parse().ok()?))
        } else {
            (name, Op::Bin(op[0], op[1], op[2]))
        })
    })
}
