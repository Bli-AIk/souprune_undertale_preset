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

mod behaviors;

use behaviors::{
    AimedSpear, FightBarBehavior, GravityDropDanmaku, RedSoul, SpiralHomingDanmaku,
    WaveBurstDanmaku,
};
use souprune_sdk::prelude::*;

export_mod! {
    behaviors: [
        ("soul_red", RedSoul, || RedSoul::new()),
        ("fight_bar", FightBarBehavior, || FightBarBehavior::new()),
    ],
    danmaku: [
        ("aimed_spear", AimedSpear, || AimedSpear::new()),
        ("spiral_homing", SpiralHomingDanmaku, || SpiralHomingDanmaku::new()),
        ("wave_burst", WaveBurstDanmaku, || WaveBurstDanmaku::new()),
        ("gravity_drop", GravityDropDanmaku, || GravityDropDanmaku::new()),
    ],
}
