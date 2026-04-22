//! Bootstrapped static asset emitters for this content guest.
//!
//! 当前内容 guest 的 bootstrap 静态资产发射模块。

use anyhow::Result;
use souprune_vessel::prelude::*;

#[path = "static_assets/actors_items_basic_items_ron.rs"]
mod actors_items_basic_items_ron;
#[path = "static_assets/app_flow_ron.rs"]
mod app_flow_ron;
#[path = "static_assets/app_global_fre_ron.rs"]
mod app_global_fre_ron;
#[path = "static_assets/app_input_ron.rs"]
mod app_input_ron;
#[path = "static_assets/battle_common_end_view_interaction_sequence_ron.rs"]
mod battle_common_end_view_interaction_sequence_ron;
#[path = "static_assets/battle_common_enemy_turn_sequence_ron.rs"]
mod battle_common_enemy_turn_sequence_ron;
#[path = "static_assets/battle_common_fight_target_sequence_ron.rs"]
mod battle_common_fight_target_sequence_ron;
#[path = "static_assets/battle_common_narration_sequence_ron.rs"]
mod battle_common_narration_sequence_ron;
#[path = "static_assets/battle_common_player_turn_sequence_ron.rs"]
mod battle_common_player_turn_sequence_ron;
#[path = "static_assets/battle_common_show_narration_sequence_ron.rs"]
mod battle_common_show_narration_sequence_ron;
#[path = "static_assets/battle_mercy_end_sequence_ron.rs"]
mod battle_mercy_end_sequence_ron;
#[path = "static_assets/battle_mercy_spare_sequence_ron.rs"]
mod battle_mercy_spare_sequence_ron;
#[path = "static_assets/battle_players_player_battle_player_ron.rs"]
mod battle_players_player_battle_player_ron;
#[path = "static_assets/battle_rules_dialogue_stop_fre_ron.rs"]
mod battle_rules_dialogue_stop_fre_ron;
#[path = "static_assets/battle_rules_dialogue_test_fre_ron.rs"]
mod battle_rules_dialogue_test_fre_ron;
#[path = "static_assets/battle_rules_fight_hit_fre_ron.rs"]
mod battle_rules_fight_hit_fre_ron;
#[path = "static_assets/battle_rules_menu_cancel_fre_ron.rs"]
mod battle_rules_menu_cancel_fre_ron;
#[path = "static_assets/battle_rules_menu_confirm_fre_ron.rs"]
mod battle_rules_menu_confirm_fre_ron;
#[path = "static_assets/battle_rules_menu_navigation_fre_ron.rs"]
mod battle_rules_menu_navigation_fre_ron;
#[path = "static_assets/battle_templates_undertale_battle_sequence_ron.rs"]
mod battle_templates_undertale_battle_sequence_ron;
#[path = "static_assets/battle_view_undertale_view_ron.rs"]
mod battle_view_undertale_view_ron;
#[path = "static_assets/narrative_dialogue_fre_ron.rs"]
mod narrative_dialogue_fre_ron;
#[path = "static_assets/narrative_dialogue_ron.rs"]
mod narrative_dialogue_ron;
#[path = "static_assets/overworld_characters_frisk_animations_animation_config_ron.rs"]
mod overworld_characters_frisk_animations_animation_config_ron;
#[path = "static_assets/overworld_characters_frisk_character_ron.rs"]
mod overworld_characters_frisk_character_ron;
#[path = "static_assets/overworld_chase_config_ron.rs"]
mod overworld_chase_config_ron;
#[path = "static_assets/overworld_players_player_behavior_ron.rs"]
mod overworld_players_player_behavior_ron;
#[path = "static_assets/overworld_rules_backpack_fre_ron.rs"]
mod overworld_rules_backpack_fre_ron;
#[path = "static_assets/overworld_rules_interaction_fre_ron.rs"]
mod overworld_rules_interaction_fre_ron;
#[path = "static_assets/overworld_view_damage_flash_view_ron.rs"]
mod overworld_view_damage_flash_view_ron;
#[path = "static_assets/overworld_view_dialogue_view_ron.rs"]
mod overworld_view_dialogue_view_ron;
#[path = "static_assets/overworld_view_undertale_backpack_view_ron.rs"]
mod overworld_view_undertale_backpack_view_ron;
#[path = "static_assets/view_structures_attack_bar_sdf_ron.rs"]
mod view_structures_attack_bar_sdf_ron;
#[path = "static_assets/view_structures_hp_bar_sdf_ron.rs"]
mod view_structures_hp_bar_sdf_ron;
#[path = "static_assets/view_structures_view_box_sdf_ron.rs"]
mod view_structures_view_box_sdf_ron;
#[path = "static_assets/view_touch_layout_ron.rs"]
mod view_touch_layout_ron;

/// Emit all bootstrapped static assets for this mod.
///
/// 发射当前 mod 的全部 bootstrap 静态资产。
pub fn emit_all(reg: &mut Registry) -> Result<()> {
    actors_items_basic_items_ron::emit(reg)?;
    app_flow_ron::emit(reg)?;
    app_global_fre_ron::emit(reg)?;
    app_input_ron::emit(reg)?;
    battle_common_end_view_interaction_sequence_ron::emit(reg)?;
    battle_common_enemy_turn_sequence_ron::emit(reg)?;
    battle_common_fight_target_sequence_ron::emit(reg)?;
    battle_common_narration_sequence_ron::emit(reg)?;
    battle_common_player_turn_sequence_ron::emit(reg)?;
    battle_common_show_narration_sequence_ron::emit(reg)?;
    battle_mercy_end_sequence_ron::emit(reg)?;
    battle_mercy_spare_sequence_ron::emit(reg)?;
    battle_players_player_battle_player_ron::emit(reg)?;
    battle_rules_dialogue_stop_fre_ron::emit(reg)?;
    battle_rules_dialogue_test_fre_ron::emit(reg)?;
    battle_rules_fight_hit_fre_ron::emit(reg)?;
    battle_rules_menu_cancel_fre_ron::emit(reg)?;
    battle_rules_menu_confirm_fre_ron::emit(reg)?;
    battle_rules_menu_navigation_fre_ron::emit(reg)?;
    battle_templates_undertale_battle_sequence_ron::emit(reg)?;
    battle_view_undertale_view_ron::emit(reg)?;
    narrative_dialogue_fre_ron::emit(reg)?;
    narrative_dialogue_ron::emit(reg)?;
    overworld_characters_frisk_character_ron::emit(reg)?;
    overworld_characters_frisk_animations_animation_config_ron::emit(reg)?;
    overworld_chase_config_ron::emit(reg)?;
    overworld_players_player_behavior_ron::emit(reg)?;
    overworld_rules_backpack_fre_ron::emit(reg)?;
    overworld_rules_interaction_fre_ron::emit(reg)?;
    overworld_view_damage_flash_view_ron::emit(reg)?;
    overworld_view_dialogue_view_ron::emit(reg)?;
    overworld_view_undertale_backpack_view_ron::emit(reg)?;
    view_structures_attack_bar_sdf_ron::emit(reg)?;
    view_structures_hp_bar_sdf_ron::emit(reg)?;
    view_structures_view_box_sdf_ron::emit(reg)?;
    view_touch_layout_ron::emit(reg)?;
    Ok(())
}
