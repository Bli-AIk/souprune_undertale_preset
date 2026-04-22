//! Bootstrapped code asset for `overworld/players/player_behavior.ron`.
//!
//! `overworld/players/player_behavior.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::overworld::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 发射当前 bootstrap 资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资产的类型化值。
pub fn asset() -> PlayerBehaviorFile {
    PlayerBehaviorFile {
        character_asset: "overworld/characters/frisk.character.ron".into(),
        spawn_position: Vec2Config { x: 0.0, y: 0.0 },
        initial_facing: Direction::Down,
        initial_state: "Walk".into(),
        run: Some(RunConfig {
            action: "Cancel".into(),
            speed_multiplier: 2.0,
        }),
        invincibility: Some(OverworldInvincibilityConfig {
            duration: 1.0,
            flash_interval: 0.25,
            normal_color: "#FF0000".into(),
            flash_color: "#800000".into(),
        }),
    }
}
