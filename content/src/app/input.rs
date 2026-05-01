//! Code representation of `app/input.ron`.
//!
//! `app/input.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::config::*;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> InputConfig {
    InputConfig {
        actions: vec![
            (
                "Down".into(),
                vec![
                    InputBinding::Key("ArrowDown".into()),
                    InputBinding::Key("KeyS".into()),
                    InputBinding::Gamepad("DPadDown".into()),
                ],
            ),
            (
                "Left".into(),
                vec![
                    InputBinding::Key("ArrowLeft".into()),
                    InputBinding::Key("KeyA".into()),
                    InputBinding::Gamepad("DPadLeft".into()),
                ],
            ),
            (
                "Cancel".into(),
                vec![
                    InputBinding::Key("KeyX".into()),
                    InputBinding::Key("ShiftLeft".into()),
                    InputBinding::Key("ShiftRight".into()),
                    InputBinding::Gamepad("East".into()),
                ],
            ),
            (
                "Up".into(),
                vec![
                    InputBinding::Key("ArrowUp".into()),
                    InputBinding::Key("KeyW".into()),
                    InputBinding::Gamepad("DPadUp".into()),
                ],
            ),
            (
                "Confirm".into(),
                vec![
                    InputBinding::Key("KeyZ".into()),
                    InputBinding::Key("Enter".into()),
                    InputBinding::Gamepad("South".into()),
                ],
            ),
            (
                "Menu".into(),
                vec![
                    InputBinding::Key("KeyC".into()),
                    InputBinding::Key("ControlLeft".into()),
                    InputBinding::Key("ControlRight".into()),
                    InputBinding::Gamepad("North".into()),
                ],
            ),
            (
                "Right".into(),
                vec![
                    InputBinding::Key("ArrowRight".into()),
                    InputBinding::Key("KeyD".into()),
                    InputBinding::Gamepad("DPadRight".into()),
                ],
            ),
        ]
        .into_iter()
        .collect(),
        navigation: NavigationConfig {
            up: Some("Up".into()),
            down: Some("Down".into()),
            left: Some("Left".into()),
            right: Some("Right".into()),
        },
        ui: UIConfig {
            confirm: Some("Confirm".into()),
            cancel: Some("Cancel".into()),
            menu: Some("Menu".into()),
        },
        touch_overlay: Some(TouchOverlayConfig {
            platforms: vec!["android".into(), "ios".into()],
            layout: Some("view/touch_layout.ron".into()),
            opacity: None,
            scale: None,
        }),
    }
}
