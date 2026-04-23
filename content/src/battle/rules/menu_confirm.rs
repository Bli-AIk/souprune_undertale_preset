//! Code representation of `battle/rules/menu_confirm.fre.ron`.
//!
//! `battle/rules/menu_confirm.fre.ron` 的代码表示。

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
        enums: vec![
            (
                "menu_context".into(),
                vec!["fight".into(), "act".into(), "item".into(), "mercy".into()],
            ),
            (
                "depth".into(),
                vec!["main".into(), "submenu".into(), "options".into()],
            ),
        ]
        .into_iter()
        .collect(),
        facts: vec![].into_iter().collect(),
        rules: vec![
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Confirm".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec!["$interactable == true".into(), "$depth == 'main'".into()],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "menu_context".into(),
                        LocalFactValue::Expr("$button_selection".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "depth".into(),
                        LocalFactValue::Enum("submenu".into()),
                    ),
                    RuleActionDef::SetLocalFact("enemy_selection".into(), LocalFactValue::Int(0)),
                    RuleActionDef::SetLocalFact("item_selection".into(), LocalFactValue::Int(0)),
                    RuleActionDef::PlaySound("confirm".into()),
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Confirm".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'act'".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "current_enemy_id".into(),
                        LocalFactValue::Expr("$enemy_ids[$enemy_selection]".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "act_count".into(),
                        LocalFactValue::Expr("$${current_enemy_id}.act_count".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "action_labels".into(),
                        LocalFactValue::Expr("$${current_enemy_id}.action_labels".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "action_sequences".into(),
                        LocalFactValue::Expr("$${current_enemy_id}.action_sequences".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "action_params".into(),
                        LocalFactValue::Expr("$${current_enemy_id}.action_params".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "depth".into(),
                        LocalFactValue::Enum("options".into()),
                    ),
                    RuleActionDef::SetLocalFact("act_selection".into(), LocalFactValue::Int(0)),
                    RuleActionDef::PlaySound("confirm".into()),
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Confirm".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'mercy'".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "current_enemy_id".into(),
                        LocalFactValue::Expr("$enemy_ids[$enemy_selection]".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "mercy_count".into(),
                        LocalFactValue::Expr("$${current_enemy_id}.mercy_count".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "mercy_labels".into(),
                        LocalFactValue::Expr("$${current_enemy_id}.mercy_labels".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "mercy_sequences".into(),
                        LocalFactValue::Expr("$${current_enemy_id}.mercy_sequences".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "mercy_params".into(),
                        LocalFactValue::Expr("$${current_enemy_id}.mercy_params".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "depth".into(),
                        LocalFactValue::Enum("options".into()),
                    ),
                    RuleActionDef::SetLocalFact("mercy_selection".into(), LocalFactValue::Int(0)),
                    RuleActionDef::PlaySound("confirm".into()),
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Confirm".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'fight'".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "selection_confirmed".into(),
                        LocalFactValue::Bool(true),
                    ),
                    RuleActionDef::SetLocalFact("interactable".into(), LocalFactValue::Bool(false)),
                    RuleActionDef::SetLocalFact(
                        "pending_sequence".into(),
                        LocalFactValue::String("battle/common/fight_target.sequence.ron".into()),
                    ),
                    RuleActionDef::PlaySound("confirm".into()),
                    RuleActionDef::EmitEvent("battle_action_executed".into()),
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Confirm".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'item'".into(),
                ],
                actions: vec![
                    RuleActionDef::Custom {
                        action_type: "UseItem".into(),
                        params: vec![("index_expr".into(), "$item_selection".into())]
                            .into_iter()
                            .collect(),
                    },
                    RuleActionDef::SetLocalFact(
                        "pending_sequence".into(),
                        LocalFactValue::String("battle/common/narration.sequence.ron".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "selection_confirmed".into(),
                        LocalFactValue::Bool(true),
                    ),
                    RuleActionDef::SetLocalFact("interactable".into(), LocalFactValue::Bool(false)),
                    RuleActionDef::PlaySound("confirm".into()),
                    RuleActionDef::EmitEvent("battle_action_executed".into()),
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Confirm".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'options'".into(),
                    "$menu_context == 'act'".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "action_param".into(),
                        LocalFactValue::Expr(
                            "$${current_enemy_id}.action_params[$act_selection]".into(),
                        ),
                    ),
                    RuleActionDef::SetLocalFact(
                        "mortar_path".into(),
                        LocalFactValue::Expr("$${current_enemy_id}.mortar_path".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "pending_sequence".into(),
                        LocalFactValue::Expr(
                            "$${current_enemy_id}.action_sequences[$act_selection]".into(),
                        ),
                    ),
                    RuleActionDef::SetLocalFact(
                        "selection_confirmed".into(),
                        LocalFactValue::Bool(true),
                    ),
                    RuleActionDef::SetLocalFact("interactable".into(), LocalFactValue::Bool(false)),
                    RuleActionDef::PlaySound("confirm".into()),
                    RuleActionDef::EmitEvent("battle_action_executed".into()),
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Confirm".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'options'".into(),
                    "$menu_context == 'mercy'".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "action_param".into(),
                        LocalFactValue::Expr("$mercy_params[$mercy_selection]".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "mortar_path".into(),
                        LocalFactValue::Expr("$${current_enemy_id}.mortar_path".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "pending_sequence".into(),
                        LocalFactValue::Expr("$mercy_sequences[$mercy_selection]".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "selection_confirmed".into(),
                        LocalFactValue::Bool(true),
                    ),
                    RuleActionDef::SetLocalFact("interactable".into(), LocalFactValue::Bool(false)),
                    RuleActionDef::PlaySound("confirm".into()),
                    RuleActionDef::EmitEvent("battle_action_executed".into()),
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Confirm".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$narration_visible == true".into(),
                    "$interactable == false".into(),
                ],
                actions: vec![RuleActionDef::SetLocalFact(
                    "confirm_pressed".into(),
                    LocalFactValue::Bool(true),
                )],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
        ],
    }
}
