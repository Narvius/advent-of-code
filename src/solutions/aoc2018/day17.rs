/// Count the number of tiles ever touched by water when the analysis is done.
pub fn one(input: &str) -> crate::Result<usize> {
    let mut map = parse(input).ok_or("failed parse")?;
    map.run_water();
    Ok(map.water_tiles())
}

/// Count the number of settled water tiles when the analysis is done.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut map = parse(input).ok_or("failed parse")?;
    map.run_water();
    Ok(map.settled_tiles())
}

/// Represents the scanned map of the underground.
struct Map {
    /// Tile data.
    data: Vec<Tile>,
    /// Width. Used to convert 2D coordinates into indices into `data`.
    width: usize,
    /// Height; marks the edge of our area of interest. Water flowing past this is ignored.
    height: usize,
}

impl Map {
    /// Runs the full water simulation.
    fn run_water(&mut self) {
        let mut heads = vec![(500, 0)];

        while let Some(head) = heads.pop() {
            if !self.in_range(head) {
                continue;
            }

            let flowing = match self.get(head) {
                Tile::Open => {
                    self.set(head, Tile::Visited);
                    true
                }
                Tile::Blocked => false,
                Tile::Visited => true,
                Tile::Settled => {
                    // Submerged scanheads float back up.
                    heads.push((head.0, head.1 - 1));
                    false
                }
            };

            if flowing {
                if !self.get((head.0, head.1 + 1)).blocked() {
                    heads.push((head.0, head.1 + 1));
                    continue;
                }

                // We're on solid ground. Scan left and right until a wall or drop is found.
                // If there's two walls, settle; otherwise for each drop spawn a new scanhead.
                let (lwall, lx) = self.horizontal_scan(head, -1);
                let (rwall, rx) = self.horizontal_scan(head, 1);

                let newtile = if lwall && rwall {
                    heads.push(head);
                    Tile::Settled
                } else {
                    Tile::Visited
                };

                if !lwall && self.get((lx, head.1)) != Tile::Visited {
                    heads.push((lx, head.1));
                }
                if !rwall && self.get((rx, head.1)) != Tile::Visited {
                    heads.push((rx, head.1));
                }

                for x in lx..=rx {
                    self.set((x, head.1), newtile);
                }
            }
        }
    }

    /// Checks if the given coordinates are in range.
    fn in_range(&self, (_x, y): (i32, i32)) -> bool {
        (y as usize) < self.height
    }

    /// Gets a tile.
    fn get(&self, (x, y): (i32, i32)) -> Tile {
        self.data
            .get(y as usize * self.width + x as usize)
            .copied()
            .unwrap_or(Tile::Open)
    }

    /// Sets a tile.
    fn set(&mut self, (x, y): (i32, i32), tile: Tile) {
        self.data[y as usize * self.width + x as usize] = tile;
    }

    /// Scans horizontally from the given point, in `dx` increments. Returns whether a wall was
    /// hit, as well as the x coordinate at which the scan terminated.
    fn horizontal_scan(&self, (x, y): (i32, i32), dx: i32) -> (bool, i32) {
        for delta in 0.. {
            let x = x + dx * delta;
            if !self.get((x, y + 1)).blocked() {
                return (false, x);
            }
            if self.get((x + dx, y)).blocked() {
                return (true, x);
            }
        }
        unreachable!()
    }

    /// Counts the number of tiles that have been ever touched by water.
    fn water_tiles(&self) -> usize {
        self.data
            .iter()
            .filter(|t| matches!(t, Tile::Visited | Tile::Settled))
            .count()
    }

    /// Counts the number of tiles that contain settled water.
    fn settled_tiles(&self) -> usize {
        self.data
            .iter()
            .filter(|t| matches!(t, Tile::Settled))
            .count()
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Tile {
    /// Water can flow through this tile.
    Open,
    /// The tile is blocked.
    Blocked,

    /// Water has touched this tile, but it is still open.
    Visited,
    /// Water has settled on this tile; it is blocked.
    Settled,
}

impl Tile {
    /// Checks whether the given tile is blocked.
    const fn blocked(&self) -> bool {
        matches!(self, Tile::Blocked | Tile::Settled)
    }
}

/// Parses the puzzle input into a map. Note that this process prunes out lines ignored in
/// the solution as per the puzzle description.
fn parse(input: &str) -> Option<Map> {
    let mut instructions = vec![];
    let (mut width, mut height, mut min_y) = (0, 0, usize::MAX);

    for line in input.lines() {
        let (fixed, range) = line.split_once(", ")?;
        let (start, end) = range[2..].split_once("..")?;

        let fixed_x = fixed.starts_with('x');
        let fixed = fixed[2..].parse().ok()?;
        let (start, end) = (start.parse().ok()?, end.parse().ok()?);

        instructions.push((fixed_x, fixed, start, end));
        (width, height, min_y) = if fixed_x {
            (width.max(fixed), height.max(end), min_y.min(start))
        } else {
            (width.max(end), height.max(fixed), min_y.min(fixed))
        };
    }

    width += 2;
    height = height - min_y + 1;
    let mut data = vec![Tile::Open; width * height];

    for (fixed_x, fixed, start, end) in instructions {
        if fixed_x {
            for y in start..=end {
                data[(y - min_y) * width + fixed] = Tile::Blocked;
            }
        } else {
            for x in start..=end {
                data[(fixed - min_y) * width + x] = Tile::Blocked;
            }
        }
    }

    Some(Map {
        data,
        width,
        height,
    })
}
