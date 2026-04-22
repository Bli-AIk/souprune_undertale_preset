//! Bootstrapped code asset for `battle/players/player.battle_player.ron`.
//!
//! `battle/players/player.battle_player.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::battle::*;
use souprune_schema::bevy_types::*;
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
pub fn asset() -> BattlePlayerConfig {
    BattlePlayerConfig {
        sprite_path: "assets/textures/common/view/heart.png".into(),
        color: BevyColor::Srgba(SrgbaColor {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }),
        physics_collider: ColliderConfig {
            shape: BattleColliderShape::Circle { radius: 8.0 },
            debug_z_offset: 10.0,
        },
        damage_trigger: ColliderConfig {
            shape: BattleColliderShape::Box {
                half_size: (2.0, 2.0),
            },
            debug_z_offset: 10.0,
        },
        z_position: 10.0,
        default_mode_id: "soul_red".into(),
        speed: 150.0,
        focus_speed_ratio: 0.5,
        default_box: "main".into(),
        invincibility: BattleInvincibilityConfig {
            duration: 1.0,
            flash_interval: 0.1,
            normal_color: BevyColor::Srgba(SrgbaColor {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            }),
            flash_color: BevyColor::Srgba(SrgbaColor {
                red: 0.5,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            }),
            damage_sound: Some("assets/audios/sfx/hurtsound.wav".into()),
        },
    }
}
