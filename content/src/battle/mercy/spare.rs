//! Code representation of `battle/mercy/spare.sequence.ron`.
//!
//! `battle/mercy/spare.sequence.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

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
