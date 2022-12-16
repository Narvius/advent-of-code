use std::collections::{HashMap, HashSet, VecDeque};

/// Find the optimal amount of pressure release.
pub fn one(input: &str) -> crate::Result<i32> {
    let (starting_position, valves) = parse(input).ok_or("parse failed")?;
    let distances = build_distance_matrix(&valves);

    Ok(find_optimal_pressure_release(
        &valves,
        &distances,
        (starting_position, 30, 0, 0),
        false,
    ))
}

/// Find the optimal amount of pressure release assuming 4 minutes are spent teaching an elephant.
///
/// Currently pretty slow (nearly 15 seconds).
pub fn two(input: &str) -> crate::Result<i32> {
    let (starting_position, valves) = parse(input).ok_or("parse failed")?;
    let distances = build_distance_matrix(&valves);

    Ok(find_optimal_pressure_release(
        &valves,
        &distances,
        (starting_position, 26, 0, 0),
        true,
    ))
}

/// A HashMap where the value associated with `(source valve id, target valve id)` is the
/// number of minutes required to traverse from one to the other.
type Distances = HashMap<(usize, usize), usize>;
/// A list of valves, where one entry is made of the flow rate of the valve when opened, and
/// a list of indices of valves it has tunnels to.
type Valves = Vec<(i32, Vec<usize>)>;

/// Given a set of valves and distances between them, finds the maximum amount of pressure
/// that can be released given some `initial` conditions. The four values in that tuple, in
/// order, describe the current position, remaining time, list of open valves, already-released
/// pressure.
///
/// Since there's fewer than 64 valves, a `u64` is used as a bitset indicating which valves
/// are open or closed. If the `n`th least significant bit is set, the `n`th valve is opened.
fn find_optimal_pressure_release(
    valves: &Valves,
    distances: &Distances,
    initial: (usize, usize, u64, i32),
    double: bool,
) -> i32 {
    let working_valves: Vec<_> = valves
        .iter()
        .enumerate()
        .filter_map(|(i, (flow_rate, _))| (*flow_rate > 0).then_some(i))
        .collect();

    let mut stack = vec![initial];
    let mut highest = 0;

    while let Some((position, time, open_mask, released_pressure)) = stack.pop() {
        let mut end = true;
        highest = highest.max(released_pressure);
        for &i in &working_valves {
            if (open_mask & (1 << i)) == 0 {
                let used_time = distances[&(position, i)] + 1;
                if used_time <= time {
                    end = false;
                    stack.push((
                        i,
                        time - used_time,
                        open_mask | (1 << i),
                        released_pressure + (time - used_time) as i32 * valves[i].0,
                    ));
                }
            }
        }
        if double && end {
            let initial_elephant = (initial.0, initial.1, open_mask, released_pressure);
            highest = highest.max(find_optimal_pressure_release(
                valves,
                distances,
                initial_elephant,
                false,
            ));
        }
    }

    highest
}

/// Builds a list of the distances between all valves.
fn build_distance_matrix(valves: &Valves) -> Distances {
    let mut result = HashMap::new();

    for i in 0..valves.len() {
        let mut stack = VecDeque::from([(i, 0)]);
        let mut visited = HashSet::from([i]);

        while let Some((j, distance)) = stack.pop_front() {
            result.insert((i, j), distance);
            let neighbours = valves[j].1.iter();
            stack
                .extend(neighbours.filter_map(|&n| visited.insert(n).then_some((n, distance + 1))));
        }
    }

    result
}

/// Parses the input into the starting position and a list of valve data (flow rate
/// and connections).
fn parse(input: &str) -> Option<(usize, Valves)> {
    let mut data = vec![];
    let mut names = vec![];

    for line in input.lines() {
        let (valve_name, line) = line.split_once(' ')?.1.split_once(' ')?;
        let (flow_rate, line) = line.split_once('=')?.1.split_once(';')?;
        let leads_to = line.split_once(" to ")?.1.split_once(' ')?.1;

        names.push(valve_name);
        data.push((flow_rate.parse().ok()?, leads_to.split(", ")));
    }

    let get_index = move |name: &str| names.iter().position(|&n| name == n);
    let starting_position = get_index("AA")?;
    Some((
        starting_position,
        data.into_iter()
            .map(|(flow_rate, leads_to)| {
                (flow_rate, leads_to.filter_map(|n| get_index(n)).collect())
            })
            .collect(),
    ))
}
