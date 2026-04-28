//! Code representation of `overworld/rules/backpack.fre.ron`.
//!
//! `overworld/rules/backpack.fre.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::fre::*;
use souprune_vessel::prelude::*;

const SCREEN_FACTS_UPDATED_EVENT: &str = "overworld:screen_facts_updated";
const PLAYER_SCREEN_Y_FACT: &str = "$overworld:player_screen_y";
const INFO_BOX_Y_OFFSET_FACT: &str = "info_box_y_offset";
const INFO_BOX_MOVE_THRESHOLD_Y: f64 = 130.0;
const INFO_BOX_MOVE_OFFSET_Y: i64 = 135;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

fn fact_value(name: &str) -> LocalFactValue {
    expr::fact(name).into()
}

fn fact_shift(name: &str, amount: i64) -> LocalFactValue {
    let value = if amount < 0 {
        expr::fact(name) - amount.abs()
    } else {
        expr::fact(name) + amount
    };
    value.into()
}

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
            (
                INFO_BOX_Y_OFFSET_FACT.into(),
                FactValueDef::Int(0),
            ),
            ("selection".into(), FactValueDef::Int(0)),
            ("item_list_selection".into(), FactValueDef::Int(0)),
            ("interactable".into(), FactValueDef::Bool(true)),
        ]
        .into_iter()
        .collect(),
        rules: vec![
            RuleDef {
                id: "move_info_box_down_when_player_is_low".into(),
                event: RuleEventDef::Event(SCREEN_FACTS_UPDATED_EVENT.into()),
                conditions: vec![
                    format!("{PLAYER_SCREEN_Y_FACT} > {INFO_BOX_MOVE_THRESHOLD_Y}"),
                    format!("${INFO_BOX_Y_OFFSET_FACT} != {INFO_BOX_MOVE_OFFSET_Y}"),
                ],
                actions: vec![RuleActionDef::SetLocalFact(
                    INFO_BOX_Y_OFFSET_FACT.into(),
                    LocalFactValue::Int(INFO_BOX_MOVE_OFFSET_Y),
                )],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 100,
                consume_event: true,
            },
            RuleDef {
                id: "restore_info_box_when_player_is_high".into(),
                event: RuleEventDef::Event(SCREEN_FACTS_UPDATED_EVENT.into()),
                conditions: vec![
                    format!("{PLAYER_SCREEN_Y_FACT} <= {INFO_BOX_MOVE_THRESHOLD_Y}"),
                    format!("${INFO_BOX_Y_OFFSET_FACT} != 0"),
                ],
                actions: vec![RuleActionDef::SetLocalFact(
                    INFO_BOX_Y_OFFSET_FACT.into(),
                    LocalFactValue::Int(0),
                )],
                modifications: vec![],
                outputs: vec![],
                enabled: true,
                priority: 100,
                consume_event: true,
            },
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
                        fact_shift("selection", -1),
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
                        fact_shift("selection", 1),
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
                        fact_shift("selection", 1),
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
                        fact_shift("selection", -1),
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
                        fact_shift("selection", 1),
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
                        fact_value("selection"),
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
                            (
                                "index_expr".into(),
                                expr::fact("item_list_selection").into(),
                            ),
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
                        params: vec![(
                            "index_expr".into(),
                            expr::fact("item_list_selection").into(),
                        )]
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
                        params: vec![(
                            "index_expr".into(),
                            expr::fact("item_list_selection").into(),
                        )]
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
