//! Undertale-style game preset — reusable WASM behaviors.
//!
//! Provides standard Undertale battle behaviors:
//! - `soul_red`: Standard red soul movement (4-directional, focus mode)
//! - `fight_bar`: Attack bar sweep and timing logic
//!
//! Undertale 风格游戏预设 — 可复用的 WASM 行为。
//!
//! 提供标准 Undertale 战斗行为：
//! - `soul_red`：标准红魂移动（四方向、聚焦模式）
//! - `fight_bar`：攻击条扫描和计时逻辑

mod behaviors;

use behaviors::{FightBarBehavior, RedSoul};
use souprune_sdk::prelude::*;

export_mod! {
    behaviors: [
        ("soul_red", RedSoul, || RedSoul::new()),
        ("fight_bar", FightBarBehavior, || FightBarBehavior::new()),
    ],
    danmaku: [],
}
