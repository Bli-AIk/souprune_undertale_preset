//! Code representation of `overworld/chase_config.ron`.
//!
//! `overworld/chase_config.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::overworld::*;
use souprune_vessel::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> ChaseConfig {
    ChaseConfig {
        heart_marker: HeartMarkerConfig {
            texture_path: "assets/textures/common/view/dr_heart.png".into(),
            offset: Vec2Config { x: 0.0, y: -5.0 },
            z_offset: 101.0,
            scale: 0.5,
            color: ColorConfig {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            fade_duration: 0.5,
        },
        outline: OutlineConfig {
            color: ColorConfig {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            fade_duration: 0.5,
            padding: 2.0,
            z_offset: 100.0,
        },
        dark_overlay: DarkOverlayConfig {
            target_alpha: 0.5,
            fade_duration: 0.5,
            overlay_size: 10000.0,
            z_offset: 50.0,
        },
        hitbox: HitboxConfig {
            shape: HitboxShapeConfig::Circle { radius: 4.0 },
            offset: Vec2Config { x: 0.0, y: -5.0 },
        },
        damage_ui: DamageUIConfig {
            layout_path: "overworld/view/damage_flash.view.ron".into(),
            display_duration: 2.5,
            damage_sound: Some("assets/audios/sfx/hurtsound.wav".into()),
        },
    }
}
