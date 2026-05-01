//! Code representation of `battle/rules/dialogue_stop.fre.ron`.
//!
//! `battle/rules/dialogue_stop.fre.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::fre::*;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::View,
        enums: vec![].into_iter().collect(),
        facts: vec![].into_iter().collect(),
        rules: vec![RuleDef {
            id: "".into(),
            event: RuleEventDef::Event("dialogue:stop".into()),
            conditions: vec![],
            actions: vec![],
            modifications: vec![],
            outputs: vec!["dialogue:battle_narration:stop".into()],
            enabled: true,
            priority: 0,
            consume_event: true,
        }],
    }
}
