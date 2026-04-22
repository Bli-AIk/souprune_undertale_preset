//! Bootstrapped code asset for `battle/common/show_narration.sequence.ron`.
//!
//! `battle/common/show_narration.sequence.ron` 的 bootstrap 代码资产。

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
                key: "narration_visible".into(),
                value: FactValueMatch::Bool(true),
            },
            Chapter::AwaitFact {
                condition: "$confirm_pressed == true".into(),
                local: true,
            },
            Chapter::SetViewFact {
                key: "confirm_pressed".into(),
                value: FactValueMatch::Bool(false),
            },
        ],
    }
}
