/// Perform only the instructions in the input within 50 units of origin; then
/// count the number of active cubes.
pub fn one(input: &str) -> crate::Result<i64> {
    run_reboot_steps(input, true)
}

/// Perform the instructions from input, then count the number of active cubes.
pub fn two(input: &str) -> crate::Result<i64> {
    run_reboot_steps(input, false)
}

/// Shared code between both parts. `initialization_only` set to true excludes
/// instructions outside of 50 units within origin.
///
/// The core idea is as follows: Instead of keeping track of each unit
/// separately, we keep a list of "added" and "subtracted" larger cubes, each
/// instruction then finds cubes it intersects with and spawns smaller
/// sub-cubes with opposite polarity. At the end, all of these virtual cubes
/// are summed.
///
/// The opposite polarity works universally:
/// - when two added cubes intersect, we need a subtracted cube to not double
///   count the intersected area
/// - when two subtracted cubes intersect, we need an added cube to not double
///   subtract the intersected area
/// - when an added and a subtracted cube intersect, the intersection would
///   naturally become 0, but the later cube has to "win", because order
///   matters. So we still need an additional overlap cube with the opposite
///   polarity.
fn run_reboot_steps(input: &str, initialization_only: bool) -> crate::Result<i64> {
    let steps = parse(input).filter(|(cube, _)| !initialization_only || is_initialization(cube));
    let mut diffs: Vec<Diff> = vec![];
    let mut to_add = vec![];

    for (cube, on) in steps {
        // Add sub-cubes with opposite polarity for intersections.
        for &(diffcube, positive) in &diffs {
            if let Some(shared) = intersect(cube, diffcube) {
                to_add.push((shared, -positive));
            }
        }

        diffs.append(&mut to_add);

        // Once all other processing is finished, add the cube itself.
        if on {
            diffs.push((cube, 1));
        }
    }

    Ok(diffs
        .into_iter()
        .map(|(cube, polarity)| volume(cube) * polarity as i64)
        .sum())
}

/// An start-inclusive, end-exclusive interval.
type Interval = (i32, i32);

/// A cube, described by three [`Interval`]s; one per axis.
type Cube = [Interval; 3];

/// An instruction from input. The bool describes if the cube is added (`true`)
/// or subtracted (`false`).
type Instruction = (Cube, bool);

/// A stored sub-cube (see [`run_reboot_steps`]). The second value indicates
/// the polarity of the cube.
type Diff = (Cube, i32);

/// Returns the intersection between two axis-aligned cubes.
fn intersect(lhs: Cube, rhs: Cube) -> Option<Cube> {
    fn interval_intersect((a0, a1): Interval, (b0, b1): Interval) -> Option<Interval> {
        (b0 <= a1 && a0 <= b1).then_some((a0.max(b0), a1.min(b1)))
    }

    Some([
        interval_intersect(lhs[0], rhs[0])?,
        interval_intersect(lhs[1], rhs[1])?,
        interval_intersect(lhs[2], rhs[2])?,
    ])
}

/// Finds the volume of a cube.
fn volume([(x0, x1), (y0, y1), (z0, z1)]: Cube) -> i64 {
    (x1 - x0) as i64 * (y1 - y0) as i64 * (z1 - z0) as i64
}

/// Checks if an instruction is an initialization instruction; that is, if all
/// coordinates in it are within 50 units of origin.
fn is_initialization(&[(x0, x1), (y0, y1), (z0, z1)]: &Cube) -> bool {
    [x0, x1, y0, y1, z0, z1]
        .iter()
        .all(|n| (-50..=50).contains(n))
}

/// Parses the puzzle input in to a series of [`Instruction`]s.
fn parse(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    fn parse_range(range: &str) -> Option<Interval> {
        let (lo, hi) = range.split_once("..")?;
        let (lo, hi): Interval = (lo.parse().ok()?, hi.parse().ok()?);
        // Puzzle input intervals are inclusive, but we want exclusive.
        Some((lo, hi + 1))
    }

    input.lines().filter_map(|line| {
        let (mode, coords) = line.split_once(" x=")?;
        let (x, coords) = coords.split_once(",y=")?;
        let (y, z) = coords.split_once(",z=")?;

        Some((
            [parse_range(x)?, parse_range(y)?, parse_range(z)?],
            mode == "on",
        ))
    })
}
