/// Find the number of constellations in the input (groups of points where each point is no
/// more than a manhattan distance of 3 away from at least one other point).
pub fn one(input: &str) -> crate::Result<usize> {
    let mut constellations = vec![];
    for p in input.lines().map(parse) {
        let p = p?;

        // Find all the touched buckets.
        let mut touched: Vec<_> = (0..constellations.len())
            .rev()
            .filter(|&i| touches(p, &constellations[i]))
            .collect();

        match touched.len() {
            // Didn't touch anything, therefore create a new bucket.
            0 => constellations.push(vec![p]),

            // Touched exactly one bucket; just join it.
            1 => constellations[touched[0]].push(p),

            // Merge all touched buckets together.
            _ => {
                let target = touched.pop().expect("len is larger 0");
                for i in touched {
                    let mut removed = constellations.remove(i);
                    constellations[target].append(&mut removed);
                }
                constellations[target].push(p);
            }
        }
    }

    Ok(constellations.len())
}

/// Freebie!
pub fn two(_input: &str) -> crate::Result<&str> {
    Ok("done!")
}

type Point = (i32, i32, i32, i32);
type Bucket = Vec<Point>;

/// 4D manhattan distance between two points.
fn d((x1, y1, z1, w1): Point, (x2, y2, z2, w2): Point) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs() + (w1 - w2).abs()
}

/// Checks if the point `p` is close enough to any point in `b` to merge into it.
fn touches(p: Point, b: &Bucket) -> bool {
    b.iter().any(|bp| d(p, *bp) <= 3)
}

/// Parses a line of puzzle input into a 4D point.
fn parse(line: &str) -> crate::Result<Point> {
    let vs = line
        .split(',')
        .map(|v| v.parse())
        .collect::<Result<Vec<_>, _>>()?;
    match vs.len() {
        4 => Ok((vs[0], vs[1], vs[2], vs[3])),
        _ => Err("invalid line".into()),
    }
}
