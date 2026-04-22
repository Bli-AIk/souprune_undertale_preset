//! Bootstrapped code asset for `battle/common/player_turn.sequence.ron`.
//!
//! `battle/common/player_turn.sequence.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::sequence::*;
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
pub fn asset() -> SequenceAsset {
    SequenceAsset {
        mode: None,
        rules_file: None,
        exits: vec![].into_iter().collect(),
        chapters: vec![
            Chapter::SetViewFact {
                key: "buttons_visible".into(),
                value: FactValueMatch::Bool(true),
            },
            Chapter::SetViewFact {
                key: "interactable".into(),
                value: FactValueMatch::Bool(true),
            },
            Chapter::SetViewFact {
                key: "depth".into(),
                value: FactValueMatch::Int(0),
            },
            Chapter::SetViewFact {
                key: "button_selection".into(),
                value: FactValueMatch::Int(0),
            },
            Chapter::AwaitFact {
                condition: "$selection_confirmed == true".into(),
                local: true,
            },
            Chapter::SetViewFact {
                key: "selection_confirmed".into(),
                value: FactValueMatch::Bool(false),
            },
            Chapter::RunSequence {
                path: None,
                path_fact: Some("pending_sequence".into()),
                params: vec![].into_iter().collect(),
            },
            Chapter::RunSequence {
                path: Some("battle/common/end_view_interaction.sequence.ron".into()),
                path_fact: None,
                params: vec![].into_iter().collect(),
            },
        ],
    }
}
