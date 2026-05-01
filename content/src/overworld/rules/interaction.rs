//! Code representation of `overworld/rules/interaction.fre.ron`.
//!
//! `overworld/rules/interaction.fre.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::fre::*;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::Local,
        enums: vec![].into_iter().collect(),
        facts: vec![].into_iter().collect(),
        rules: vec![
            RuleDef {
                id: "overworld_open_backpack".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Menu".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$state:sequence_sub_state == 'Normal'".into(),
                    "$dialogue:active != true".into(),
                ],
                actions: vec![
                    RuleActionDef::Log {
                        message: "Opening Backpack".into(),
                    },
                    RuleActionDef::Custom {
                        action_type: "SetSubState".into(),
                        params: vec![("state".into(), "Backpack".into())]
                            .into_iter()
                            .collect(),
                    },
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "overworld_dialogue_started".into(),
                event: RuleEventDef::Event("dialogue:started".into()),
                conditions: vec![],
                actions: vec![RuleActionDef::Custom {
                    action_type: "SetSubState".into(),
                    params: vec![("state".into(), "Dialogue".into())]
                        .into_iter()
                        .collect(),
                }],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 100,
                consume_event: true,
            },
            RuleDef {
                id: "overworld_dialogue_ended".into(),
                event: RuleEventDef::Event("dialogue:ended".into()),
                conditions: vec![],
                actions: vec![
                    RuleActionDef::Custom {
                        action_type: "SetSubState".into(),
                        params: vec![("state".into(), "Normal".into())]
                            .into_iter()
                            .collect(),
                    },
                    RuleActionDef::Custom {
                        action_type: "DespawnView".into(),
                        params: vec![].into_iter().collect(),
                    },
                ],
                modifications: vec![FreFactModificationDef::Set {
                    key: "dialogue:has_focus".into(),
                    value: FactValueDef::Bool(false),
                }],
                outputs: vec![],
                enabled: true,
                priority: 100,
                consume_event: true,
            },
        ],
    }
}
