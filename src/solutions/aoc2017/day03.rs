use std::collections::HashMap;

/// Find the Manhattan distance from origin of the input number on the spiral.
pub fn one(input: &str) -> Result<String, String> {
    let v: usize = input.parse().map_err(|_| format!("failed to parse num"))?;

    // Small optimization: We can skip MOST of the spiral by noting that the bottom
    // right diagonal is n^2 for odd n, and deriving a closer starting position
    // based on that.
    let p = {
        let sqrt = (v as f64).sqrt().floor() as i32;
        sqrt - (1 - sqrt % 2)
    };
    let mut coords = SpiralCoords {
        x: (p - 1) / 2,
        y: (p - 1) / 2,
        dir: 0,
    };
    let skip = (p * p + 1) as usize;

    if let Some((x, y)) = coords.nth(v - skip) {
        Ok(format!("{}", x.abs() + y.abs()))
    } else {
        Err(format!("unreachable"))
    }
}

/// On the "Fibonacci spiral", find the smallest number larger than the input.
pub fn two(input: &str) -> Result<String, String> {
    let target: i32 = input.parse().map_err(|_| format!("failed to parse num"))?;
    let mut spiral = HashMap::from([((0, 0), 1)]);

    for (x, y) in (SpiralCoords { x: 0, y: 0, dir: 0 }) {
        let mut sum = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                sum += *spiral.entry((x + dx, y + dy)).or_default();
            }
        }
        spiral.insert((x, y), sum);

        if sum > target {
            return Ok(sum.to_string());
        }
    }

    Err(format!("unreachable"))
}

/// An iterator that produces all the coordinates along a spiral.
struct SpiralCoords {
    x: i32,
    y: i32,
    dir: i32,
}

impl Iterator for SpiralCoords {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            0 => {
                self.x += 1;
                if self.x > self.y {
                    self.dir = 1;
                }
            }
            1 => {
                self.y -= 1;
                if self.y.abs() == self.x {
                    self.dir = 2;
                }
            }
            2 => {
                self.x -= 1;
                if self.x == self.y {
                    self.dir = 3;
                }
            }
            3 => {
                self.y += 1;
                if self.y == self.x.abs() {
                    self.dir = 0;
                }
            }
            _ => None?,
        }
        Some((self.x, self.y))
    }
}
