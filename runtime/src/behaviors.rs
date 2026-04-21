//! Reusable Undertale-style WASM behaviors.
//!
//! 可复用的 Undertale 风格 WASM 行为。

mod aimed_spear;
mod fight_bar;
mod gravity_drop;
mod red_soul;
mod spiral_homing;
mod wave_burst;

pub use aimed_spear::AimedSpear;
pub use fight_bar::FightBarBehavior;
pub use gravity_drop::GravityDropDanmaku;
pub use red_soul::RedSoul;
pub use spiral_homing::SpiralHomingDanmaku;
pub use wave_burst::WaveBurstDanmaku;
