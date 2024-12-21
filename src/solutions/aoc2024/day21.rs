use std::{
    collections::HashMap,
    iter::{once, repeat},
};

use crate::common::{permutations_of, product};

/// Find the total complexity of inputting all codes with 2 additional robots between you and the
/// final door robot.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(total_complexity(input, 2))
}

/// Find the total complexity of inputting all codes with 25 additional robots between you and the
/// final door robot.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(total_complexity(input, 25))
}

/// Find the total complexity of all codes, given `robot_count` robots between you and the final
/// door robot.
fn total_complexity(input: &str, robot_count: usize) -> usize {
    let (num, dir, codes) = parse(input);
    let movement = find_best_movement(&num, &dir, robot_count);

    codes
        .map(|(multiplier, code)| {
            multiplier * {
                let (mut result, mut pos) = (0, 'A');
                for c in code.chars() {
                    result += 1 + movement[&(robot_count, pos, c)];
                    pos = c;
                }
                result
            }
        })
        .sum()
}

/// Returns a `Movement` map, which can be used to efficiently solved the puzzle.
///
/// `num` is a [key => position] mapping for the numeric keypad, `dir` is the same for the
/// directional keypad. `robot_count` is the amount of *additional* robots between you and and the
/// final door robot.
///
/// # Concept
///
/// Let's call every robot pushing buttons on a keypad a "layer". In part 1, we have three
/// layers--the first extra robot, the second extra robot, and the final door robot.
///
/// A key insight is that whenever a layer wants to press a button, the layer before it needs to
/// 1) press some combination of directions
/// 2) go back to the 'A' button
/// 3) press it
///
/// Crucial here is 2): Every previous layer is ALWAYS on 'A', because they start on 'A' before
/// your current layer presses anything, and is back on 'A' when they do. This greatly simplifies
/// matters, because we no longer need to care about any specific paths, only their lengths.
///
/// For example, if extra robot 1 (layer 0) wants to press the '<' button, the manual inputs
/// required to do that are "v<<" (three inputs) plus another one to make it press it, for a total
/// of four. Going back to 'A' would require the inputs ">>^", also three.
///
/// So we have two `Movement` map entries:
/// - `(0, 'A', '<') = 3`
/// - `(0, '<', 'A') = 3`
///
/// Why are we going back to A? Because whenever the robot wants to press a button, we have to go
/// back to A to do that. And the only reason to move is because the next robot in line needs to
/// press a button--therefore we'll always start from 'A', press some buttons required to navigate
/// it to what it wants to press, go back to 'A' to press it, so the next robot presses what they
/// wanted.
///
/// The trick now is that because after every `layer` button press, all previous layers are always
/// at 'A', we can use previous layer data to build data for the next layer without having to care
/// what the exact path was--because we're always assuming start at 'A' and end at 'A'! The length
/// is enough!
///
/// For example, let's say layer 1 is currently on the '>' button, but wants to press '<'. It would
/// have to get moved to the left twice--which means that layer 0 has to output '<' twice, and then
/// 'A' once.
///
/// So, in terms of manual inputs, we have to:
/// - move layer 0 to the '<' button
/// - press 'A' twice
/// - move layer 0 back to the 'A' button
/// - press 'A' once
///
/// Because we already have a full `Movement` map for layer 0, we know that that takes 3 inputs to
/// go from 'A' to '<', and another three to move back to 'A'.
///
/// So layer 1 going from '>' to '<' takes 8 manual inputs--plus one more to actually output it.
///
/// Thus we have another `Movement` map entry:
/// - `(1, '>', '<') = 8`
///
/// Just repeat this for all combinations of start and end buttons, always taking the path that
/// uses the fewest manual inputs (found by simply trying all possible paths).
///
/// And then in the end, the entries for layer `robot_count` are the cost (in terms of manual
/// inputs) of moving between keys on the final numerical keypad, at which point you can easily
/// find the cost of inputting codes.
fn find_best_movement(num: &Keypad, dir: &Keypad, robot_count: usize) -> Movement {
    let mut m = Movement::new();

    // For layer 0 buttons (manual input), it doesn't matter what order we press buttons in--since
    // nothing further down the line depends on the order. Thus we simply count the total amount of
    // presses required.
    m.extend(
        product(dir.keys().copied(), dir.keys().copied())
            .filter(|&(s, t)| s != ' ' && t != ' ')
            .map(|(s, t)| {
                let ((sx, sy), (tx, ty)) = (dir[&s], dir[&t]);
                ((0, s, t), sx.abs_diff(tx) + sy.abs_diff(ty))
            }),
    );

    // All further robots try to find the shortest combinations they can for themselves, using the
    // previous layers.
    for layer in 1..robot_count {
        find_layer(&mut m, dir, layer, (0, 0));
    }
    find_layer(&mut m, num, robot_count, (0, 3));

    m
}

/// Given a `Movement` map completed for layers 0 through `layer - 1`, expands it by another layer;
/// assuming the robot for that layer types on `keypad`. See the documentation for
/// `find_best_movement` for more information.
///
/// `empty` is a convenience parameter--it denotes the missing button that would make the provided
/// `keypad` a rectangle. Makes it easier to detect invalid paths (since robots are not allowed to
/// ever point to a non-button).
fn find_layer(movement: &mut Movement, keypad: &Keypad, layer: usize, empty: (usize, usize)) {
    for (s, t) in product(keypad.keys().copied(), keypad.keys().copied()) {
        // For an identical source and target, the previous pusher just needs to hit 'A' once.
        if s == t {
            movement.insert((layer, s, t), 0);
            continue;
        }

        // Simply try all permutations of the path possible.
        let ((sx, sy), (tx, ty)) = (keypad[&s], keypad[&t]);
        let base: Vec<char> = (repeat('<').take(sx.saturating_sub(tx)))
            .chain(repeat('^').take(sy.saturating_sub(ty)))
            .chain(repeat('>').take(tx.saturating_sub(sx)))
            .chain(repeat('v').take(ty.saturating_sub(sy)))
            .collect();

        let best_cost = (permutations_of(base).into_iter())
            .filter_map(|path| {
                // We have to reject all paths that route through the empty tile.
                let (mut x, mut y) = (sx, sy);
                for &c in &path {
                    match c {
                        '<' => x -= 1,
                        '^' => y -= 1,
                        '>' => x += 1,
                        'v' => y += 1,
                        _ => {}
                    };

                    if (x, y) == empty {
                        return None;
                    }
                }

                // Now calculate the total cost. We have to leave the previous layer position at
                // 'A', so we also add cost[pos -> 'A'].
                let (mut cost, mut pos) = (0, 'A');
                for &c in path.iter().chain(once(&'A')) {
                    cost += movement[&(layer - 1, pos, c)] + 1;
                    pos = c;
                }
                Some(cost - 1)
            })
            .min()
            .expect("at least one path");

        movement.insert((layer, s, t), best_cost);
    }
}

type V2 = (usize, usize);
type Keypad = HashMap<char, V2>;
type Movement = HashMap<(usize, char, char), usize>;

/// Returns the layouts of both relevant keypads, as well as the codes alongside their complexity
/// multiplier.
fn parse(input: &str) -> (Keypad, Keypad, impl Iterator<Item = (usize, &str)>) {
    let num_keyboard = HashMap::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);

    let dir_keypad = HashMap::from([
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    (
        num_keyboard,
        dir_keypad,
        input
            .lines()
            .map(|s| (s.trim_matches('A').parse().expect("a number"), s)),
    )
}
