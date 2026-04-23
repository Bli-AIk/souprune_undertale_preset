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
    view_layout(vec![view_node("DialogueBox")
        .visible_when("$dialogue_active")
        .texts(vec![view_text(
            "DialogueText",
            "{{dialogue_text}}",
            "DTM-Mono",
        )
        .world_scale(vector2(13.25, 13.25))
        .translation(vector3(-130.5, 27.85, 1.0))
        .line_height(1.125)
        .character_spacing(1.0)
        .word_spacing(-7.5)])
        .view_box(
            view_box(283.0, 70.0)
                .border_width(3.0)
                .offset(vector3(0.5, -78.0, 0.0))
                .structure_file("view/structures/view_box.sdf.ron"),
        )])
    .require_file("narrative/dialogue.fre.ron")
    .initial_facts(vec![
        ("depth", InitialFactValue::Int(0)),
        ("dialogue_active", InitialFactValue::Bool(true)),
        ("dialogue_text", InitialFactValue::String("".into())),
    ])
}
