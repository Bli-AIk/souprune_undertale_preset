//! Code representation of `overworld/characters/frisk.character.ron`.
//!
//! `overworld/characters/frisk.character.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::character::*;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> CharacterAsset {
    CharacterAsset {
        name: "Frisk".into(),
        collider_size: Vec2XY { x: 20.0, y: 12.0 },
        collider_offset: Vec2XY { x: 0.0, y: -9.0 },
        base_speed: 100.0,
        animation_config: "overworld/characters/frisk/animations.animation_config.ron".into(),
        interaction_script: None,
    }
}
