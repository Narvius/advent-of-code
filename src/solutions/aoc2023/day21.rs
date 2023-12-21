use std::collections::{HashSet, VecDeque};

/// Find the number of tiles the elf could end on after 64 steps.
///
/// Leaning on a special flood fill, see [`floodfill`].
pub fn one(input: &str) -> crate::Result<usize> {
    let map: Vec<_> = input.lines().map(str::as_bytes).collect();
    let start = (0..map.len())
        .flat_map(|y| (0..map[y].len()).map(move |x| (x, y)))
        .find(|&(x, y)| map[y][x] == b'S')
        .ok_or("no start")?;

    Ok(floodfill(&map, &[start], 64))
}

/// Given an infinitely-wrapping map and a lot of steps, find the number of tiles the elf could end
/// up on.
pub fn two(input: &str) -> crate::Result<usize> {
    // PARITY
    //
    // I call a map "even" or "odd" depending on whether the `S` tile is a final square or not. A
    // map is even if it is, and odd if it is not. Note that the `S` tile is always exactly in the
    // center.
    //
    // If we did this on a 3x3 map with no obstacles whatsoever, it might look like this:
    //
    // +---+---+
    // |O O| O |
    // | O |O O|
    // |O O| O |
    // +---+---+
    //
    // The left map is even, the right map is odd.
    //
    // ALTERNATING PARITY
    //
    // As is visible on the example, if each individual map is a square with odd side lengths,
    // neighbouring maps have opposite parity. The input map is 131x131, so it fulfills these
    // requirements.
    //
    // Therefore, neighbouring maps will always alternate being odd and even. Because the step
    // count in part 2 is odd, the center map is an odd map, which means that the first connected
    // map in each direction is even, and so on.
    //
    // OPTIMAL WAY OF ENTERING ANY SUB-MAP
    //
    // A crucial observation is that our map has a fully empty + running down the middle of the
    // map, and empty borders. This means we can freely navigate from any edge point to the center
    // and vice versa without any obstacles.
    //
    // Let's draw a slightly larger example; with map size 3x3 and 2 * 3 + (3/2) = 7 steps:
    //
    // +---+---+
    // | O |   |
    // |O O|   |
    // | O |O  |
    // +---+---+---+
    // |O O| O |   |
    // | O |O O|   |
    // |O O| O |O  |
    // +---+---+---+
    // | O |O O| O |
    // |OSO| O |O O|
    // | O |O O| O |
    // +---+---+---+
    // |O O| O |O  |
    // | O |O O|   |
    // |O O| O |   |
    // +---+---+---+
    // | O |O  |
    // |O O|   |
    // | O |   |
    // +---+---+
    //
    // No matter where we're going, the optimal spot to floodfill each sub-map from is the point
    // CLOSEST to S. For maps on the main +, that's the midpoint of the closest edge; for
    // everything else it's the closest corner.
    //
    // SOLUTION CONCEPT
    //
    // Due to the orthogonal-only steps, the final supermap will have a diamond-shaped fill,
    // with all maps but the outermost two layers being completely filled. We can categorize and
    // count each type of sub-map, because they repeat in a pattern.
    //
    // There are three differentiating factors of maps:
    // - parity (2: odd, even)
    // - direction entered from (9: each corner, each midpoint, the center)
    // - available steps (enough to fill all, or a number less than required to fill all)
    //
    // Leading to roughly 30-40 categories of map we have to count and consider. Then, floodfill
    // each type once, multiply by the amount of that type of map we have, sum all of it. We have
    // our result.
    //
    // The amount of steps is of the shape `2n * map_size + map_size / 2`, which is relevant,
    // because that means that going in a straight line in any direction we will end EXACTLY on the
    // far edge of a map, giving us a predictable shape of the outermost part of the diamond
    // (further expanded on three comments down).

    const STEPS: usize = 26501365;
    let map: Vec<_> = input.lines().map(str::as_bytes).collect();
    let s = map.len();

    // The amount of completely-filled rooms on a main half-axis. Basically, the result of going
    // straight in a line in one direction, but the last room is special and not counted here.
    let mainline = (STEPS - map.len() / 2) / map.len() - 1;

    // Of the (uneven amount of) mainline rooms, half rounded up are EVEN, half rounded down are
    // ODD. That's because the first and last rooms in the sequence are even; and there's an
    // additional partially-filled ODD room at the end we're not counting here.
    let (mainline_evens, mainline_odds) = ((mainline + 1) / 2, mainline / 2);

    // Each quarter is composed of alternating odd and even rooms entered from the corner closest
    // to the center. For example, looking at the bottom right quarter of a `mainline = 5` example:
    //
    // +------
    // |OEOEoe
    // |EOEoe
    // |OEoe
    // |Eoe
    // |oe
    // |e
    //
    // Lowercase letters are partially filled, uppercase letters are completely filled. Because for
    // each of those rooms, the optimal way of floodfilling them is from the top left corner (point
    // closest to S in the center), all of the same letter are identically-filled. It takes the
    // same amount of steps to reach each 'o' or 'e', too, so they have the same amount of leftover
    // steps for their partial fill.
    //
    // We can calculate the total number completely-filled maps in a quarter with some math
    // shortcuts. The number of odd maps is the sum of the first `n` odd numbers (which is simply
    // `n^2`), and the number of even maps is the sum of the first `n` even numbers (which is
    // simply `n(n + 1)`).
    //
    // In this case, `n` is half of `mainline` rounded down, where `mainline` is the number of
    // completely-filled rooms along the axis. In the example above, it is 5, resulting in 4 odd
    // and 6 even completely-filled rooms in the quarter (which matches the image).
    //
    // The number of "close" (odd) edge tiles is simply equal to the mainline count, and the number
    // of "far" (even) edge tiles is one more than that.
    let quarter_odds = (mainline / 2) * (mainline / 2);
    let quarter_evens = (mainline / 2) * (1 + mainline / 2);
    let quarter_edge_close = mainline;
    let quarter_edge_far = quarter_edge_close + 1;

    // For completely-filled maps, it doesn't matter *how* they got filled, so we can put them all
    // in one bucket.
    let total_evens = 4 * (mainline_evens + quarter_evens);
    let total_odds = 1 + 4 * (mainline_odds + quarter_odds);

    // That's it, we have now counted the exact number of all relevant maps. All that remains is
    // doing the correct floodfills to get the number of target tiles in each kind, and multiply it
    // by the number of rooms.

    // Lists of all entrances used.
    let center: &[(usize, usize)] = &[(s / 2, s / 2)];
    let middles = &[
        (0, s / 2),     // entering from left edge
        (s / 2, 0),     // entering from top edge
        (s - 1, s / 2), // entering from right edge
        (s / 2, s - 1), // entering from bottom edge
    ];
    let corners = &[
        (0, 0),         // entering from top left corner
        (s - 1, 0),     // entering from top right corner
        (s - 1, s - 1), // entering from bottom right corner
        (0, s - 1),     // entering from bottom left corner
    ];

    // Each entry describes one type of room, potentially mirrored 4 times across all quarters/half
    // axes. There are four values governing each combination:
    // - how many there are;
    // - which points we start from (when there are multiple, it's the mirroring mentioned earlier);
    // - whether we enter the tile with an odd number of steps remaining;
    // - and the step limit, if any.
    let combinations = [
        // Completely-filled maps
        (total_odds, center, true, None),
        (total_evens, center, false, None),
        // Partially-filled maps
        (1, middles, false, Some(s - 1)), // Final special mainline maps
        (quarter_edge_close, corners, true, Some(3 * s / 2 - 1)),
        (quarter_edge_far, corners, false, Some(s / 2 - 1)),
    ];

    // For every map type, do the appropriate floodfill once, multiply it by the amount of that map
    // we have, and sum all of those.
    let mut total = 0;
    for (count, starts, odd_steps, step_limit) in combinations {
        for &(x, y) in starts {
            let mut starts = vec![];
            // If we have an odd amount of steps remaining, we use all neighbours of the given
            // start point as the actual start points, to account for the 1 extra step and have an
            // even amount of remaining steps again, which the floodfill requires as a
            // precondition.
            if odd_steps {
                if x > 0 {
                    starts.push((x - 1, y));
                }
                if x < (s - 1) {
                    starts.push((x + 1, y));
                }
                if y > 0 {
                    starts.push((x, y - 1));
                }
                if y < (s - 1) {
                    starts.push((x, y + 1));
                }
            } else {
                starts.push((x, y));
            }

            let steps = step_limit.map(|s| s - s % 2).unwrap_or(usize::MAX);
            let tiles = floodfill(&map, &starts, steps);

            total += count * tiles;
        }
    }
    Ok(total)
}

