//! Bootstrapped code asset for `battle/mercy/end.sequence.ron`.
//!
//! `battle/mercy/end.sequence.ron` 的 bootstrap 代码资源。

use anyhow::Result;
use souprune_schema::sequence::*;
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
pub fn asset() -> SequenceAsset {
    SequenceAsset {
        mode: None,
        rules_file: None,
        exits: vec![].into_iter().collect(),
        chapters: vec![Chapter::RunSequence {
            path: Some("battle/common/end_view_interaction.sequence.ron".into()),
            path_fact: None,
            params: vec![].into_iter().collect(),
        }],
    }
}
