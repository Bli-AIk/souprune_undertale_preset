//! Undertale item Use/Check/Drop actions implemented in the prelude runtime.
//!
//! 前置运行时中的 Undertale 物品使用、查看与丢弃逻辑。

use souprune_sdk::prelude::*;

const ACTION_USE_ITEM: &str = "UseItem";
const ACTION_CHECK_ITEM: &str = "CheckItem";
const ACTION_DROP_ITEM: &str = "DropItem";

const PLAYER_INVENTORY: &str = "player:inventory";
const PLAYER_HP: &str = "player:hp";
const PLAYER_HP_MAX: &str = "player:hp_max";
const PLAYER_WEAPON: &str = "player:weapon";
const PLAYER_ARMOR: &str = "player:armor";

const DIALOGUE_DEFAULT_CHANNEL: &str = "main";
const DIALOGUE_PENDING_VIEW: &str = "dialogue:pending_view";
const DIALOGUE_PENDING_MORTAR_PATH: &str = "dialogue:pending_mortar_path";
const DIALOGUE_PENDING_MORTAR_NODE: &str = "dialogue:pending_mortar_node";
const DIALOGUE_HAS_TYPEWRITER: &str = "dialogue:has_typewriter";
const DIALOGUE_HAS_FOCUS: &str = "dialogue:has_focus";
const DIALOGUE_VOICE: &str = "dialogue:voice";
const DIALOGUE_PENDING_START: &str = "dialogue:pending_start";
const DIALOGUE_ITEM_NAME: &str = "dialogue:item_name";
const DIALOGUE_ITEM_DESCRIPTION: &str = "dialogue:item_description";
const DIALOGUE_ITEM_HEAL_AMOUNT: &str = "dialogue:item_heal_amount";
const DIALOGUE_ITEM_VALUE: &str = "dialogue:item_value";

const MORTAR_PATH_FACT: &str = "mortar_path";
const ACTION_PARAM_FACT: &str = "action_param";

const DEFAULT_ITEM_MORTAR: &str = "items/_defaults.mortar";
const DEFAULT_DIALOGUE_VIEW: &str = "overworld/view/dialogue.view.ron";
const DEFAULT_DIALOGUE_VOICE: &str = "assets/audios/voice/voice_monster.wav";

/// Return the custom action names handled by this module.
///
/// 返回本模块处理的自定义 action 名称。
pub fn handled_actions() -> Vec<String> {
    vec![
        ACTION_USE_ITEM.into(),
        ACTION_CHECK_ITEM.into(),
        ACTION_DROP_ITEM.into(),
    ]
}

/// Dispatch an item custom action.
///
/// 分发物品自定义 action。
pub fn handle_action(ctx: &Context, action_type: &str, params: &[ActionParam]) -> bool {
    let pairs: Vec<(&str, &str)> = params
        .iter()
        .map(|param| (param.name.as_str(), param.value.as_str()))
        .collect();

    match action_type {
        ACTION_USE_ITEM => {
            use_item(ctx, &pairs);
            true
        }
        ACTION_CHECK_ITEM => {
            check_item(ctx, &pairs);
            true
        }
        ACTION_DROP_ITEM => {
            drop_item(ctx, &pairs);
            true
        }
        _ => false,
    }
}

fn use_item(ctx: &Context, params: &[(&str, &str)]) {
    let Some(index) = resolve_index_expr(ctx, param(params, "index_expr").unwrap_or_default())
    else {
        return;
    };
    let Some(item_id) = inventory_item_id(ctx, index) else {
        ctx.warn(&format!("UseItem ignored: no item at index {index}"));
        return;
    };
    let Some(item_type) = item_type(ctx, &item_id) else {
        ctx.warn(&format!("UseItem ignored: item '{item_id}' has no type fact"));
        return;
    };

    let actual_healed = match item_type.as_str() {
        "Food" => apply_food_effects(ctx, &item_id, index),
        "Weapon" => {
            equip_item(ctx, &item_id, index, PLAYER_WEAPON);
            0
        }
        "Armor" => {
            equip_item(ctx, &item_id, index, PLAYER_ARMOR);
            0
        }
        _ => 0,
    };

    let (mortar_path, action_param) = use_dialogue_target(ctx, &item_id, &item_type);
    ctx.set_fact_string(MORTAR_PATH_FACT, &mortar_path);
    ctx.set_fact_string(ACTION_PARAM_FACT, action_param);

    let item_data = ItemDialogueData {
        locale_key: item_locale_key(ctx, &item_id).unwrap_or_default(),
        description: item_description(ctx, &item_id),
        heal_amount: actual_healed,
        item_value: compute_item_value(ctx, &item_id, &item_type),
    };

    if param(params, "start_dialogue").is_some_and(|value| value == "true") {
        start_item_dialogue(ctx, &mortar_path, action_param, item_data);
    } else {
        set_item_dialogue_data(ctx, item_data);
    }
}

