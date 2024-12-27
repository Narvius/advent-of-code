use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

/// Find the result of running the program in the puzzle input; as a single number formed from the
/// bits in znn..z00.
pub fn one(input: &str) -> crate::Result<usize> {
    let (mut signals, ops) = parse(input);
    let mut ops: VecDeque<_> = ops.collect();

    while let Some((a, op, b, t, n)) = ops.pop_front() {
        if let (Some(a), Some(b)) = (signals.get(a), signals.get(b)) {
            signals.insert(
                t,
                match op {
                    "AND" => *a & *b,
                    "OR" => *a | *b,
                    "XOR" => *a ^ *b,
                    _ => return Err("invalid operation".into()),
                },
            );
        } else if n != signals.len() {
            ops.push_back((a, op, b, t, signals.len()));
        }
    }

    let mut signals: Vec<_> = signals
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .collect();
    signals.sort_unstable_by_key(|&(k, _)| std::cmp::Reverse(k));

    Ok(signals
        .into_iter()
        .fold(0, |acc, (_, v)| 2 * acc + usize::from(v)))
}

/// Find the four pairs of outputs that need to have their labels swapped in order to make the
/// input program a binary full adder.
pub fn two(input: &str) -> crate::Result<String> {
    let mut map = Map::new();
    for (a, op, b, t, _) in parse(input).1 {
        map.insert(t, (a, op, b));
    }

    let mut swapped = vec![];

    'outer: loop {
        let mut trees = HashMap::new();
        for &key in map.keys() {
            trees.insert(key, tree_for(&map, key));
        }
        'scan: for z in 2.. {
            // If we go beyond the highest `znn` available in the puzzle, we can stop.
            let key = format!("z{z:0>2}");
            if !map.contains_key(key.as_str()) {
                break 'outer;
            }

            // Find all differences between the tree and what it should be for `znn`, sorted from
            // deepest to most shallow.
            let tree = trees[key.as_str()].clone();
            let expected = expected_tree(z);
            let wrongs = mismatches(tree, expected);

            // Go through the list of mismatches, and try to find a different tree that matches the
            // expected sub-tree. If one is found (called `swap` here), physically swap them in the
            // `map`, and mark those two as found.
            for (label, expected) in wrongs {
                let swap = trees
                    .iter()
                    .find(|&(_, t)| mismatches(t.clone(), expected.clone()).is_empty());

                if let Some((swap, _)) = swap {
                    let t = map.remove(label).unwrap();
                    let t = map.insert(swap, t).unwrap();
                    map.insert(label, t);
                    swapped.extend([label.to_string(), swap.to_string()]);

                    break 'scan;
                }
            }
        }
    }
    swapped.sort();
    Ok(swapped.join(","))
}

/// Returns a list of mismatches between the two provided trees, sorted from deepest to shallowest
/// point of divergence. A `mismatch` is delivered as a pair of (label, expected tree).
fn mismatches(lhs: Rc<Tree<&str>>, rhs: Rc<Tree<()>>) -> Vec<Expect<'_>> {
    fn find_wrongs<'a>(
        l: Rc<Tree<&'a str>>,
        r: Rc<Tree<()>>,
        wrongs: &mut Vec<Expect<'a>>,
    ) -> bool {
        match (l.as_ref(), r.as_ref()) {
            (Tree::Gate(label, la, lop, lb), Tree::Gate(_, ra, rop, rb)) => {
                let mut subwrongs = vec![];
                let result = lop == rop
                    && ((find_wrongs(la.clone(), ra.clone(), &mut subwrongs)
                        && find_wrongs(lb.clone(), rb.clone(), &mut subwrongs))
                        || (find_wrongs(la.clone(), rb.clone(), &mut subwrongs)
                            && find_wrongs(lb.clone(), ra.clone(), &mut subwrongs)));

                if !result {
                    if subwrongs.is_empty() {
                        wrongs.push((label, Rc::clone(&r)));
                    } else {
                        wrongs.extend(subwrongs.pop());
                    }
                }
                result
            }
            (Tree::Leaf(a), Tree::Leaf(b)) => a == b,
            (Tree::Gate(label, _, _, _), _) => {
                wrongs.push((label, Rc::clone(&r)));
                false
            }
            _ => false,
        }
    }

    let mut wrongs = vec![];
    find_wrongs(lhs, rhs, &mut wrongs);
    wrongs
}

/// Produces the expected tree for a full adder for the `n`th least significant bit, without a
/// label.
fn expected_tree(n: usize) -> Rc<Tree<()>> {
    fn expected_carry(n: usize) -> Tree<()> {
        if n == 0 {
            Op::And.terminal(0)
        } else {
            Op::Or.gate(
                Op::And.terminal(n),
                Op::And.gate(Op::Xor.terminal(n), expected_carry(n - 1)),
            )
        }
    }

    Rc::new(Op::Xor.gate(Op::Xor.terminal(n), expected_carry(n - 1)))
}

/// The possible binary operations in the puzzle input.
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    /// A gate with two arbitrary subtrees.
    fn gate(self, a: Tree<()>, b: Tree<()>) -> Tree<()> {
        Tree::Gate((), Rc::new(a), self, Rc::new(b))
    }

    // A gate where both subexpressions are `Leaf(n)`.
    fn terminal(self, n: usize) -> Tree<()> {
        Tree::Gate((), Rc::new(Tree::Leaf(n)), self, Rc::new(Tree::Leaf(n)))
    }
}

/// A binary tree representing the full operation that goes into calculating one output.
#[derive(Debug)]
enum Tree<T> {
    /// A binary operation with arbitrary subexpresions.
    Gate(T, Rc<Tree<T>>, Op, Rc<Tree<T>>),
    /// A leaf node (an input labelled `xnn` or `ynn`). You never need to differentiate between `x`
    /// or `y` inputs, only the number matters; so we don't.
    Leaf(usize),
}

/// Produces the [`Tree`] for the output labelled `key`.
fn tree_for<'a>(map: &Map<'a>, key: &str) -> Rc<Tree<&'a str>> {
    Rc::new(if let Some((a, op, b)) = map.get(&key) {
        let op = match *op {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("bad op"),
        };
        let key = map.keys().find(|&&k| k == key).unwrap();
        Tree::Gate(key, tree_for(map, a), op, tree_for(map, b))
    } else {
        Tree::Leaf(key[1..].parse::<usize>().expect("a number"))
    })
}

type Expect<'a> = (&'a str, Rc<Tree<()>>);
type Map<'a> = HashMap<&'a str, (&'a str, &'a str, &'a str)>;
type Line<'a> = (&'a str, &'a str, &'a str, &'a str, usize);

/// Parses the puzzle input into a list of initial inputs and list of lines in the program.
fn parse(input: &str) -> (HashMap<&str, bool>, impl Iterator<Item = Line> + '_) {
    let (inputs, ops) = input.split_once("\n\n").expect("two sections");
    let inputs = inputs
        .lines()
        .filter_map(|line| {
            let (label, value) = line.split_once(": ")?;
            Some((label, value == "1"))
        })
        .collect();
    let ops = ops.lines().filter_map(|line| {
        let (lhs, rhs) = line.split_once(" -> ")?;
        let mut lhs = lhs.split_whitespace();
        Some((lhs.next()?, lhs.next()?, lhs.next()?, rhs, 0))
    });
    (inputs, ops)
}
