//! Bootstrapped code asset for `overworld/rules/backpack.fre.ron`.
//!
//! `overworld/rules/backpack.fre.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::fre::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 发射当前 bootstrap 资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_ron("overworld/rules/backpack.fre.ron", &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资产的类型化值。
pub fn asset() -> FreAsset {
    FreAsset {
        scope: RuleScopeDef::View,
        enums: vec![(
            "depth".into(),
            vec![
                "menu".into(),
                "item_list".into(),
                "item_options".into(),
                "status".into(),
            ],
        )]
        .into_iter()
        .collect(),
        facts: vec![
            ("depth".into(), FactValueDef::Enum("menu".into())),
            ("selection".into(), FactValueDef::Int(0)),
            ("item_list_selection".into(), FactValueDef::Int(0)),
            ("interactable".into(), FactValueDef::Bool(true)),
        ]
        .into_iter()
        .collect(),
        rules: vec![
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Up".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec!["$selection > 0".into()],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "selection".into(),
                        LocalFactValue::Expr("$selection - 1".into()),
                    ),
                    RuleActionDef::PlaySound("choice".into()),
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
                    action: "Down".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec!["$depth == 'menu'".into(), "$selection < 1".into()],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "selection".into(),
                        LocalFactValue::Expr("$selection + 1".into()),
                    ),
                    RuleActionDef::PlaySound("choice".into()),
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
                    action: "Down".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$depth == 'item_list'".into(),
                    "$selection < $player:inventory.len() - 1".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "selection".into(),
                        LocalFactValue::Expr("$selection + 1".into()),
                    ),
                    RuleActionDef::PlaySound("choice".into()),
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
                    action: "Left".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec!["$depth == 'item_options'".into(), "$selection > 0".into()],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "selection".into(),
                        LocalFactValue::Expr("$selection - 1".into()),
                    ),
                    RuleActionDef::PlaySound("choice".into()),
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
                    action: "Right".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec!["$depth == 'item_options'".into(), "$selection < 2".into()],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "selection".into(),
                        LocalFactValue::Expr("$selection + 1".into()),
                    ),
                    RuleActionDef::PlaySound("choice".into()),
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
                conditions: vec!["$depth == 'menu'".into(), "$selection == 0".into()],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "depth".into(),
                        LocalFactValue::Enum("item_list".into()),
                    ),
                    RuleActionDef::SetLocalFact("selection".into(), LocalFactValue::Int(0)),
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
                conditions: vec!["$depth == 'menu'".into(), "$selection == 1".into()],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "depth".into(),
                        LocalFactValue::Enum("status".into()),
                    ),
                    RuleActionDef::SetLocalFact("selection".into(), LocalFactValue::Int(0)),
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
                conditions: vec!["$depth == 'item_list'".into()],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "item_list_selection".into(),
                        LocalFactValue::Expr("$selection".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "depth".into(),
                        LocalFactValue::Enum("item_options".into()),
                    ),
                    RuleActionDef::SetLocalFact("selection".into(), LocalFactValue::Int(0)),
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
                conditions: vec!["$depth == 'item_options'".into(), "$selection == 0".into()],
                actions: vec![
                    RuleActionDef::Custom {
                        action_type: "UseItem".into(),
                        params: vec![
                            ("start_dialogue".into(), "true".into()),
                            ("index_expr".into(), "$item_list_selection".into()),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    RuleActionDef::SwitchState("Normal".into()),
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
                conditions: vec!["$depth == 'item_options'".into(), "$selection == 1".into()],
                actions: vec![
                    RuleActionDef::Custom {
                        action_type: "CheckItem".into(),
                        params: vec![("index_expr".into(), "$item_list_selection".into())]
                            .into_iter()
                            .collect(),
                    },
                    RuleActionDef::SwitchState("Normal".into()),
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
                conditions: vec!["$depth == 'item_options'".into(), "$selection == 2".into()],
                actions: vec![
                    RuleActionDef::Custom {
                        action_type: "DropItem".into(),
                        params: vec![("index_expr".into(), "$item_list_selection".into())]
                            .into_iter()
                            .collect(),
                    },
                    RuleActionDef::SwitchState("Normal".into()),
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
                    action: "Cancel".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec!["$depth == 'menu'".into()],
                actions: vec![RuleActionDef::SwitchState("Normal".into())],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 100,
                consume_event: true,
            },
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Menu".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec!["$depth == 'menu'".into()],
                actions: vec![RuleActionDef::SwitchState("Normal".into())],
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
                conditions: vec!["$depth == 'item_list'".into()],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "depth".into(),
                        LocalFactValue::Enum("menu".into()),
                    ),
                    RuleActionDef::SetLocalFact("selection".into(), LocalFactValue::Int(0)),
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
                    action: "Cancel".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec!["$depth == 'item_options'".into()],
                actions: vec![RuleActionDef::SetLocalFact(
                    "depth".into(),
                    LocalFactValue::Enum("item_list".into()),
                )],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Cancel".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec!["$depth == 'status'".into()],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "depth".into(),
                        LocalFactValue::Enum("menu".into()),
                    ),
                    RuleActionDef::SetLocalFact("selection".into(), LocalFactValue::Int(1)),
                ],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 0,
                consume_event: true,
            },
        ],
    }
}
