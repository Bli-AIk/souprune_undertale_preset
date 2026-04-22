//! Bootstrapped code asset for `battle/common/end_view_interaction.sequence.ron`.
//!
//! `battle/common/end_view_interaction.sequence.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 发射当前 bootstrap 资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_ron("battle/common/end_view_interaction.sequence.ron", &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资产的类型化值。
pub fn asset() -> SequenceAsset {
    SequenceAsset {
        mode: None,
        rules_file: None,
        exits: vec![].into_iter().collect(),
        chapters: vec![
            Chapter::SetViewFact {
                key: "interactable".into(),
                value: FactValueMatch::Bool(false),
            },
            Chapter::SetViewFact {
                key: "buttons_visible".into(),
                value: FactValueMatch::Bool(false),
            },
            Chapter::SetViewFact {
                key: "narration_visible".into(),
                value: FactValueMatch::Bool(false),
            },
            Chapter::SetViewFact {
                key: "dialogue_text".into(),
                value: FactValueMatch::String("".into()),
            },
            Chapter::SetViewFact {
                key: "dialogue_visible".into(),
                value: FactValueMatch::Bool(false),
            },
            Chapter::SetViewFact {
                key: "depth".into(),
                value: FactValueMatch::Int(-1),
            },
        ],
    }
}
