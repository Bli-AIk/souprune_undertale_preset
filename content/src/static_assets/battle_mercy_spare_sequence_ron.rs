//! Bootstrapped code asset for `battle/mercy/spare.sequence.ron`.
//!
//! `battle/mercy/spare.sequence.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 发射当前 bootstrap 资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_ron("battle/mercy/spare.sequence.ron", &asset())?;
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
            Chapter::RunSequence {
                path: Some("battle/common/narration.sequence.ron".into()),
                path_fact: None,
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
