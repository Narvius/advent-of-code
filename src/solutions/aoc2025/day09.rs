use std::collections::HashMap;

/// Find the size of the largest axis-aligned rectangle that can be formed by taking two points
/// from the input as its opposite corners.
pub fn one(input: &str) -> crate::Result<i64> {
    let points = Vec::from_iter(input.lines().filter_map(|line| {
        let (a, b) = line.split_once(',')?;
        Some((a.parse::<i64>().ok()?, b.parse::<i64>().ok()?))
    }));

    let mut largest = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let ((x1, y1), (x2, y2)) = (points[i], points[j]);
            largest = largest.max(((x1 - x2 + 1) * (y1 - y2 + 1)).abs());
        }
    }

    Ok(largest)
}

/// Find the largest axis-aligned rectangle fully contained within the polygon formed by the input
/// points.
pub fn two(input: &str) -> crate::Result<i64> {
    let points = Vec::from_iter(input.lines().filter_map(|line| {
        let (a, b) = line.split_once(',')?;
        Some((a.parse::<i64>().ok()?, b.parse::<i64>().ok()?))
    }));
    let mut verticals: HashMap<i64, (i64, i64)> = HashMap::new();

    // Collect all vertical lines in the input, as a mapping of `x => (y1..y2)`.
    for w in points
        .windows(2)
        .chain(std::iter::once(&[points[points.len() - 1], points[0]][..]))
    {
        if w[0].0 == w[1].0 {
            verticals.insert(w[0].0, (w[0].1.min(w[1].1), w[0].1.max(w[1].1)));
        }
    }

    // Get our "lattice points". In the input polygon, all edges are axis-aligned. As such we can
    // reduce it into a grid of rectangles, what I call the "lattice". This lattice splits at every
    // `x` and `y` coordinate that shows up in the input, so we collect those here, in sorted
    // order.
    //
    // For example, the example shown in the puzzle input has a 3x3 lattice, because there's 4
    // distinct X and Y coordinates each. The original polygon looks like this (with surrounding
    // space trimmed off):
    //
    // .....#XXX#
    // .....X...X
    // #XXXX#...X
    // X........X
    // #XXXXXX#.X
    // .......X.X
    // .......#X#
    //
    // Lattice points:
    //
    // xs: 2, 7, 9, 11
    // ys: 1, 3, 5, 7
    //
    // So we split the above grid according to the above lattice points (note, that in my
    // implementation, lines ON splits count as belonging to both sides, so the picture may be a
    // little bit disorienting):
    //
    // .....#  #XX  XX#
    // .....X  X..  ..X
    // #XXXX#  #..  ..X
    //
    // #XXXX#  #..  ..X
    // X.....  ...  ..#
    // #XXXXX  XX#  #.X
    //
    // #XXXXX  XX#  #.X
    // ......  ..X  X.X
    // ......  ..#  #X#
    //
    // Now for each of those rectangles, we can decide whether all of it is inside or outside the
    // polygon, denoted here with '#' and '.' respectively:
    //
    // .##
    // ###
    // ..#
    //
    // And that is the lattice.
    let (mut xs, mut ys): (Vec<_>, Vec<_>) = points.iter().copied().unzip();
    xs.sort_unstable();
    ys.sort_unstable();
    xs.dedup();
    ys.dedup();

    let mut lattice = Vec::with_capacity((xs.len() - 1) * (ys.len() - 1));

    // We do one raycast per lattice row, using an arbitrary y value contained inside that row of
    // rectangles. The ray goes from the left edge straight to the right; intersections with the
    // polygon are easy to handle thanks to the pre-calculated `verticals`.
    for j in 0..ys.len() - 1 {
        let y = (ys[j] + ys[j + 1]) / 2;
        let mut inside = false;

        for x in &xs[0..xs.len() - 1] {
            if let Some(&(y1, y2)) = verticals.get(x) {
                if y1 <= y && y <= y2 {
                    inside = !inside;
                }
            }

            lattice.push(inside);
        }
    }

    // Now we can check if rectangles are contained within the polygon via the lattice, though we
    // only bother for rectangles that would actually be larger.
    let mut largest = 0;
    for i in 0..points.len() {
        'next_rect: for j in i + 1..points.len() {
            let ((x1, y1), (x2, y2)) = (points[i], points[j]);
            let size = (x1.abs_diff(x2) as i64 + 1) * (y1.abs_diff(y2) as i64 + 1);

            if size > largest {
                let i1 = xs.iter().position(|&x| x == x1).unwrap();
                let i2 = xs.iter().position(|&x| x == x2).unwrap();
                let j1 = ys.iter().position(|&y| y == y1).unwrap();
                let j2 = ys.iter().position(|&y| y == y2).unwrap();

                for i in i1.min(i2)..i1.max(i2) {
                    for j in j1.min(j2)..j1.max(j2) {
                        if !lattice[i + (xs.len() - 1) * j] {
                            continue 'next_rect;
                        }
                    }
                }

                largest = size;
            }
        }
    }

    Ok(largest)
}