fn check_item(ctx: &Context, params: &[(&str, &str)]) {
    let Some(index) = resolve_index_expr(ctx, param(params, "index_expr").unwrap_or_default())
    else {
        return;
    };
    let Some(item_id) = inventory_item_id(ctx, index) else {
        ctx.warn(&format!("CheckItem ignored: no item at index {index}"));
        return;
    };
    let Some(item_type) = item_type(ctx, &item_id) else {
        ctx.warn(&format!(
            "CheckItem ignored: item '{item_id}' has no type fact"
        ));
        return;
    };

    let (mortar_path, node) = check_dialogue_target(ctx, &item_id, &item_type);
    start_item_dialogue(
        ctx,
        &mortar_path,
        node,
        ItemDialogueData {
            locale_key: item_locale_key(ctx, &item_id).unwrap_or_default(),
            description: item_description(ctx, &item_id),
            heal_amount: 0,
            item_value: compute_item_value(ctx, &item_id, &item_type),
        },
    );
}

fn drop_item(ctx: &Context, params: &[(&str, &str)]) {
    let Some(index) = resolve_index_expr(ctx, param(params, "index_expr").unwrap_or_default())
    else {
        return;
    };
    let Some(item_id) = inventory_item_id(ctx, index) else {
        ctx.warn(&format!("DropItem ignored: no item at index {index}"));
        return;
    };
    let Some(item_type) = item_type(ctx, &item_id) else {
        ctx.warn(&format!("DropItem ignored: item '{item_id}' has no type fact"));
        return;
    };

    if item_type == "KeyItem" {
        return;
    }

    let mut inventory = inventory(ctx);
    remove_inventory_index(&mut inventory, index);
    ctx.set_global_fact_string_list(PLAYER_INVENTORY, &inventory);

    let (mortar_path, node) = drop_dialogue_target(ctx, &item_id);
    start_item_dialogue(
        ctx,
        &mortar_path,
        node,
        ItemDialogueData {
            locale_key: item_locale_key(ctx, &item_id).unwrap_or_default(),
            description: item_description(ctx, &item_id),
            heal_amount: 0,
            item_value: compute_item_value(ctx, &item_id, &item_type),
        },
    );
}

fn resolve_index_expr(ctx: &Context, index_expr: &str) -> Option<usize> {
    let expr = index_expr.trim();
    if expr.is_empty() {
        ctx.warn("Item action ignored: missing index_expr");
        return None;
    }

    let value = if let Some(fact_key) = expr.strip_prefix('$') {
        ctx.get_fact_int(fact_key)
    } else {
        expr.parse().ok()
    };

    match value {
        Some(index) if index >= 0 => Some(index as usize),
        _ => {
            ctx.warn(&format!(
                "Item action ignored: index_expr '{index_expr}' did not resolve to non-negative int"
            ));
            None
        }
    }
}

fn inventory_item_id(ctx: &Context, index: usize) -> Option<String> {
    inventory(ctx).get(index).cloned()
}

fn inventory(ctx: &Context) -> Vec<String> {
    ctx.get_fact_string_list(PLAYER_INVENTORY)
        .unwrap_or_default()
}

fn item_fact_key(item_id: &str, field: &str) -> String {
    format!("items:{item_id}.{field}")
}

fn item_type(ctx: &Context, item_id: &str) -> Option<String> {
    ctx.get_fact_string(&item_fact_key(item_id, "type"))
}

fn item_mortar(ctx: &Context, item_id: &str) -> Option<String> {
    ctx.get_fact_string(&item_fact_key(item_id, "mortar"))
}

