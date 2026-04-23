//! Bootstrapped code asset for `app/flow.ron`.
//!
//! `app/flow.ron` 的 bootstrap 代码资源。

use anyhow::Result;
use souprune_schema::config::*;
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
pub fn asset() -> StateConfig {
    StateConfig {
        states: vec![
            (
                "Chase".into(),
                StateDefinition {
                    view_interactive: false,
                    player_movable: true,
                    player_can_interact: Some(false),
                    camera_follow_player: true,
                    view_layout: None,
                    initial_layer: None,
                    on_enter_sound: None,
                    on_exit_sound: None,
                    chase_config: Some("overworld/chase_config.ron".into()),
                },
            ),
            (
                "Cutscene".into(),
                StateDefinition {
                    view_interactive: false,
                    player_movable: false,
                    player_can_interact: None,
                    camera_follow_player: true,
                    view_layout: None,
                    initial_layer: None,
                    on_enter_sound: None,
                    on_exit_sound: None,
                    chase_config: None,
                },
            ),
            (
                "Normal".into(),
                StateDefinition {
                    view_interactive: false,
                    player_movable: true,
                    player_can_interact: None,
                    camera_follow_player: true,
                    view_layout: None,
                    initial_layer: None,
                    on_enter_sound: None,
                    on_exit_sound: None,
                    chase_config: None,
                },
            ),
            (
                "Dialogue".into(),
                StateDefinition {
                    view_interactive: false,
                    player_movable: false,
                    player_can_interact: None,
                    camera_follow_player: true,
                    view_layout: None,
                    initial_layer: None,
                    on_enter_sound: None,
                    on_exit_sound: None,
                    chase_config: None,
                },
            ),
            (
                "Backpack".into(),
                StateDefinition {
                    view_interactive: true,
                    player_movable: false,
                    player_can_interact: None,
                    camera_follow_player: true,
                    view_layout: Some("overworld/view/undertale_backpack.view.ron".into()),
                    initial_layer: None,
                    on_enter_sound: Some("assets/audios/sfx/confirm.wav".into()),
                    on_exit_sound: None,
                    chase_config: None,
                },
            ),
        ]
        .into_iter()
        .collect(),
    }
}