/// Performs a special floodfill, and returns the number of tiles reached.
///
/// Key insight: Whenever your step count is even, you can drain it to 0 in place (by pacing back
/// and forth). Thus, we are simply searching for all tiles that are an even amount of steps away
/// from the start. This, of course, assumes that `steps` is an even number.
///
/// So, for the actual floodfill, we always take size-2 steps--instead of the immediate neighbours,
/// we go to all of the 8 tiles that are two steps away.
fn floodfill(map: &[&[u8]], start: &[(usize, usize)], steps: usize) -> usize {
    let mut queue = VecDeque::from_iter(start.iter().map(|&s| (s, steps / 2)));
    let mut found = HashSet::new();
    while let Some(((x, y), remaining)) = queue.pop_front() {
        if !found.insert((x, y)) || remaining == 0 {
            continue;
        }
        let open = |dx: i32, dy: i32| {
            let Ok(x) = usize::try_from(x as i32 + dx) else {
                return false;
            };
            let Ok(y) = usize::try_from(y as i32 + dy) else {
                return false;
            };

            !found.contains(&(x, y))
                && matches!(map.get(y).and_then(|line| line.get(x)), Some(b'.' | b'S'))
        };

        let (left, up, right, down) = (open(-1, 0), open(0, -1), open(1, 0), open(0, 1));
        let dirs = [
            (left, -2, 0),
            (left || up, -1, -1),
            (up, 0, -2),
            (up || right, 1, -1),
            (right, 2, 0),
            (right || down, 1, 1),
            (down, 0, 2),
            (down || left, -1, 1),
        ];

        for (reachable, dx, dy) in dirs {
            if reachable && open(dx, dy) {
                queue.push_back((
                    ((x as i32 + dx) as usize, (y as i32 + dy) as usize),
                    remaining - 1,
                ));
            }
        }
    }

    found.len()
}
