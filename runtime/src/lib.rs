//! Undertale-style game preset — reusable WASM behaviors.
//!
//! Provides standard Undertale battle behaviors:
//! - `soul_red`: Standard red soul movement (4-directional, focus mode)
//! - `fight_bar`: Attack bar sweep and timing logic
//! - `aimed_spear`: Aimed projectile behavior
//! - `spiral_homing`: Spiral homing projectile
//! - `wave_burst`: Wave burst projectile pattern
//! - `gravity_drop`: Gravity-affected falling projectile
//!
//! Undertale 风格游戏预设 — 可复用的 WASM 行为。

mod battle_box;
mod battle_player;
mod behaviors;
mod enemy_turn;
mod item_actions;

use battle_box::{BattleArea, BattleBoxActionHandler};
use behaviors::{
    AimedSpear, FightBarBehavior, GravityDropDanmaku, RedSoul, SpiralHomingDanmaku,
    WaveBurstDanmaku,
};
use souprune_sdk::prelude::*;

#[derive(Default)]
struct UndertaleActionHandler;

impl CustomActionHandler for UndertaleActionHandler {
    fn handled_actions() -> Vec<String> {
        let mut actions = BattleBoxActionHandler::handled_actions();
        actions.extend(battle_player::handled_actions());
        actions.extend(enemy_turn::handled_actions());
        actions.extend(item_actions::handled_actions());
        actions
    }

    fn handle_action(&self, ctx: &Context, action_type: &str, params: &[ActionParam]) -> bool {
        BattleBoxActionHandler.handle_action(ctx, action_type, params)
            || battle_player::handle_action(ctx, action_type, params)
            || enemy_turn::handle_action(ctx, action_type, params)
            || item_actions::handle_action(ctx, action_type, params)
    }
}

export_mod! {
    behaviors: [
        ("undertale_battle_area", BattleArea, || BattleArea::new()),
        ("soul_red", RedSoul, || RedSoul::new()),
        ("fight_bar", FightBarBehavior, || FightBarBehavior::new()),
    ],
    danmaku: [
        ("aimed_spear", AimedSpear, || AimedSpear::new()),
        ("spiral_homing", SpiralHomingDanmaku, || SpiralHomingDanmaku::new()),
        ("wave_burst", WaveBurstDanmaku, || WaveBurstDanmaku::new()),
        ("gravity_drop", GravityDropDanmaku, || GravityDropDanmaku::new()),
    ],
    patterns: [],
    custom_actions: UndertaleActionHandler,
}
