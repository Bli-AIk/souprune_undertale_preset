//! View asset for `overworld/view/dialogue.view.ron`.
//!
//! `overworld/view/dialogue.view.ron` 的 view 资源。

use anyhow::Result;
use souprune_schema::view::*;
use souprune_vessel::prelude::*;

/// Emit this asset.
///
/// 生成当前资源。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资源的类型化值。
pub fn asset() -> ViewLayoutAsset {
    ViewLayout {
        roots: Vec::from([ViewNodeDef {
            name: "DialogueBox".into(),
            visible_when: Some("$dialogue:active == true".into()),
            texts: Vec::from([TextDef {
                id: "DialogueText".into(),
                font: "DTM-Mono".into(),
                content: Some("{{dialogue:main:text}}".into()),
                world_scale: vector2(13.25, 13.25),
                transform: SerializableTransform {
                    translation: Some(vector3(-130.5, 27.85, 1.0)),
                    ..Default::default()
                },
                line_height: Some(1.125),
                char_spacing: Some(1.0),
                word_spacing: Some(-7.5),
                ..Default::default()
            }]),
            view_box: Some(ViewBoxLogicDef {
                width: 283.0,
                height: 70.0,
                border_width: 3.0,
                offset: vector3(0.5, -78.0, 0.0),
                structure_file: Some("view/structures/view_box.sdf.ron".into()),
                ..Default::default()
            }),
            ..Default::default()
        }]),
        requires: Vec::from([DataRequirement::File("narrative/dialogue.fre.ron".into())]),
        facts: Some(
            Vec::from([("depth".into(), InitialFactValue::Int(0))])
                .into_iter()
                .collect(),
        ),
        ..Default::default()
    }
}
