//! Code representation of `narrative/dialogue.fre.ron`.
//!
//! `narrative/dialogue.fre.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::fre::*;
use souprune_vessel::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto_with(
        file!(),
        &asset(),
        EmitPathConfig::new().output_path("narrative/dialogue.fre.ron"),
    )?;
    Ok(())
}

pub fn asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::Local,
        enums: vec![].into_iter().collect(),
        facts: vec![(
            "dialogue:typewriter_speed".into(),
            FactValueDef::Float(0.03),
        )]
        .into_iter()
        .collect(),
        rules: vec![
            RuleDef {
                id: "battle_narration_advance_on_confirm".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Confirm".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$dialogue:battle_narration:has_focus == true".into(),
                    "$dialogue:battle_narration:typewriter_playing == false".into(),
                ],
                actions: vec![],
                modifications: vec![],
                outputs: vec!["dialogue_advance".into()],
                enabled: true,
                priority: 10,
                consume_event: true,
            },
            RuleDef {
                id: "battle_narration_skip_on_cancel".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Cancel".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$depth == 0".into(),
                    "$dialogue:battle_narration:has_focus == true".into(),
                    "$dialogue:battle_narration:typewriter_playing == true".into(),
                ],
                actions: vec![],
                modifications: vec![],
                outputs: vec!["dialogue_skip_typewriter".into()],
                enabled: true,
                priority: 10,
                consume_event: true,
            },
            RuleDef {
                id: "battle_enemy_speech_advance_on_confirm".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Confirm".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$dialogue:battle_enemy_speech:has_focus == true".into(),
                    "$dialogue:battle_enemy_speech:typewriter_playing == false".into(),
                ],
                actions: vec![],
                modifications: vec![],
                outputs: vec!["dialogue_advance".into()],
                enabled: true,
                priority: 10,
                consume_event: true,
            },
            RuleDef {
                id: "battle_enemy_speech_skip_on_confirm".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Confirm".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$dialogue:battle_enemy_speech:has_focus == true".into(),
                    "$dialogue:battle_enemy_speech:typewriter_playing == true".into(),
                ],
                actions: vec![],
                modifications: vec![],
                outputs: vec!["dialogue_skip_typewriter".into()],
                enabled: true,
                priority: 10,
                consume_event: true,
            },
            RuleDef {
                id: "battle_enemy_speech_skip_on_cancel".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Cancel".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$depth == 0".into(),
                    "$dialogue:battle_enemy_speech:has_focus == true".into(),
                    "$dialogue:battle_enemy_speech:typewriter_playing == true".into(),
                ],
                actions: vec![],
                modifications: vec![],
                outputs: vec!["dialogue_skip_typewriter".into()],
                enabled: true,
                priority: 10,
                consume_event: true,
            },
        ],
    }
}
