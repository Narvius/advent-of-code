use std::collections::HashSet;

use crate::common::Grid;

/// Appraise the total fencing cost if the perimeter cost is equal to the length of the perimeter.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(appraise(input, |e| e.len()))
}

/// Appraise the total fencing cost if the perimeter cost is equal to the number of straight
/// segments making it up.
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(appraise(input, |mut edge| {
        edge.sort();
        1 + edge
            .windows(2)
            .filter(|w| {
                let (((dx1, dy1), (x1, y1)), ((dx2, dy2), (x2, y2))) = (w[0], w[1]);
                let same_side = (dx1, dy1) == (dx2, dy2);
                let contiguous = (x1, y1 + 1) == (x2, y2);
                !(same_side && contiguous)
            })
            .count()
    }))
}

/// Appraises the total cost for fencing all plots, using the `fencing` function to get the total
/// multiplier from perimeter cost.
fn appraise(input: &str, fencing: fn(Vec<(V2, V2)>) -> usize) -> usize {
    let grid = Grid::from_input(input);
    let mut visited = HashSet::new();

    grid.iter_with_position()
        .map(|(p, crop)| {
            let mut stack = vec![p];
            let mut area = 0;
            let mut edge = vec![];

            while let Some((x, y)) = stack.pop() {
                if !visited.insert((x, y)) {
                    continue;
                }

                area += 1;

                for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                    if Some(crop) == grid.get((x + dx, y + dy)) {
                        stack.push((x + dx, y + dy));
                    } else if dx == 0 {
                        edge.push(((dx, dy), (y, x)));
                    } else {
                        edge.push(((dx, dy), (x, y)));
                    }
                }
            }

            area * fencing(edge)
        })
        .sum()
}

type V2 = (i32, i32);
