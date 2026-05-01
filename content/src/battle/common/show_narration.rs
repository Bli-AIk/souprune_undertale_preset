//! Code representation of `battle/common/show_narration.sequence.ron`.
//!
//! `battle/common/show_narration.sequence.ron` 的代码表示。

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
