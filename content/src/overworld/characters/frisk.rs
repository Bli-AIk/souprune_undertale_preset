//! Bootstrapped code asset for `overworld/characters/frisk.character.ron`.
//!
//! `overworld/characters/frisk.character.ron` 的 bootstrap 代码资源。

use anyhow::Result;
use souprune_schema::character::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 生成当前 bootstrap 资源。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资源的类型化值。
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
