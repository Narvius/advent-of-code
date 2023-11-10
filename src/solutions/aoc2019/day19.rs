use crate::common::intcode::v2::*;

/// Count the number of active squares in the nearest 50x50 region.
pub fn one(input: &str) -> crate::Result<usize> {
    let p = Program::with_capacity(input, 500, [])?;
    let mut result = 0;
    for x in 0..50 {
        for y in 0..50 {
            if active(&p, (x, y)) {
                result += 1;
            }
        }
    }
    Ok(result)
}

/// Find the coordinates of the top left corner of the closest 100x100 square
/// that is fully active.
pub fn two(input: &str) -> crate::Result<Int> {
    const TARGET_SIZE: Int = 100;
    let p = Program::with_capacity(input, 500, [])?;

    // Find an arbitrary starting point a bit along the cone, to avoid weirdness
    // relating to it being too thin in the beginning.
    let (mut min_x, mut max_x) = (Int::MAX, Int::MIN);
    for x in 0..20 {
        if active(&p, (x, 10)) {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
        }
    }

    // Build slope followers. Advance `top` follower until the the horizontal
    // distance between the last `top` and `bottom` element is enough to fit
    // a `TARGET_SIZE` square.
    //
    // Then, keep advancing them in lockstep until the vertical distance is
    // also enough.
    let mut top = SlopeFollower::new(p.clone(), (max_x, 10), true);
    let mut bottom = SlopeFollower::new(p.clone(), (min_x, 10), false);

    let (mut l, mut t, mut r, mut b) = (0, 0, 0, 0);
    loop {
        while (r - l) < (TARGET_SIZE - 1) {
            (r, t) = top.next().unwrap();
        }
        if (b - t) == (TARGET_SIZE - 1) {
            return Ok(10000 * l + t);
        }
        (l, b) = bottom.next().unwrap();
    }
}

/// Checks whether a given square is active.
fn active(p: &Program, (x, y): (Int, Int)) -> bool {
    let mut p = p.clone();
    p.input.extend([x, y]);
    while let Ok(Outcome::Ok) = p.step() {}
    p.output.pop_front().unwrap_or(0) == 1
}

/// An iterator that returns relevant points along either the top or bottom edge
/// of the tractor beam.
struct SlopeFollower {
    previous: (Int, Int),
    program: Program,
    top: bool,
}

impl SlopeFollower {
    fn new(program: Program, start: (Int, Int), top: bool) -> Self {
        Self {
            program,
            top,
            previous: start,
        }
    }
}

impl Iterator for SlopeFollower {
    type Item = (Int, Int);

    fn next(&mut self) -> Option<Self::Item> {
        // The top slope follower tries to go right, and if it can't, goes down until it can.
        // The bottom slope follower tries to go down, it if it can't, goes right until it can.
        let (dx, dy) = if self.top { (1, 0) } else { (0, 1) }; // direction of next active
        let (sx, sy) = (dy, dx); // scan direction to go in if +(dx,dy) is not active

        for i in 0.. {
            let (x, y) = self.previous;
            let (x, y) = (x + i * sx + dx, y + i * sy + dy);

            let mut p = self.program.clone();
            p.input.extend([x, y]);
            while let Outcome::Ok = p.step().ok()? {}

            if let Some(1) = p.output.pop_front() {
                return Some(std::mem::replace(&mut self.previous, (x, y)));
            }
        }

        unreachable!()
    }
}
