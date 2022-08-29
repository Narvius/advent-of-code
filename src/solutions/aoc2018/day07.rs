/// Find the order the tasks need to be completed in, using alphabetical order to
/// resolve ambiguities.
pub fn one(input: &str) -> crate::Result<String> {
    let mut tasks = parse(input);
    let mut output = String::with_capacity(26);

    while let Some(position) = tasks.iter().position(|t| t.1.is_empty()) {
        let (task, _) = tasks.remove(position);
        output.push((task as u8 + b'A').into());
        remove_as_prerequisite(task, &mut tasks);
    }

    Ok(output)
}

/// Find how long it takes to finish all tasks with five workers.
pub fn two(input: &str) -> crate::Result<usize> {
    let mut tasks = parse(input);
    let mut time = 0;
    let mut workers: [Worker; 5] = [None; 5];

    // Assigns tasks that have no (remaining) prerequisites to idle workers, removing the
    // task from the remaining list.
    fn allocate_work(workers: &mut [Worker], tasks: &mut Vec<OrderedTask>) {
        for worker in workers {
            if worker.is_none() {
                if let Some(position) = tasks.iter().position(|t| t.1.is_empty()) {
                    let (task, _) = tasks.swap_remove(position);
                    // The problem statement one-indexes the letters (A = 1...), whereas we
                    // zero-index them; so we have to add an extra 1 here to get the
                    // correct task time.
                    *worker = Some((task, 60 + task + 1));
                }
            }
        }
    }

    // We only loop until all work is allocated. At that point, we can just add the
    // longest remaining work time of any worker to the result, without playing out
    // the time actually passing.
    allocate_work(&mut workers, &mut tasks);
    while !tasks.is_empty() {
        time += 1;
        for worker in &mut workers {
            if let Some((task, time)) = worker {
                *time -= 1;
                if *time == 0 {
                    remove_as_prerequisite(*task, &mut tasks);
                    *worker = None;
                }
            }
        }
        allocate_work(&mut workers, &mut tasks);
    }

    let longest_remaining = workers.iter().flatten().map(|p| p.1).max().unwrap_or(0);
    Ok(time + longest_remaining)
}

// Removes a newly-finished task from the prerequisites of all remaining tasks.
fn remove_as_prerequisite(task: usize, tasks: &mut [OrderedTask]) {
    for (_, v) in tasks {
        if let Some(position) = v.iter().position(|&e| e == task) {
            v.swap_remove(position);
        }
    }
}

/// A worker that is either idle, or has a task with an ID and a remaining time.
type Worker = Option<(usize, usize)>;

/// A task consisting of an ID and the IDs of all prerequisite tasks.
type OrderedTask = (usize, Vec<usize>);

/// Parses the puzzle input, constructing a list of tasks that each has an ID and
/// a list of the IDs of their prerequisite tasks.
fn parse(input: &str) -> Vec<OrderedTask> {
    let mut items: Vec<(usize, Vec<usize>)> = (0..26).map(|i| (i, vec![])).collect();

    let prerequisites = input.lines().filter_map(|line| {
        let bytes = line.as_bytes();
        Some((bytes.get(5)? - b'A', bytes.get(36)? - b'A'))
    });

    for (prerequisite, step) in prerequisites {
        items[step as usize].1.push(prerequisite as usize);
    }

    items
}
