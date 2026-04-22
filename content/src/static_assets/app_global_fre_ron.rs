//! Bootstrapped code asset for `app/global.fre.ron`.
//!
//! `app/global.fre.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::fre::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 发射当前 bootstrap 资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_ron("app/global.fre.ron", &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资产的类型化值。
pub fn asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::Global,
        enums: vec![].into_iter().collect(),
        facts: vec![
            ("player:def".into(), FactValueDef::Int(0)),
            ("player:atk".into(), FactValueDef::Int(0)),
            ("player:gold".into(), FactValueDef::Int(42)),
            ("player:lv".into(), FactValueDef::Int(1)),
            ("player:hp_max".into(), FactValueDef::Int(20)),
            ("player:exp".into(), FactValueDef::Int(0)),
            ("player:next_exp".into(), FactValueDef::Int(10)),
            ("player:name".into(), FactValueDef::String("Chara".into())),
            ("player:hp".into(), FactValueDef::Int(20)),
            ("player:weapon".into(), FactValueDef::String("stick".into())),
            (
                "player:armor".into(),
                FactValueDef::String("bandage".into()),
            ),
            (
                "player:inventory".into(),
                FactValueDef::StringList(vec![
                    "monster_candy".into(),
                    "monster_candy".into(),
                    "monster_candy".into(),
                    "monster_candy".into(),
                    "monster_candy".into(),
                    "UNDEFITEM".into(),
                ]),
            ),
            ("player:inventory_capacity".into(), FactValueDef::Int(8)),
        ]
        .into_iter()
        .collect(),
        rules: vec![],
    }
}
