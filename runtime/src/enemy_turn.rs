//! Undertale enemy turn selection implemented in the prelude runtime.
//!
//! 前置运行时中的 Undertale 敌人回合选择逻辑。

use souprune_sdk::prelude::*;

const ACTION_SELECT_ENEMY_TURN: &str = "SelectEnemyTurn";
const DEFAULT_OUTPUT_FACT: &str = "_selected_enemy_turn";

/// Return the custom action names handled by this module.
///
/// 返回本模块处理的自定义 action 名称。
pub fn handled_actions() -> Vec<String> {
    vec![ACTION_SELECT_ENEMY_TURN.into()]
}

/// Dispatch an enemy-turn custom action.
///
/// 分发敌人回合自定义 action。
pub fn handle_action(ctx: &Context, action_type: &str, params: &[ActionParam]) -> bool {
    if action_type != ACTION_SELECT_ENEMY_TURN {
        return false;
    }

    let pairs: Vec<(&str, &str)> = params
        .iter()
        .map(|param| (param.name.as_str(), param.value.as_str()))
        .collect();
    select_enemy_turn(ctx, &pairs);
    true
}

fn select_enemy_turn(ctx: &Context, params: &[(&str, &str)]) {
    let output = param(params, "output").unwrap_or(DEFAULT_OUTPUT_FACT);
    let enemy_id = param(params, "enemy_id").unwrap_or_default().trim();
    if enemy_id.is_empty() {
        ctx.warn("SelectEnemyTurn ignored: missing enemy_id");
        ctx.set_fact_string(output, "");
        return;
    }

    let group = resolve_group(
        ctx,
        enemy_id,
        param(params, "group").unwrap_or_default().trim(),
    );
    let prefix = turn_group_prefix(enemy_id, &group);
    let turns = read_string_list(ctx, &format!("{prefix}.turns"));
    if turns.is_empty() {
        ctx.warn(&format!(
            "SelectEnemyTurn ignored: enemy '{enemy_id}' group '{}' has no turns",
            display_group(&group)
        ));
        ctx.set_fact_string(output, "");
        return;
    }

    let strategy = ctx
        .get_fact_string(&format!("{prefix}.turn_strategy"))
        .unwrap_or_else(|| "Sequential".into());
    let index_key = format!("{prefix}.turn_index");
    let pool_key = format!("{prefix}.turn_pool");
    let counter_key = format!("{prefix}.turn_random_counter");
    let state = TurnSelectionState {
        index: ctx.get_fact_int(&index_key).unwrap_or(0),
        pool: parse_index_list(&ctx.get_fact_string(&pool_key).unwrap_or_default()),
        counter: ctx.get_fact_int(&counter_key).unwrap_or(0),
    };

    let selection = select_turn(&turns, &strategy, state);
    ctx.set_fact_string(output, &selection.path);
    if let Some(index) = selection.next_index {
        ctx.set_fact_int(&index_key, index);
    }
    if let Some(pool) = selection.next_pool {
        ctx.set_fact_string(&pool_key, &join_indices(&pool));
    }
    ctx.set_fact_int(&counter_key, selection.next_counter);
}

#[derive(Debug, Clone, PartialEq)]
struct TurnSelectionState {
    index: i64,
    pool: Vec<usize>,
    counter: i64,
}

#[derive(Debug, Clone, PartialEq)]
struct TurnSelection {
    path: String,
    next_index: Option<i64>,
    next_pool: Option<Vec<usize>>,
    next_counter: i64,
}

fn select_turn(turns: &[String], strategy: &str, state: TurnSelectionState) -> TurnSelection {
    match strategy {
        "Random" | "random" => select_random_turn(turns, state),
        "Shuffle" | "shuffle" => select_shuffle_turn(turns, state),
        _ => select_sequential_turn(turns, state),
    }
}

fn select_sequential_turn(turns: &[String], state: TurnSelectionState) -> TurnSelection {
    let current = state.index.max(0) as usize;
    let idx = current % turns.len();
    TurnSelection {
        path: turns[idx].clone(),
        next_index: Some(((current + 1) % turns.len()) as i64),
        next_pool: None,
        next_counter: state.counter,
    }
}

