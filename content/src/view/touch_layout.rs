//! Code representation of `view/touch_layout.ron`.
//!
//! `view/touch_layout.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::config::*;
use souprune_vessel::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> TouchLayoutDef {
    TouchLayoutDef {
        opacity: 0.9,
        scale: 1.0,
        mobile_scale: 0.75,
        controller: Some(TouchControllerDef {
            anchor: TouchAnchor::BottomLeft,
            offset_x: 16.0,
            offset_y: 16.0,
            size: 360.0,
            base_texture: "assets/textures/common/touch/xbox/controller.png".into(),
            overlays: vec![
                (
                    "Down".into(),
                    "assets/textures/common/touch/xbox/controller_down.png".into(),
                ),
                (
                    "Left".into(),
                    "assets/textures/common/touch/xbox/controller_left.png".into(),
                ),
                (
                    "Up".into(),
                    "assets/textures/common/touch/xbox/controller_up.png".into(),
                ),
                (
                    "Right".into(),
                    "assets/textures/common/touch/xbox/controller_right.png".into(),
                ),
            ]
            .into_iter()
            .collect(),
        }),
        buttons: vec![
            TouchButtonDef {
                action: "Confirm".into(),
                texture: None,
                pressed_texture: None,
                frames: Some(vec![
                    "assets/textures/common/touch/xbox/a_0.png".into(),
                    "assets/textures/common/touch/xbox/a_1.png".into(),
                    "assets/textures/common/touch/xbox/a_2.png".into(),
                    "assets/textures/common/touch/xbox/a_3.png".into(),
                ]),
                label: None,
                anchor: TouchAnchor::BottomRight,
                offset_x: 112.0,
                offset_y: 20.0,
                width: 128.0,
                height: 128.0,
            },
            TouchButtonDef {
                action: "Cancel".into(),
                texture: None,
                pressed_texture: None,
                frames: Some(vec![
                    "assets/textures/common/touch/xbox/b_0.png".into(),
                    "assets/textures/common/touch/xbox/b_1.png".into(),
                    "assets/textures/common/touch/xbox/b_2.png".into(),
                    "assets/textures/common/touch/xbox/b_3.png".into(),
                ]),
                label: None,
                anchor: TouchAnchor::BottomRight,
                offset_x: 20.0,
                offset_y: 112.0,
                width: 128.0,
                height: 128.0,
            },
            TouchButtonDef {
                action: "Menu".into(),
                texture: None,
                pressed_texture: None,
                frames: Some(vec![
                    "assets/textures/common/touch/xbox/x_0.png".into(),
                    "assets/textures/common/touch/xbox/x_1.png".into(),
                    "assets/textures/common/touch/xbox/x_2.png".into(),
                    "assets/textures/common/touch/xbox/x_3.png".into(),
                ]),
                label: None,
                anchor: TouchAnchor::BottomRight,
                offset_x: 204.0,
                offset_y: 112.0,
                width: 128.0,
                height: 128.0,
            },
            TouchButtonDef {
                action: "".into(),
                texture: None,
                pressed_texture: None,
                frames: Some(vec![
                    "assets/textures/common/touch/xbox/y_0.png".into(),
                    "assets/textures/common/touch/xbox/y_1.png".into(),
                    "assets/textures/common/touch/xbox/y_2.png".into(),
                    "assets/textures/common/touch/xbox/y_3.png".into(),
                ]),
                label: None,
                anchor: TouchAnchor::BottomRight,
                offset_x: 112.0,
                offset_y: 204.0,
                width: 128.0,
                height: 128.0,
            },
        ],
    }
}
