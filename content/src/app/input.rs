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
                    InputBinding::Key(KeyboardKey::ArrowDown),
                    InputBinding::Key(KeyboardKey::KeyS),
                    InputBinding::Gamepad("DPadDown".into()),
                ],
            ),
            (
                "Left".into(),
                vec![
                    InputBinding::Key(KeyboardKey::ArrowLeft),
                    InputBinding::Key(KeyboardKey::KeyA),
                    InputBinding::Gamepad("DPadLeft".into()),
                ],
            ),
            (
                "Cancel".into(),
                vec![
                    InputBinding::Key(KeyboardKey::KeyX),
                    InputBinding::Key(KeyboardKey::ShiftLeft),
                    InputBinding::Key(KeyboardKey::ShiftRight),
                    InputBinding::Gamepad("East".into()),
                ],
            ),
            (
                "Up".into(),
                vec![
                    InputBinding::Key(KeyboardKey::ArrowUp),
                    InputBinding::Key(KeyboardKey::KeyW),
                    InputBinding::Gamepad("DPadUp".into()),
                ],
            ),
            (
                "Confirm".into(),
                vec![
                    InputBinding::Key(KeyboardKey::KeyZ),
                    InputBinding::Key(KeyboardKey::Enter),
                    InputBinding::Gamepad("South".into()),
                ],
            ),
            (
                "Menu".into(),
                vec![
                    InputBinding::Key(KeyboardKey::KeyC),
                    InputBinding::Key(KeyboardKey::ControlLeft),
                    InputBinding::Key(KeyboardKey::ControlRight),
                    InputBinding::Gamepad("North".into()),
                ],
            ),
            (
                "Right".into(),
                vec![
                    InputBinding::Key(KeyboardKey::ArrowRight),
                    InputBinding::Key(KeyboardKey::KeyD),
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
