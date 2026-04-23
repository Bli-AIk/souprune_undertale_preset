//! Bootstrapped code asset for `overworld/view/dialogue.view.ron`.
//!
//! `overworld/view/dialogue.view.ron` 的 bootstrap 代码资源。

use anyhow::Result;
use souprune_schema::val::*;
use souprune_schema::view::*;
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
pub fn asset() -> ViewLayoutAsset {
    ViewLayout {
        roots: vec![ViewNodeDef {
            name: "DialogueBox".into(),
            tags: vec![],
            style: StyleDef {
                width: None,
                height: None,
                left: None,
                right: None,
                top: None,
                bottom: None,
                position_type: None,
                flex_direction: None,
                justify_content: None,
                align_items: None,
            },
            visible_when: Some("$dialogue_active".into()),
            background_color: None,
            border_color: None,
            image: None,
            sprite: None,
            state_sprite: None,
            texts: vec![TextDef {
                id: "DialogueText".into(),
                content: Some("{{dialogue_text}}".into()),
                font: "DTM-Mono".into(),
                align: None,
                anchor: None,
                world_scale: (Val::Static(13.25), Val::Static(13.25)),
                color: (
                    Val::Static(1.0),
                    Val::Static(1.0),
                    Val::Static(1.0),
                    Val::Static(1.0),
                ),
                transform: SerializableTransform {
                    translation: Some((Val::Static(-130.5), Val::Static(27.85), Val::Static(1.0))),
                    rotation: None,
                    scale: None,
                },
                line_height: Some(1.125),
                char_spacing: Some(1.0),
                word_spacing: Some(-7.5),
                conditional_style: None,
                visible_when: None,
            }],
            view_box: Some(ViewBoxLogicDef {
                width: 283.0,
                height: 70.0,
                border_width: 3.0,
                offset: (Val::Static(0.5), Val::Static(-78.0), Val::Static(0.0)),
                fill_shader: None,
                structure_file: Some("view/structures/view_box.sdf.ron".into()),
                fill_color: None,
            }),
            children: vec![],
            repeat: None,
        }],
        requires: vec![DataRequirement::File("narrative/dialogue.fre.ron".into())],
        facts: Some(
            vec![
                ("dialogue_text".into(), InitialFactValue::String("".into())),
                ("dialogue_active".into(), InitialFactValue::Bool(true)),
                ("depth".into(), InitialFactValue::Int(0)),
            ]
            .into_iter()
            .collect(),
        ),
        world_space: false,
        coordinate_system: CoordinateSystem::Standard,
    }
}
