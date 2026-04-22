//! Bootstrapped code asset for `battle/rules/fight_hit.fre.ron`.
//!
//! `battle/rules/fight_hit.fre.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::fre::*;
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
pub fn asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::View,
        enums: vec![].into_iter().collect(),
        facts: vec![].into_iter().collect(),
        rules: vec![RuleDef {
            id: "".into(),
            event: RuleEventDef::Event("fight:hit".into()),
            conditions: vec![],
            actions: vec![RuleActionDef::PlaySound("slice".into())],
            modifications: vec![],
            outputs: vec![],
            enabled: true,
            priority: 0,
            consume_event: true,
        }],
    }
}
