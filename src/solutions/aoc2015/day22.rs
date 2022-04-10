/// Find the cheapest possible win.
pub fn one(input: &str) -> Result<String, String> {
    Ok(least_mana_win(parse(input)?, false).to_string())
}

/// Find the cheapest possible win in hard mode.
pub fn two(input: &str) -> Result<String, String> {
    Ok(least_mana_win(parse(input)?, true).to_string())
}

/// Runs through all possible outcomes, and finds the cheapest win.
fn least_mana_win(initial: State, hard: bool) -> i32 {
    let (mut realities, mut best_win) = (vec![initial], i32::MAX);

    while let Some(state) = realities.pop() {
        // We've already spent more than the cheapest win so far; no need to keep going.
        if state.spent > best_win {
            continue;
        }

        // Record the win.
        if state.enemy_health <= 0 {
            best_win = best_win.min(state.spent);
            continue;
        }

        // Try casting each spell, and keep the outcomes where we live.
        realities.extend((0..SPELLS.len()).filter_map(|s| simulate_turn(state, s, hard)));
    }

    best_win
}

/// Simulates one full player turn and one full enemy turn, casting the provided `spell`. Returns
/// `None` if the outcome is ad (we died or were unable to cast the spell), otherwise
/// `Some(new state)`. If `hard` is set, the player loses 1 HP at the beginning of each turn.
fn simulate_turn(mut state: State, spell: usize, hard: bool) -> Option<State> {
    // Advances all effects by one turn, and applies any damage or mana regen they do.
    fn tick_effects(state: &mut State) {
        if state.effects[1] > 0 {
            state.enemy_health -= 3;
        }
        if state.effects[2] > 0 {
            state.mana += 101;
        }
        state.effects[0] -= 1;
        state.effects[1] -= 1;
        state.effects[2] -= 1;
    }

    // Player turn.
    if hard {
        state.health -= 1;
        if state.health <= 0 {
            return None;
        }
    }

    tick_effects(&mut state);
    if state.enemy_health <= 0 {
        return Some(state);
    }

    let (cost, damage, healing, effect, duration) = *SPELLS.get(spell)?;

    let success = {
        let cost_met = cost <= state.mana;
        let effect_available = effect.map(|i| state.effects[i]).unwrap_or(0) <= 0;
        cost_met && effect_available
    };

    if !success {
        return None;
    }

    state.mana -= cost;
    state.spent += cost;
    state.enemy_health -= damage;
    state.health += healing;
    if let Some(i) = effect {
        state.effects[i] = duration;
    }

    // Enemy turn.
    tick_effects(&mut state);
    if state.enemy_health <= 0 {
        return Some(state);
    }

    let damage = state.enemy_damage - 0.max(state.effects[0]).signum() * 7;
    state.health -= damage;
    if state.health <= 0 {
        return None;
    }

    Some(state)
}

/// Spell data. In order: mana cost, damage dealt, healing done, effect inflicted, duration of
/// inflicted effect. Effects are 0 for shield, 1 for poison, 2 for recharge.
static SPELLS: [(i32, i32, i32, Option<usize>, i32); 5] = [
    (53, 4, 0, None, -1),    // Magic Missile
    (73, 2, 2, None, -1),    // Drain
    (113, 0, 0, Some(0), 6), // Shield
    (173, 0, 0, Some(1), 6), // Poison
    (229, 0, 0, Some(2), 5), // Recharge
];

/// A battle state.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
struct State {
    health: i32,
    mana: i32,
    spent: i32,

    enemy_health: i32,
    enemy_damage: i32,

    effects: [i32; 3],
}

/// Parses the puzzle input into a valid initial battle state.
fn parse(input: &str) -> Result<State, String> {
    let data: Vec<_> = input
        .lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|v| v.1.parse().ok())
        .collect();
    if let &[enemy_health, enemy_damage] = data.as_slice() {
        Ok(State {
            health: 50,
            mana: 500,
            enemy_health,
            enemy_damage,
            ..Default::default()
        })
    } else {
        Err(format!("failed to parse puzzle input"))
    }
}
