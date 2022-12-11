use std::collections::VecDeque;

/// Find the level of monkey business after 20 turns, whilst feeling relief after every
/// item inspection.
pub fn one(input: &str) -> crate::Result<usize> {
    monkey_business(input, 20, true)
}

/// Find the level of monkey business after 10000 turns, without feeling relief after every
/// item inspection.
pub fn two(input: &str) -> crate::Result<usize> {
    monkey_business(input, 10000, false)
}

/// Runs the algorithm described in the puzzle `turns` times for monkeys (parsed)[`parse`] from
/// `input`. If `relief_factor` is given, worry values are divided by it every time they are
/// inspected.
///
/// The result is product of the two highest inspect counts amongst all monkeys.
fn monkey_business(input: &str, turns: usize, relief_factor: bool) -> crate::Result<usize> {
    let mut monkeys = parse(input);

    // To avoid numbers from exploding when there is no relief, we instead take the worry
    // values modulo the product of all monkey tests. That way, no test is affected, but the
    // numbers stay in a smaller, predictable range.
    let (cap, relief) = match relief_factor {
        true => (i64::MAX, 3),
        false => (monkeys.iter().map(|m| m.test).product(), 1),
    };

    for _ in 0..turns {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                monkeys[i].inspect_count += 1;

                let BinOp(a, op, b) = monkeys[i].operation;
                item = (op(a.unwrap_or(item), b.unwrap_or(item)) / relief) % cap;

                let index = monkeys[i].targets[usize::from(item % monkeys[i].test == 0)];
                monkeys[index].items.push_back(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|m| usize::MAX - m.inspect_count);
    Ok(monkeys[0].inspect_count * monkeys[1].inspect_count)
}

/// A single monkey.
struct Monkey {
    items: VecDeque<i64>,
    operation: BinOp,
    test: i64,
    targets: [usize; 2],
    inspect_count: usize,
}

/// A binary expression involving two numbers. If a number is not given, the old item value is
/// used instead.
struct BinOp(Option<i64>, fn(i64, i64) -> i64, Option<i64>);

/// Parses the puzzle input into a list of monkeys.
fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("Monkey")
        .filter_map(|chunk| {
            let mut lines = chunk.lines().skip(1);
            let items = lines.next()?.split_once(": ")?.1;
            let operation = lines.next()?.split_once("= ")?.1;
            let test = lines.next()?.split_once("by ")?.1;
            let if_true = lines.next()?.split_once("monkey ")?.1;
            let if_false = lines.next()?.split_once("monkey ")?.1;

            let mut op = operation.split(' ');

            Some(Monkey {
                items: items.split(", ").filter_map(|v| v.parse().ok()).collect(),
                operation: BinOp(
                    op.next()?.parse().ok(),
                    if op.next()? == "*" {
                        <i64 as std::ops::Mul>::mul
                    } else {
                        <i64 as std::ops::Add>::add
                    },
                    op.next()?.parse().ok(),
                ),
                test: test.parse().ok()?,
                targets: [if_false.parse().ok()?, if_true.parse().ok()?],
                inspect_count: 0,
            })
        })
        .collect()
}
