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

fn fact_value(name: &str) -> LocalFactValue {
    LocalFactValue::Expr(expr::fact(name).into_string())
}

fn fact_element(name: &str, index: expr::Expression) -> LocalFactValue {
    LocalFactValue::Expr(expr::fact_at(name, index).into_string())
}

fn current_enemy_field(field: &str) -> LocalFactValue {
    LocalFactValue::Expr(expr::dynamic_fact("current_enemy_id", field).into_string())
}

fn current_enemy_field_at(field: &str, index: expr::Expression) -> LocalFactValue {
    LocalFactValue::Expr(expr::dynamic_fact_at("current_enemy_id", field, index).into_string())
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
                        fact_value("button_selection"),
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
                        fact_element("enemy_ids", expr::fact("enemy_selection")),
                    ),
                    RuleActionDef::SetLocalFact("act_count".into(), current_enemy_field("act_count")),
                    RuleActionDef::SetLocalFact(
                        "action_labels".into(),
                        current_enemy_field("action_labels"),
                    ),
                    RuleActionDef::SetLocalFact(
                        "action_sequences".into(),
                        current_enemy_field("action_sequences"),
                    ),
                    RuleActionDef::SetLocalFact(
                        "action_params".into(),
                        current_enemy_field("action_params"),
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
                        fact_element("enemy_ids", expr::fact("enemy_selection")),
                    ),
                    RuleActionDef::SetLocalFact("mercy_count".into(), current_enemy_field("mercy_count")),
                    RuleActionDef::SetLocalFact(
                        "mercy_labels".into(),
                        current_enemy_field("mercy_labels"),
                    ),
                    RuleActionDef::SetLocalFact(
                        "mercy_sequences".into(),
                        current_enemy_field("mercy_sequences"),
                    ),
                    RuleActionDef::SetLocalFact(
                        "mercy_params".into(),
                        current_enemy_field("mercy_params"),
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
                        params: vec![("index_expr".into(), expr::fact("item_selection").into_string())]
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
                        current_enemy_field_at("action_params", expr::fact("act_selection")),
                    ),
                    RuleActionDef::SetLocalFact(
                        "mortar_path".into(),
                        current_enemy_field("mortar_path"),
                    ),
                    RuleActionDef::SetLocalFact(
                        "pending_sequence".into(),
                        current_enemy_field_at("action_sequences", expr::fact("act_selection")),
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
                        fact_element("mercy_params", expr::fact("mercy_selection")),
                    ),
                    RuleActionDef::SetLocalFact(
                        "mortar_path".into(),
                        current_enemy_field("mortar_path"),
                    ),
                    RuleActionDef::SetLocalFact(
                        "pending_sequence".into(),
                        fact_element("mercy_sequences", expr::fact("mercy_selection")),
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
