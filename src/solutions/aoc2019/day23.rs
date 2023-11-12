use crate::common::intcode::v2::*;

/// Find the first packet sent to 255 in the network.
pub fn one(input: &str) -> crate::Result<Int> {
    run_network(input, Mode::BreakOnNat)
}

/// Find the first packet echoed from NAT to the first machine twice in a row.
pub fn two(input: &str) -> crate::Result<Int> {
    run_network(input, Mode::BreakOnRepeatEcho)
}

/// Decides what condition makes [`run_network`] terminate.
enum Mode {
    /// Terminate when a packet is first sent to adress 255.
    BreakOnNat,
    /// Terminate when the same packet is echoed from 255 to 0 twice in a row.
    BreakOnRepeatEcho,
}

/// Simulates the network as described in the puzzle, until a termination
/// condition is met, at which point the Y value of the relevant packet
/// returned. `mode` determines the actual termination condition.
fn run_network(input: &str, mode: Mode) -> crate::Result<Int> {
    const MACHINES: usize = 50;

    let p = Program::with_capacity(input, 2300, [])?;
    let mut ps = vec![p; MACHINES];

    for (i, p) in ps.iter_mut().enumerate() {
        p.run_with([i as i64])?;
    }

    let mut last_echoed = [0, 0];
    let mut stored = [0, 0];

    ps[0].input.push_back(-1);
    for i in (0..MACHINES).cycle() {
        // If, at the beginning of a cycle, all machines are idle (no input),
        // echo the last stored packet to machine #0.
        if i == 0 && ps.iter().all(|p| p.input.is_empty()) {
            // Termination condition for `BreakOnRepeatEcho`. Will only ever
            // be reached if that is `mode`.
            if stored == last_echoed {
                return Ok(stored[1]);
            }

            last_echoed = stored;
            ps[0].input.extend(stored);
        }

        // If we have no input, idle using input -1.
        if ps[i].input.is_empty() {
            ps[i].input.push_back(-1);
        }

        // Route outputs to the designated target machines. If the target is
        // 255, it's a NAT packet. Either store it, or, in `BreakOnNat`, just
        // return.
        ps[i].run()?;
        while ps[i].output.len() >= 3 {
            let addr = ps[i].output.pop_front().unwrap();
            let x = ps[i].output.pop_front().unwrap();
            let y = ps[i].output.pop_front().unwrap();

            if addr == 255 {
                if let Mode::BreakOnNat = mode {
                    return Ok(y);
                }
                stored = [x, y];
            } else {
                ps[addr as usize].input.extend([x, y]);
            }
        }
    }

    unreachable!()
}
