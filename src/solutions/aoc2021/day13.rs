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
    Ok(build_representation(&points).ok_or("failed output")?)
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

/// Turns the set of points into a human-readable representation.
fn build_representation(points: &Points) -> Option<String> {
    let xs = points.iter().min_by_key(|p| p.0)?.0..=points.iter().max_by_key(|p| p.0)?.0;
    let ys = points.iter().min_by_key(|p| p.1)?.1..=points.iter().max_by_key(|p| p.1)?.1;
    let mut output = String::with_capacity(1 + (1 + xs.size_hint().0) * ys.size_hint().0);
    output.push('\n');

    for y in ys {
        for x in xs.clone() {
            output.push(if points.contains(&(x, y)) { '#' } else { ' ' });
        }
        output.push('\n');
    }

    Some(output)
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
