use std::collections::VecDeque;

/// Find the sum of the quality levels of each blueprint.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(parse(input)
        .enumerate()
        .map(|(i, blueprint)| best_geodes(24, blueprint) * (i as i32 + 1))
        .sum())
}

/// Find the product of geode counts for the first three blueprints.
pub fn two(input: &str) -> crate::Result<i32> {
    Ok(parse(input)
        .take(3)
        .map(|blueprint| best_geodes(32, blueprint))
        .product())
}

/// Find the best possible amount of geodes that can be cracked in the given `time` using
/// the provided blueprint.
///
/// Uses a simple breadth-first search with pruning of branches that fall behind too much
/// on geode production in relation to the others; as well as some other optimizations.
fn best_geodes(time: i32, costs: Blueprint) -> i32 {
    let max_ore_gain = costs.into_iter().map(|costs| costs[0]).max().unwrap();
    let max_clay_gain = costs.into_iter().map(|costs| costs[1]).max().unwrap();
    let max_obsidian_gain = costs.into_iter().map(|costs| costs[2]).max().unwrap();
    let bot_caps = [max_ore_gain, max_clay_gain, max_obsidian_gain, i32::MAX];

    let info = Info { bot_caps, costs };

    let mut queue = VecDeque::from([State::new(time)]);
    let mut best = 0;

    while let Some(state) = queue.pop_front() {
        best = best.max(state.ores[3]);

        // We can't catch up anymore.
        if best - 2 > state.ores[3] {
            continue;
        }

        if let Some(state) = state.with_built(&info, Some(3)) {
            // If we can build a geode bot, we always just do that.
            queue.push_back(state);
        } else {
            // Investigate all other possible futures.
            let futures = [0, 1, 2].map(|n| state.with_built(&info, Some(n)));
            queue.extend(futures.into_iter().flatten());

            // If we can build *any* bot, there's no reason not to. But if we can't, we also
            // see what happens if we don't build any.
            if !futures.iter().all(|future| future.is_some()) {
                queue.extend(state.with_built(&info, None));
            }
        }
    }

    best
}

/// Data that doesn't change during an invocation of [`best_geode`].
struct Info {
    bot_caps: Bots,
    costs: Blueprint,
}

/// A state being checked.
#[derive(Copy, Clone)]
struct State {
    time: i32,
    ores: Ores,
    bots: Bots,
    allowed: [bool; 4],
}

impl State {
    /// Creates a new default state with the given amount of time left.
    fn new(time: i32) -> State {
        Self {
            time,
            ores: [0; 4],
            bots: [1, 0, 0, 0],
            /// Keeps track of which bots we're allowed to build. We aren't allowed to build
            /// a bot that we COULD have built earlier, but haven't. That's because doing so
            /// would be a strictly worse outcome, so we don't need to bother checking it.
            allowed: [true; 4],
        }
    }

    /// Attempts to build the selected `bot` (`None` means just advancing time, without building
    /// anything). If it was possible to build the bot, returns the new state; otherwise `None`.
    fn with_built(&self, info: &Info, bot: Option<usize>) -> Option<Self> {
        if self.time == 0 {
            return None;
        }

        match bot {
            None => {
                // Mark all bots we COULD have built as forbidden.
                let mut result = *self;
                for bot in 0..self.bots.len() {
                    result.allowed[bot] = !self.can_build(info, bot);
                }
                result.advance();
                Some(result)
            }
            Some(bot) => self.can_build(info, bot).then(|| {
                let mut result = *self;

                // Spend resources.
                for (ore, cost) in result.ores.iter_mut().zip(info.costs[bot].into_iter()) {
                    *ore -= cost;
                }

                // Advance time BEFORE gaining the bot.
                result.advance();

                // Gain bot; all bots are allowed again.
                result.bots[bot] += 1;
                result.allowed = [true; 4];
                result
            }),
        }
    }

    /// Advances the state by one minute, which also gains one tick of ore production.
    fn advance(&mut self) {
        self.time -= 1;
        for (ore, bots) in self.ores.iter_mut().zip(self.bots.into_iter()) {
            *ore += bots;
        }
    }

    /// Checks whether the given bot can be construction, given various limitations.
    fn can_build(&self, info: &Info, bot: usize) -> bool {
        self.allowed[bot]
            && self.bots[bot] < info.bot_caps[bot]
            && self
                .ores
                .iter()
                .zip(info.costs[bot].iter())
                .all(|(o, c)| c <= o)
    }
}

type Ores = [i32; 4];
type Bots = [i32; 4];
type Blueprint = [Ores; 4];

/// Parses the puzzle input into a sequence of blueprints.
fn parse(input: &str) -> impl Iterator<Item = Blueprint> + '_ {
    fn get_cost<'a>(s: &'a str, indices: &[usize]) -> Option<(Ores, &'a str)> {
        let mut result = [0; 4];
        let (cost, s) = s.split_once("costs ")?.1.split_once('.')?;
        let costs = cost.split_ascii_whitespace().filter_map(|v| v.parse().ok());
        for (n, &i) in costs.zip(indices) {
            result[i] = n;
        }
        Some((result, s))
    }

    input.lines().filter_map(|line| {
        let line = line.split_once(": ")?.1;
        let (ore_cost, line) = get_cost(line, &[0])?;
        let (clay_cost, line) = get_cost(line, &[0])?;
        let (obsidian_cost, line) = get_cost(line, &[0, 1])?;
        let (geode_cost, _) = get_cost(line, &[0, 2])?;

        Some([ore_cost, clay_cost, obsidian_cost, geode_cost])
    })
}
