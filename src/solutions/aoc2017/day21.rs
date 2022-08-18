use std::collections::HashMap;

/// Expand the initial tile 5 times using the recipes from input; count the number of lit
/// tiles at the end.
pub fn one(input: &str) -> crate::Result<usize> {
    let rules = build_rules(input)?;
    
    Ok(Tile::initial().step3(&rules)?.into_iter().flat_map(|t| t.step2(&rules)).map(|t| t.count()).sum())
}

/// Expand the initial tile 18 times using the recipes from input; count the number of lit
/// tiles at the end.
pub fn two(input: &str) -> crate::Result<usize> {
    let rules = build_rules(input)?;
    let mut tiles = HashMap::from([(Tile::initial(), 1)]);

    for _ in 0..6 {
        let mut next_tiles = HashMap::new();
        for (prev, n) in tiles {
            for next in prev.step3(&rules)? {
                *next_tiles.entry(next).or_default() += n;
            }
        }
        tiles = next_tiles;
    }

    Ok(tiles.into_iter().map(|(t, n)| t.count() * n).sum())
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Tile {
    T2([bool; 4]),
    T3([bool; 9]),
}

impl Tile {
    /// The starting tile as given by the puzzle description.
    fn initial() -> Self {
        Self::T3([false, true, false, false, false, true, true, true, true])
    }

    /// Given a 3x3 tile, steps it twice, producing four 3x3 tiles. These need to be
    /// reinterpreted into 2x2 tiles if used in further iterations.
    fn step2(self, rules: &Rules) -> Vec<Self> {
        rules[&self].iter().flat_map(|t| rules[t].iter().copied()).collect()
    }

    /// Given a 3x3 tile, steps it three times, producing nine 3x3 tiles. Performs a correct
    /// reinterpretation step after two iterations. These nine 3x3 tiles can all then be
    /// expanded independently of each other in the future.
    fn step3(self, rules: &Rules) -> crate::Result<Vec<Self>> {
        match self.step2(rules).as_slice() {
            [Self::T3(a), Self::T3(b), Self::T3(c), Self::T3(d)] => {
                let tiles = [
                    Self::T2([a[0], a[1], a[3], a[4]]),
                    Self::T2([a[2], b[0], a[5], b[3]]),
                    Self::T2([b[1], b[2], b[4], b[5]]),
                    Self::T2([a[6], a[7], c[0], c[1]]),
                    Self::T2([a[8], b[6], c[2], d[0]]),
                    Self::T2([b[7], b[8], d[1], d[2]]),
                    Self::T2([c[3], c[4], c[6], c[7]]),
                    Self::T2([c[5], d[3], c[8], d[6]]),
                    Self::T2([d[4], d[5], d[7], d[8]]),
                ];
                Ok(tiles.iter().flat_map(|t| rules[t].iter().copied()).collect())
            }
            _ => Err("didn't start with a 3x3 tile".into()),
        }
    }

    /// Counts the number of lit cells in the tile.
    fn count(self) -> usize {
        match self {
            Tile::T2(t) => t.iter().filter(|&&b| b).count(),
            Tile::T3(t) => t.iter().filter(|&&b| b).count(),
        }
    }

    /// Rotates the tile one step (90 degrees) clockwise.
    fn rotate(t: Self) -> Self {
        match t {
            Self::T2(t) => Self::T2([t[2], t[0], t[3], t[1]]),
            Self::T3(t) => Self::T3([t[6], t[3], t[0], t[7], t[4], t[1], t[8], t[5], t[2]]),
        }
    }

    /// Flips the tile around the vertical center (so flipping left with right).
    fn flip_v(t: Self) -> Self {
        match t {
            Self::T2(t) => Self::T2([t[1], t[0], t[3], t[2]]),
            Self::T3(t) => Self::T3([t[2], t[1], t[0], t[5], t[4], t[3], t[8], t[7], t[6]]),
        }
    }

    /// Flips the tile around the horizontal center (so flipping up and down).
    fn flip_h(t: Self) -> Self {
        match t {
            Self::T2(t) => Self::T2([t[2], t[3], t[0], t[1]]),
            Self::T3(t) => Self::T3([t[6], t[7], t[8], t[3], t[4], t[5], t[0], t[1], t[2]]),
        }
    }

    /// Convenience function to apply (or not) rotation and vertical/horizontal flips.
    fn transform(mut self, r: bool, v: bool, h: bool) -> Tile {
        if r {
            self = Self::rotate(self);
        }
        if v {
            self = Self::flip_v(self);
        }
        if h {
            self = Self::flip_h(self);
        }
        self
    }

    /// Returns all possible variants of a tile (rotated, flipped, etc). There's always up to
    /// eight unique ones. That's because rotating twice is the same as flipping vertically
    /// and horizontally; which means that we only need to keep track of one rotation, giving
    /// us three degrees of freedom (rotated once, flipped vertically, flipped horizontally).
    fn all_variants(self) -> [Self; 8] {
        [
            self.transform(false, false, false),
            self.transform(true, false, false),
            self.transform(false, true, false),
            self.transform(true, true, false),
            self.transform(false, false, true),
            self.transform(true, false, true),
            self.transform(false, true, true),
            self.transform(true, true, true),
        ]
    }
}

type Rules = HashMap<Tile, Vec<Tile>>;

/// Builds the rules map from puzzle input. Note that every left-hand side is expanded
/// into all possible variants. That way there are more rules, but they automatically
/// cover rotation/flipping without any extra handling in the code that uses the ruleset.
fn build_rules(input: &str) -> crate::Result<Rules> {
    let mut rules = HashMap::new();
    for rule in parse(input) {
        match rule {
            Some((prev, next)) => {
                for prev in prev.all_variants() {
                    rules.entry(prev).or_insert_with(|| next.clone());
                }
            }
            None => return Err("invalid rule in input".into()),
        }
    }
    Ok(rules)
}

/// Parses the puzzle input into a series of rules.
fn parse(input: &str) -> impl Iterator<Item = Option<(Tile, Vec<Tile>)>> + '_ {
    fn v(s: &str) -> Vec<bool> {
        s.chars().filter_map(|c| (c != '/').then(|| c == '#')).collect()
    }
    fn p2(s: &str) -> Option<Tile> {
        Some(Tile::T2(v(s).try_into().ok()?))
    }
    fn p3(s: &str) -> Option<Tile> {
        Some(Tile::T3(v(s).try_into().ok()?))
    }
    fn p4(s: &str) -> Option<Vec<Tile>> {
        let v: Vec<_> = v(s);
        (v.len() == 16).then(|| vec![
            Tile::T2([v[0], v[1], v[4], v[5]]),
            Tile::T2([v[2], v[3], v[6], v[7]]),
            Tile::T2([v[8], v[9], v[12], v[13]]),
            Tile::T2([v[10], v[11], v[14], v[15]]),
        ])
    }

    input.lines().map(|line| {
        let (prev, next) = line.split_once(" => ")?;
        match (prev.len(), next.len()) {
            (5, 11) => Some((p2(prev)?, vec![p3(next)?])),
            (11, 19) => Some((p3(prev)?, p4(next)?)),
            _ => None,
        }
    })
}
