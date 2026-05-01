//! Code representation of `battle/rules/dialogue_test.fre.ron`.
//!
//! `battle/rules/dialogue_test.fre.ron` 的代码表示。

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
        facts: vec![("battle:intro_shown".into(), FactValueDef::Bool(false))]
            .into_iter()
            .collect(),
        rules: vec![
            RuleDef {
                id: "battle_encounter_intro_fallback".into(),
                event: RuleEventDef::Event("battle:show_intro".into()),
                conditions: vec!["$battle:intro_shown == false".into()],
                actions: vec![RuleActionDef::Log {
                    message: "Battle: using preset fallback intro handler".into(),
                }],
                modifications: vec![FreFactModificationDef::Set {
                    key: "battle:intro_shown".into(),
                    value: FactValueDef::Bool(true),
                }],
                outputs: vec![],
                enabled: true,
                priority: 100,
                consume_event: true,
            },
            RuleDef {
                id: "battle_dialogue_started".into(),
                event: RuleEventDef::Event("dialogue:started".into()),
                conditions: vec![],
                actions: vec![],
                modifications: vec![FreFactModificationDef::Set {
                    key: "battle:paused_for_dialogue".into(),
                    value: FactValueDef::Bool(true),
                }],
                outputs: vec![],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
            RuleDef {
                id: "battle_dialogue_ended".into(),
                event: RuleEventDef::Event("dialogue:ended".into()),
                conditions: vec![],
                actions: vec![],
                modifications: vec![
                    FreFactModificationDef::Set {
                        key: "dialogue:has_focus".into(),
                        value: FactValueDef::Bool(false),
                    },
                    FreFactModificationDef::Set {
                        key: "battle:paused_for_dialogue".into(),
                        value: FactValueDef::Bool(false),
                    },
                    FreFactModificationDef::Remove("dialogue:pending_mortar_path".into()),
                    FreFactModificationDef::Remove("dialogue:pending_mortar_node".into()),
                    FreFactModificationDef::Remove("dialogue:pending_view".into()),
                ],
                outputs: vec![],
                enabled: true,
                priority: 5,
                consume_event: true,
            },
        ],
    }
}