fn select_random_turn(turns: &[String], state: TurnSelectionState) -> TurnSelection {
    let counter = state.counter.saturating_add(1);
    let idx = pseudo_random_index(counter as u64, turns.len());
    TurnSelection {
        path: turns[idx].clone(),
        next_index: None,
        next_pool: None,
        next_counter: counter,
    }
}

fn select_shuffle_turn(turns: &[String], state: TurnSelectionState) -> TurnSelection {
    let mut counter = state.counter;
    let mut pool: Vec<usize> = state
        .pool
        .into_iter()
        .filter(|idx| *idx < turns.len())
        .collect();
    if pool.is_empty() {
        pool = (0..turns.len()).collect();
        counter = counter.saturating_add(1);
        shuffle_indices(&mut pool, counter as u64);
    }

    let idx = pool.remove(0);
    TurnSelection {
        path: turns[idx].clone(),
        next_index: None,
        next_pool: Some(pool),
        next_counter: counter,
    }
}

fn resolve_group(ctx: &Context, enemy_id: &str, requested: &str) -> String {
    if !requested.is_empty() {
        return requested.to_string();
    }
    read_string_list(ctx, &format!("{enemy_id}.turn_group_names"))
        .into_iter()
        .next()
        .unwrap_or_default()
}

fn turn_group_prefix(enemy_id: &str, group: &str) -> String {
    if group.is_empty() {
        enemy_id.to_string()
    } else {
        format!("{enemy_id}.{group}")
    }
}

fn display_group(group: &str) -> &str {
    if group.is_empty() { "(default)" } else { group }
}

fn read_string_list(ctx: &Context, key: &str) -> Vec<String> {
    ctx.get_fact_string_list(key)
        .unwrap_or_else(|| split_list(&ctx.get_fact_string(key).unwrap_or_default()))
}

fn split_list(value: &str) -> Vec<String> {
    value
        .split([',', '\n'])
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

fn parse_index_list(value: &str) -> Vec<usize> {
    split_list(value)
        .into_iter()
        .filter_map(|item| item.parse().ok())
        .collect()
}

fn join_indices(indices: &[usize]) -> String {
    indices
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn param<'a>(params: &'a [(&str, &str)], key: &str) -> Option<&'a str> {
    params
        .iter()
        .find_map(|(name, value)| (*name == key).then_some(*value))
}

fn pseudo_random_index(seed: u64, len: usize) -> usize {
    (xorshift64(seed.max(1)) % len as u64) as usize
}

fn shuffle_indices(indices: &mut [usize], seed: u64) {
    let mut state = seed.max(1);
    for i in (1..indices.len()).rev() {
        state = xorshift64(state);
        let j = (state % (i as u64 + 1)) as usize;
        indices.swap(i, j);
    }
}

fn xorshift64(mut value: u64) -> u64 {
    value ^= value << 13;
    value ^= value >> 7;
    value ^ (value << 17)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_selection_cycles_turn_index() {
        let turns = vec!["a.sequence.ron".to_string(), "b.sequence.ron".to_string()];
        let selection = select_turn(
            &turns,
            "Sequential",
            TurnSelectionState {
                index: 1,
                pool: Vec::new(),
                counter: 0,
            },
        );

        assert_eq!(selection.path, "b.sequence.ron");
        assert_eq!(selection.next_index, Some(0));
    }

    #[test]
    fn shuffle_selection_reuses_remaining_pool() {
        let turns = vec![
            "a.sequence.ron".to_string(),
            "b.sequence.ron".to_string(),
            "c.sequence.ron".to_string(),
        ];
        let selection = select_turn(
            &turns,
            "Shuffle",
            TurnSelectionState {
                index: 0,
                pool: vec![2, 1],
                counter: 3,
            },
        );

        assert_eq!(selection.path, "c.sequence.ron");
        assert_eq!(selection.next_pool, Some(vec![1]));
        assert_eq!(selection.next_counter, 3);
    }

    #[test]
    fn split_list_reads_comma_or_newline_encoded_fallbacks() {
        assert_eq!(
            split_list("a.sequence.ron,b.sequence.ron\nc.sequence.ron"),
            vec![
                "a.sequence.ron".to_string(),
                "b.sequence.ron".to_string(),
                "c.sequence.ron".to_string()
            ]
        );
    }
}
