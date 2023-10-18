/// Find the total energy of all bodies after 1000 steps of simulation.
pub fn one(input: &str) -> crate::Result<i32> {
    let mut bodies = parse(input)?;
    for _ in 0..1000 {
        for axis in 0..3 {
            step_axis(&mut bodies, axis);
        }
    }
    Ok(bodies.into_iter().map(|b| b.energy()).sum())
}

/// Find the amount of steps required before the simulation returns to the original state.
///
/// This makes use of the fact that all three axes are completely independent. So we find how
/// many steps it takes for each axis to repeat independently, and then find the least common
/// multiple of those three to find the answer (for example, if the periods were 15, 18 and 20,
/// they would align after step 180, the least common multiple of 15, 18 and 20).
pub fn two(input: &str) -> crate::Result<usize> {
    let mut bodies = parse(input)?;

    Ok((0..3)
        .map(|axis| {
            let mut steps = 1;
            step_axis(&mut bodies, axis);
            while !axis_aligned(&bodies, axis) {
                steps += 1;
                step_axis(&mut bodies, axis);
            }
            steps
        })
        .fold(1, crate::common::lcm))
}

/// Checks if all bodies are "aligned" on the given axis; that is, they have the same position
/// on that axis as originally, and a speed of zero along that axis.
fn axis_aligned(bodies: &[Body], axis: usize) -> bool {
    bodies
        .iter()
        .all(|b| b.current[axis] == b.original[axis] && b.speed[axis] == 0)
}

/// Advances physics for the given axis (0 = x, 1 = y, 2 = z).
fn step_axis(bodies: &mut [Body], axis: usize) {
    for i in 0..bodies.len() {
        for j in 0..bodies.len() {
            bodies[i].speed[axis] += (bodies[j].current[axis] - bodies[i].current[axis]).signum();
        }
    }

    for body in bodies {
        body.current[axis] += body.speed[axis];
    }
}

/// A singular body from the puzzle input.
struct Body {
    original: [i32; 3],
    current: [i32; 3],
    speed: [i32; 3],
}

impl Body {
    /// Returns the total energy of the body, as described by the puzzle.
    fn energy(&self) -> i32 {
        self.current.iter().map(|v| v.abs()).sum::<i32>()
            * self.speed.iter().map(|v| v.abs()).sum::<i32>()
    }
}

/// Parses the puzzle input into a list of [bodies](`Body`).
fn parse(input: &str) -> crate::Result<Vec<Body>> {
    regex::Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>")?
        .captures_iter(input)
        .map(|c| {
            let v = [c[1].parse()?, c[2].parse()?, c[3].parse()?];
            Ok(Body {
                original: v,
                current: v,
                speed: [0; 3],
            })
        })
        .collect()
}
