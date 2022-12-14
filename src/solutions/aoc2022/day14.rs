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
            .find(|&(cx, cy)| cy < end_line && !map[cx as usize][cy as usize]);

        match candidate {
            Some(new_pos) => pos = new_pos,
            None => {
                map[pos.0 as usize][pos.1 as usize] = true;
                return pos;
            }
        }
    }
}

type Map = Vec<Vec<bool>>;
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

    let max_y = 2 + lines
        .iter()
        .flat_map(|v| v.iter())
        .fold(0, |acc, p| acc.max(p.1));

    let mut map = vec![vec![false; max_y as usize]; 1000];
    for line in lines {
        for segment in line.windows(2) {
            for x in range(segment[0].0, segment[1].0) {
                map[x as usize][segment[0].1 as usize] = true;
            }
            for y in range(segment[0].1, segment[1].1) {
                map[segment[0].0 as usize][y as usize] = true;
            }
        }
    }

    Ok((map, max_y))
}
