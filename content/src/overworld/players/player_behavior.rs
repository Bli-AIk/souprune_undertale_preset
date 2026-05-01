//! Code representation of `overworld/players/player_behavior.ron`.
//!
//! `overworld/players/player_behavior.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::overworld::*;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

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
