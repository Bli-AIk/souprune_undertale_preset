//! Code representation of `battle/rules/menu_cancel.fre.ron`.
//!
//! `battle/rules/menu_cancel.fre.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::fre::*;
use souprune_vessel::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::View,
        enums: vec![(
            "depth".into(),
            vec!["main".into(), "submenu".into(), "options".into()],
        )]
        .into_iter()
        .collect(),
        facts: vec![].into_iter().collect(),
        rules: vec![
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Cancel".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec!["$interactable == true".into(), "$depth == 'options'".into()],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "depth".into(),
                        LocalFactValue::Enum("submenu".into()),
                    ),
                    RuleActionDef::SetLocalFact("act_selection".into(), LocalFactValue::Int(0)),
                    RuleActionDef::SetLocalFact("mercy_selection".into(), LocalFactValue::Int(0)),
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 100,
                consume_event: true,
            },
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Cancel".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec!["$interactable == true".into(), "$depth == 'submenu'".into()],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "depth".into(),
                        LocalFactValue::Enum("main".into()),
                    ),
                    RuleActionDef::SetLocalFact("enemy_selection".into(), LocalFactValue::Int(0)),
                    RuleActionDef::SetLocalFact("enemy_view_offset".into(), LocalFactValue::Int(0)),
                    RuleActionDef::SetLocalFact("item_selection".into(), LocalFactValue::Int(0)),
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 100,
                consume_event: true,
            },
        ],
    }
}
