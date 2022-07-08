/// Find the particle that, ultimately, stays closest to origin. That's simply the one with
/// the slowest acceleration. We use manhattan/taxicab distance.
pub fn one(input: &str) -> crate::Result<usize> {
    fn manhattan((ax, ay, az): (i32, i32, i32), (bx, by, bz): (i32, i32, i32)) -> i32 {
        (ax - bx).abs() + (ay - by).abs() + (az - bz).abs()
    }

    Ok(parse(input)
        .map(|p| manhattan(p.a, (0, 0, 0)))
        .enumerate()
        .min_by_key(|p| p.1)
        .map(|p| p.0)
        .unwrap_or(0))
}

/// Simulate all the particles, destroying ones that collide. Return the number of particles
/// remaining after all collisions are done.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut ps: Vec<Particle> = parse(input).collect();

    // The number of iterations is basically arbitrarily-chosen. May require more for some
    // problem sets.
    for _ in 0..100 {
        for p in &mut ps {
            p.step();
        }
        ps.sort_unstable_by_key(|p| p.p);

        let mut from = 0;
        loop {
            let mut run = 0;
            while let Some(p) = ps.get(from + run + 1) {
                if ps[from].p == p.p {
                    run += 1;
                } else {
                    break;
                }
            }

            if run > 0 {
                ps.drain(from..=(from + run));
            } else if from < ps.len() {
                from += 1;
            } else {
                break;
            }
        }
    }

    Ok(ps.len())
}

/// A single particle from the puzzle input; a triplet of 3-dimensional position, velocity
/// and acceleration.
struct Particle {
    p: (i32, i32, i32),
    v: (i32, i32, i32),
    a: (i32, i32, i32),
}

impl Particle {
    /// Advances the particle by one simulation step.
    fn step(&mut self) {
        self.v = {
            let (ax, ay, az) = self.a;
            let (vx, vy, vz) = self.v;
            (vx + ax, vy + ay, vz + az)
        };
        self.p = {
            let (vx, vy, vz) = self.v;
            let (px, py, pz) = self.p;
            (px + vx, py + vy, pz + vz)
        };
    }
}

/// Parses the input into particles.
fn parse(input: &str) -> impl Iterator<Item = Particle> + '_ {
    fn triple(s: &str) -> Option<(i32, i32, i32)> {
        let (_, d) = s[0..s.len() - 1].split_once('<')?;
        let t: Vec<_> = d.split(',').collect();
        match t.as_slice() {
            [x, y, z] => Some((x.parse().ok()?, y.parse().ok()?, z.parse().ok()?)),
            _ => None,
        }
    }

    input.lines().filter_map(|line| {
        let t: Vec<_> = line.split(", ").collect();
        match t.as_slice() {
            [p, v, a] => Some(Particle {
                p: triple(p)?,
                v: triple(v)?,
                a: triple(a)?,
            }),
            _ => None,
        }
    })
}
