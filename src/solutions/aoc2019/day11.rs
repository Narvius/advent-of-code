use std::collections::HashSet;

use crate::common::intcode::v2::*;

/// Find how many tiles the emergency hull painting robot paints when starting on black.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(run_robot(input, false)?.1.len())
}

/// Find the identifier painted by the emergency hull painting robot when starting on white.
pub fn two(input: &str) -> crate::Result<String> {
    Ok(crate::common::pixel_display_from_set(
        run_robot(input, true)?.0,
    ))
}

type Points = HashSet<(i32, i32)>;

/// Runs the emergency hull painting robot; returns two sets of points. The first set contains
/// the coordinates of all tiles painted white; the second set contains the coordinates of
/// all tiles that have been painted at least once.
fn run_robot(input: &str, starting_panel_white: bool) -> crate::Result<(Points, Points)> {
    let mut p = Program::with_capacity(input, 2000, [])?;
    let (mut pos, mut dir) = ((0, 0), (0, -1));
    let mut panels = HashSet::new();
    let mut painted = HashSet::new();

    if starting_panel_white {
        panels.insert((0, 0));
    }
    p.input.push_back(Int::from(panels.contains(&pos)));
    while let Outcome::Ok = p.step()? {}

    while !p.output.is_empty() {
        painted.insert(pos);
        match p.output.pop_front() {
            Some(0) => panels.remove(&pos),
            Some(1) => panels.insert(pos),
            n => Err(format!("invalid paint instruction {n:?}"))?,
        };

        dir = match p.output.pop_front() {
            Some(0) => (dir.1, -dir.0),
            Some(1) => (-dir.1, dir.0),
            n => Err(format!("invalid turn direction {n:?}"))?,
        };

        pos = (pos.0 + dir.0, pos.1 + dir.1);
        p.input.push_back(Int::from(panels.contains(&pos)));
        while let Outcome::Ok = p.step()? {}
    }

    Ok((panels, painted))
}
