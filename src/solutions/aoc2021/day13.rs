use std::collections::HashSet;

/// Apply the first fold, count the dots.
pub fn one(input: &str) -> crate::Result<usize> {
    let (mut points, folds) = parse(input)?;
    apply_fold(&mut points, folds[0]);
    Ok(points.len())
}

/// Apply all folds, read off the result.
pub fn two(input: &str) -> crate::Result<String> {
    let (mut points, folds) = parse(input)?;
    for fold in folds {
        apply_fold(&mut points, fold);
    }
    Ok(crate::common::pixel_display_from_set(points))
}

/// Applies a fold as described in the puzzle description.
fn apply_fold(points: &mut Points, (up, coord): (bool, i32)) {
    let moved: Vec<_> = points
        .iter()
        .filter(|p| if up { p.1 > coord } else { p.0 > coord })
        .copied()
        .collect();
    for p in moved {
        points.remove(&p);
        points.insert(if up {
            (p.0, 2 * coord - p.1)
        } else {
            (2 * coord - p.0, p.1)
        });
    }
}

type Points = HashSet<(i32, i32)>;
type Folds = Vec<(bool, i32)>;

/// Parses the puzzle input into a set of points and a list of folds.
fn parse(input: &str) -> crate::Result<(Points, Folds)> {
    let mut points = HashSet::new();
    let mut folds = vec![];

    for line in input.lines() {
        if line.contains(',') {
            let (x, y) = line.split_once(',').ok_or("unexpected input")?;
            points.insert((x.parse()?, y.parse()?));
        } else if line.contains('=') {
            let (pre, coord) = line.split_once('=').ok_or("unexpected input")?;
            folds.push((pre.ends_with('y'), coord.parse()?));
        }
    }

    Ok((points, folds))
}