fn item_locale_key(ctx: &Context, item_id: &str) -> Option<String> {
    ctx.get_fact_string(&item_fact_key(item_id, "locale_key"))
}

fn item_description(ctx: &Context, item_id: &str) -> String {
    ctx.get_fact_string(&item_fact_key(item_id, "description"))
        .unwrap_or_default()
}

fn item_heal(ctx: &Context, item_id: &str) -> Option<i64> {
    ctx.get_fact_int(&item_fact_key(item_id, "heal"))
}

fn item_consumable(ctx: &Context, item_id: &str) -> bool {
    ctx.get_fact_bool(&item_fact_key(item_id, "consumable"))
        .unwrap_or(false)
}

fn item_use_audio(ctx: &Context, item_id: &str) -> Option<String> {
    ctx.get_fact_string(&item_fact_key(item_id, "use_audio"))
}

fn item_child_item(ctx: &Context, item_id: &str) -> Option<String> {
    ctx.get_fact_string(&item_fact_key(item_id, "child_item"))
}

fn apply_food_effects(ctx: &Context, item_id: &str, index: usize) -> i64 {
    let mut actual_healed = 0;
    if let Some(heal_amount) = item_heal(ctx, item_id) {
        let hp = ctx.get_fact_int(PLAYER_HP).unwrap_or(0);
        let hp_max = ctx.get_fact_int(PLAYER_HP_MAX).unwrap_or(20);
        let new_hp = (hp + heal_amount).min(hp_max);
        actual_healed = new_hp - hp;
        ctx.set_global_fact_int(PLAYER_HP, new_hp);
    }

    if let Some(clip_path) = item_use_audio(ctx, item_id) {
        ctx.play_sound_full_path(&clip_path);
    }

    let mut inventory = inventory(ctx);
    if index < inventory.len() {
        if let Some(child_id) = item_child_item(ctx, item_id) {
            inventory[index] = child_id;
        } else if item_consumable(ctx, item_id) {
            inventory.remove(index);
        }
        ctx.set_global_fact_string_list(PLAYER_INVENTORY, &inventory);
    }

    actual_healed
}

fn equip_item(ctx: &Context, item_id: &str, index: usize, equipment_fact: &str) {
    let old_item = ctx.get_fact_string(equipment_fact).unwrap_or_default();
    ctx.set_global_fact_string(equipment_fact, item_id);

    let mut inventory = inventory(ctx);
    if index >= inventory.len() {
        return;
    }

    if old_item.is_empty() {
        inventory.remove(index);
    } else {
        inventory[index] = old_item;
    }
    ctx.set_global_fact_string_list(PLAYER_INVENTORY, &inventory);
}

fn compute_item_value(ctx: &Context, item_id: &str, item_type: &str) -> i64 {
    match item_type {
        "Food" => item_heal(ctx, item_id).unwrap_or(0),
        "Weapon" => ctx
            .get_fact_int(&item_fact_key(item_id, "damage"))
            .unwrap_or(0),
        "Armor" => ctx
            .get_fact_int(&item_fact_key(item_id, "defense"))
            .unwrap_or(0),
        _ => 0,
    }
}

fn use_dialogue_target(ctx: &Context, item_id: &str, item_type: &str) -> (String, &'static str) {
    item_mortar(ctx, item_id)
        .map(|mortar| (mortar, "OnUse"))
        .unwrap_or_else(|| (DEFAULT_ITEM_MORTAR.into(), default_use_node(item_type)))
}

fn check_dialogue_target(ctx: &Context, item_id: &str, item_type: &str) -> (String, &'static str) {
    item_mortar(ctx, item_id)
        .map(|mortar| (mortar, "OnCheck"))
        .unwrap_or_else(|| (DEFAULT_ITEM_MORTAR.into(), default_check_node(item_type)))
}

fn drop_dialogue_target(ctx: &Context, item_id: &str) -> (String, &'static str) {
    item_mortar(ctx, item_id)
        .map(|mortar| (mortar, "OnDrop"))
        .unwrap_or_else(|| (DEFAULT_ITEM_MORTAR.into(), "OnDropDefault"))
}

