use std::collections::HashSet;

pub fn one(input: &str) -> crate::Result<usize> {
    find_settled_count(input, |p, end_line| p.1 + 1 == end_line)
}

pub fn two(input: &str) -> crate::Result<usize> {
    Ok(1 + find_settled_count(input, |p, _| p == (500, 0))?)
}

fn find_settled_count(input: &str, condition: fn(Point, i32) -> bool) -> crate::Result<usize> {
    let (mut map, end_line) = parse(input)?;
    (0..)
        .map(move |_| drop_sand(&mut map, end_line))
        .position(|p| condition(p, end_line))
        .ok_or_else(|| "no result".into())
}

fn drop_sand(map: &mut Map, end_line: i32) -> (i32, i32) {
    let mut pos = (500, 0);

    loop {
        let candidate = [(0, 1), (-1, 1), (1, 1)]
            .map(|(dx, dy)| (pos.0 + dx, pos.1 + dy))
            .into_iter()
            .find(|c| c.1 < end_line && !map.contains(c));

        match candidate {
            Some(new_pos) => pos = new_pos,
            None => {
                map.insert(pos);
                return pos;
            }
        }
    }
}

type Map = HashSet<Point>;
type Point = (i32, i32);

fn parse(input: &str) -> crate::Result<(Map, i32)> {
    fn range(a: i32, b: i32) -> std::ops::RangeInclusive<i32> {
        if a > b {
            b..=a
        } else {
            a..=b
        }
    }

    let lines: Vec<Vec<Point>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .filter_map(|s| {
                    let (x, y) = s.split_once(',')?;
                    Some((x.parse().ok()?, y.parse().ok()?))
                })
                .collect()
        })
        .collect();

    let mut map = HashSet::new();
    for line in input.lines() {
        let points: Vec<Point> = line
            .split(" -> ")
            .filter_map(|s| {
                let (x, y) = s.split_once(',')?;
                Some((x.parse().ok()?, y.parse().ok()?))
            })
            .collect();

        for segment in points.windows(2) {
            for x in range(segment[0].0, segment[1].0) {
                map.insert((x, segment[0].1));
            }
            for y in range(segment[0].1, segment[1].1) {
                map.insert((segment[0].0, y));
            }
        }
    }

    let end_line = map.iter().map(|p| p.1).max().ok_or("no points")?;
    Ok((map, end_line + 2))
}
