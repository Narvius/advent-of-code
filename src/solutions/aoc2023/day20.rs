use std::collections::{HashMap, VecDeque};

/// Find the total number of sent `low` and `high` signals in 1000 button presses.
pub fn one(input: &str) -> crate::Result<usize> {
    let (mut modules, broadcaster_id) = parse(input).ok_or("parse failed")?;
    let (mut lo, mut hi) = (0, 0);
    for _ in 0..1000 {
        let (added_lo, added_hi, _) = press_button(&mut modules, broadcaster_id);
        (lo, hi) = (lo + added_lo, hi + added_hi);
    }
    Ok(lo * hi)
}

/// Find the number of steps required for `rx` to receive a `low` input.
pub fn two(input: &str) -> crate::Result<usize> {
    let (mut modules, broadcaster_id) = parse(input).ok_or("parse failed")?;

    // The last module in the list is always `rx`. `rx` has one input, a conjunction; so for `rx`
    // to receive a low, all inputs to the conjunction must be high. Thus, we simply measure the
    // first time each input sends that conjunction a high, assume it's cyclical, and then simply
    // LCM those cyclical signal intervals as usual.
    let monitored_id = modules.last().unwrap().sources[0];
    let mut first_highs = vec![None; modules[monitored_id].sources.len()];
    let mut found = 0;

    for n in 1.. {
        let (_, _, highs) = press_button(&mut modules, broadcaster_id);
        for v in highs {
            if first_highs[v].is_none() {
                first_highs[v] = Some(n);
                found += 1;
            }
        }

        if found == first_highs.len() {
            let vals = first_highs.into_iter().flatten();
            return vals.reduce(crate::common::lcm).ok_or("no result".into());
        }
    }

    unreachable!()
}

/// Makes the broadcaster module send a low signal to all its targets, then resolves all resulting
/// signals. Returns the total amount of low signals sent, total amount of high signals sent, and a
/// list of all final conjunction inputs that received a high signal at any point.
fn press_button(modules: &mut [Module], broadcaster_id: usize) -> (usize, usize, Vec<usize>) {
    let mut queue = VecDeque::from([(broadcaster_id, broadcaster_id, false)]);
    let (mut lo, mut hi) = (0, 0);
    let mut monitored_highs = vec![];

    while let Some((from, to, high)) = queue.pop_front() {
        match high {
            false => lo += 1,
            true => hi += 1,
        }

        let output = match (&modules[to].module_type, high) {
            (ModuleType::Special, signal) => Some(signal),
            (ModuleType::FlipFlop, false) => {
                modules[to].state = 1 - modules[to].state;
                Some(modules[to].state == 1)
            }
            (ModuleType::Conjunction, signal) => {
                let p = modules[to]
                    .sources
                    .iter()
                    .position(|&i| i == from)
                    .expect("input from non-input module");
                let mask = 1 << p as u32;
                if signal {
                    modules[to].state |= mask;
                } else {
                    modules[to].state &= !mask;
                }

                // Monitor the final conjunction.
                if high && modules[to].targets == [modules.len() - 1] {
                    monitored_highs.push(p);
                }

                Some(modules[to].state.count_ones() as usize != modules[to].sources.len())
            }
            _ => None,
        };

        if let Some(signal) = output {
            for &target in &modules[to].targets {
                queue.push_back((to, target, signal));
            }
        }
    }

    (lo, hi, monitored_highs)
}

/// A single module from the input.
#[derive(Default)]
struct Module {
    module_type: ModuleType,
    state: u32,
    sources: Vec<usize>,
    targets: Vec<usize>,
}

/// The type of a [`Module`].
#[derive(Copy, Clone, Default)]
enum ModuleType {
    #[default]
    Special,
    FlipFlop,
    Conjunction,
}

/// Parses the puzzle input into a list of [`Module`]s and the index of the broadcaster module.
fn parse(input: &str) -> Option<(Vec<Module>, usize)> {
    let mut broadcaster_id = 0;
    let mut modules = vec![];
    let mut conns = vec![];
    let mut order = HashMap::new();

    for line in input.lines() {
        let (module, targets) = line.split_once(" -> ")?;
        let (name, module_type) = match module {
            name if name.starts_with('%') => (&name[1..], ModuleType::FlipFlop),
            name if name.starts_with('&') => (&name[1..], ModuleType::Conjunction),
            "broadcaster" => {
                broadcaster_id = modules.len();
                (module, ModuleType::Special)
            }
            name => (name, ModuleType::Special),
        };

        for target in targets.split(", ") {
            conns.push((name, target));
        }

        order.insert(name, modules.len());
        modules.push(Module {
            module_type,
            ..Default::default()
        });
    }

    for (src, tgt) in conns {
        if !order.contains_key(tgt) {
            order.insert(tgt, modules.len());
            modules.push(Module {
                module_type: ModuleType::Special,
                ..Default::default()
            });
        }
        modules[order[src]].targets.push(order[tgt]);
        modules[order[tgt]].sources.push(order[src]);
    }

    Some((modules, broadcaster_id))
}