fn default_use_node(item_type: &str) -> &'static str {
    match item_type {
        "Food" => "OnUseFoodDefault",
        "Weapon" => "OnUseWeaponDefault",
        "Armor" => "OnUseArmorDefault",
        _ => "OnUseKeyItemDefault",
    }
}

fn default_check_node(item_type: &str) -> &'static str {
    match item_type {
        "Food" => "OnCheckFoodDefault",
        "Weapon" => "OnCheckWeaponDefault",
        "Armor" => "OnCheckArmorDefault",
        _ => "OnCheckDefault",
    }
}

struct ItemDialogueData {
    locale_key: String,
    description: String,
    heal_amount: i64,
    item_value: i64,
}

fn start_item_dialogue(
    ctx: &Context,
    mortar_path: &str,
    node: &str,
    item_data: ItemDialogueData,
) {
    ctx.set_fact_string(DIALOGUE_PENDING_VIEW, &dialogue_view_default(ctx));
    ctx.set_fact_string(DIALOGUE_PENDING_MORTAR_PATH, mortar_path);
    ctx.set_fact_string(DIALOGUE_PENDING_MORTAR_NODE, node);
    ctx.set_fact_bool(DIALOGUE_HAS_TYPEWRITER, true);
    ctx.set_fact_bool(DIALOGUE_HAS_FOCUS, true);
    ctx.set_fact_bool(
        &dialogue_channel_key(DIALOGUE_DEFAULT_CHANNEL, "has_typewriter"),
        true,
    );
    ctx.set_fact_bool(
        &dialogue_channel_key(DIALOGUE_DEFAULT_CHANNEL, "has_focus"),
        true,
    );

    let voice = dialogue_voice_default(ctx);
    if !voice.is_empty() {
        ctx.set_fact_string(DIALOGUE_VOICE, &voice);
        ctx.set_fact_string(&dialogue_channel_key(DIALOGUE_DEFAULT_CHANNEL, "voice"), &voice);
    }

    set_item_dialogue_data(ctx, item_data);
    ctx.set_fact_bool(DIALOGUE_PENDING_START, true);
}

fn set_item_dialogue_data(ctx: &Context, item_data: ItemDialogueData) {
    ctx.set_fact_string(DIALOGUE_ITEM_NAME, &item_data.locale_key);
    ctx.set_fact_string(DIALOGUE_ITEM_DESCRIPTION, &item_data.description);
    ctx.set_fact_int(DIALOGUE_ITEM_HEAL_AMOUNT, item_data.heal_amount);
    ctx.set_fact_int(DIALOGUE_ITEM_VALUE, item_data.item_value);
}

fn dialogue_view_default(ctx: &Context) -> String {
    ctx.get_fact_string("ut:dialogue_view_default")
        .unwrap_or_else(|| DEFAULT_DIALOGUE_VIEW.into())
}

fn dialogue_voice_default(ctx: &Context) -> String {
    ctx.get_fact_string("ut:dialogue_voice_default")
        .unwrap_or_else(|| DEFAULT_DIALOGUE_VOICE.into())
}

fn dialogue_channel_key(channel: &str, field: &str) -> String {
    format!("dialogue:{channel}:{field}")
}

fn remove_inventory_index(inventory: &mut Vec<String>, index: usize) {
    if index < inventory.len() {
        inventory.remove(index);
    }
}

fn param<'a>(params: &'a [(&str, &str)], key: &str) -> Option<&'a str> {
    params
        .iter()
        .find_map(|(name, value)| (*name == key).then_some(*value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_use_dialogue_node_depends_on_item_type() {
        assert_eq!(default_use_node("Food"), "OnUseFoodDefault");
        assert_eq!(default_use_node("Weapon"), "OnUseWeaponDefault");
        assert_eq!(default_use_node("Armor"), "OnUseArmorDefault");
        assert_eq!(default_use_node("KeyItem"), "OnUseKeyItemDefault");
    }

    #[test]
    fn remove_inventory_index_ignores_out_of_range_index() {
        let mut inventory = vec!["a".to_string(), "b".to_string()];

        remove_inventory_index(&mut inventory, 9);

        assert_eq!(inventory, vec!["a".to_string(), "b".to_string()]);
    }
}
