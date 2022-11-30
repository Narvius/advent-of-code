/// Find the arrangement of the 9 input cups after 100 steps of the cup game; starting at (but
/// not including) cup #1.
pub fn one(input: &str) -> crate::Result<String> {
    let cups = run_cups(input.trim(), 9, 100).ok_or("failed cup game")?;

    let mut result = String::new();
    let mut pointer = 0;
    while cups[pointer] != 0 {
        result.push((b'1' + cups[pointer] as u8) as char);
        pointer = cups[pointer];
    }

    Ok(result)
}

/// Run the cup game for 1 million cups and 10 million steps, then find the product of the labels
/// of the two cups right after cup #1.
pub fn two(input: &str) -> crate::Result<usize> {
    let cups = run_cups(input.trim(), 1_000_000, 10_000_000).ok_or("failed cup game")?;
    Ok((cups[0] + 1) * (cups[cups[0]] + 1))
}

/// Runs the cup game with the given starting arrangement and number of cups, for the given
/// amount of steps.
fn run_cups(input: &str, cups: usize, steps: usize) -> Option<Vec<usize>> {
    // A "linked list" where nodes are labelled `n`, and `cups[n]` is the next pointer.
    let mut cups: Vec<_> = (1..=cups).collect();
    let mut current = (input.bytes().next()? - b'1') as usize;

    // Scramble the initial couple cups as described in the input.
    let input = input.as_bytes();
    let first_cup = (input[0] - b'1') as usize;
    let last_given_cup = (input.last().unwrap() - b'1') as usize;

    for w in input.windows(2) {
        cups[(w[0] - b'1') as usize] = (w[1] - b'1') as usize;
    }

    if input.len() < cups.len() {
        // There's extra cups after the initial ones. Link them into the list.
        cups[last_given_cup] = input.len();
        *cups.last_mut().unwrap() = first_cup;
    } else {
        // There's no extra cups. Just link the last cup to the first one.
        cups[last_given_cup] = first_cup;
    }

    for _ in 0..steps {
        // The next three cups.
        let (a, b, c) = (
            cups[current],
            cups[cups[current]],
            cups[cups[cups[current]]],
        );

        // Find the spot to put them.
        let mut target = (cups.len() + current - 1) % cups.len();
        while [a, b, c].contains(&target) {
            target = (cups.len() + target - 1) % cups.len();
        }

        let (pre_splice, to_move_first, to_move_last, post_splice) = (target, a, c, cups[target]);

        cups[current] = cups[to_move_last]; // Take out the 3 cups.
        cups[pre_splice] = to_move_first; // Append them to the target cup.
        cups[to_move_last] = post_splice; // Put the cup that was after the target cup after the spliced-in 3 cups.

        current = cups[current];
    }

    Some(cups)
}
