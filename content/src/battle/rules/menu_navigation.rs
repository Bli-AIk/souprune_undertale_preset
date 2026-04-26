//! Code representation of `battle/rules/menu_navigation.fre.ron`.
//!
//! `battle/rules/menu_navigation.fre.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::fre::*;
use souprune_vessel::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
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
        enums: vec![
            (
                "depth".into(),
                vec!["main".into(), "submenu".into(), "options".into()],
            ),
            (
                "menu_context".into(),
                vec!["fight".into(), "act".into(), "item".into(), "mercy".into()],
            ),
        ]
        .into_iter()
        .collect(),
        facts: vec![].into_iter().collect(),
        rules: vec![
            RuleDef {
                id: "".into(),
                event: RuleEventDef::ActionEvent {
                    action: "Left".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'main'".into(),
                    "$button_selection > 0".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "button_selection".into(),
                        fact_shift("button_selection", -1),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'main'".into(),
                    "$button_selection == 0".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact("button_selection".into(), LocalFactValue::Int(3)),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'main'".into(),
                    "$button_selection < 3".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "button_selection".into(),
                        fact_shift("button_selection", 1),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'main'".into(),
                    "$button_selection == 3".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact("button_selection".into(), LocalFactValue::Int(0)),
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
                    action: "Up".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context != 'item'".into(),
                    "$enemy_selection > 0".into(),
                    "$enemy_selection > $enemy_view_offset".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "enemy_selection".into(),
                        fact_shift("enemy_selection", -1),
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
                    action: "Up".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context != 'item'".into(),
                    "$enemy_selection > 0".into(),
                    "$enemy_selection <= $enemy_view_offset".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "enemy_selection".into(),
                        fact_shift("enemy_selection", -1),
                    ),
                    RuleActionDef::SetLocalFact(
                        "enemy_view_offset".into(),
                        fact_shift("enemy_view_offset", -1),
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
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context != 'item'".into(),
                    "$enemy_selection < $enemy_names - 1".into(),
                    "$enemy_selection < $enemy_view_offset + $enemy_display_limit - 1".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "enemy_selection".into(),
                        fact_shift("enemy_selection", 1),
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
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context != 'item'".into(),
                    "$enemy_selection < $enemy_names - 1".into(),
                    "$enemy_selection >= $enemy_view_offset + $enemy_display_limit - 1".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "enemy_selection".into(),
                        fact_shift("enemy_selection", 1),
                    ),
                    RuleActionDef::SetLocalFact(
                        "enemy_view_offset".into(),
                        fact_shift("enemy_view_offset", 1),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'item'".into(),
                    "$item_selection % 2 == 1".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "item_selection".into(),
                        fact_shift("item_selection", -1),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'item'".into(),
                    "$item_selection % 2 == 0".into(),
                    "$item_selection >= 4".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "item_selection".into(),
                        fact_shift("item_selection", -3),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'item'".into(),
                    "$item_selection % 2 == 0".into(),
                    "$item_selection < 4".into(),
                    "$item_selection + 5 < $item_count".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "item_selection".into(),
                        fact_shift("item_selection", 5),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'item'".into(),
                    "$item_selection % 2 == 0".into(),
                    "$item_selection < 4".into(),
                    "$item_selection + 5 >= $item_count".into(),
                    "$item_selection + 1 < $item_count".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "item_selection".into(),
                        fact_shift("item_selection", 1),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'item'".into(),
                    "$item_selection % 2 == 0".into(),
                    "$item_selection + 1 < $item_count".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "item_selection".into(),
                        fact_shift("item_selection", 1),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'item'".into(),
                    "$item_selection % 2 == 1".into(),
                    "$item_selection < 4".into(),
                    "$item_selection + 3 < $item_count".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "item_selection".into(),
                        fact_shift("item_selection", 3),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'item'".into(),
                    "$item_selection % 2 == 1".into(),
                    "$item_selection >= 4".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "item_selection".into(),
                        fact_shift("item_selection", -5),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'item'".into(),
                    "$item_selection % 2 == 1".into(),
                    "$item_selection < 4".into(),
                    "$item_selection + 3 >= $item_count".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "item_selection".into(),
                        fact_shift("item_selection", -1),
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
                    action: "Up".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'item'".into(),
                    "$item_selection % 4 >= 2".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "item_selection".into(),
                        fact_shift("item_selection", -2),
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
                    "$interactable == true".into(),
                    "$depth == 'submenu'".into(),
                    "$menu_context == 'item'".into(),
                    "$item_selection % 4 < 2".into(),
                    "$item_selection + 2 < $item_count".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "item_selection".into(),
                        fact_shift("item_selection", 2),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'options'".into(),
                    "$menu_context == 'act'".into(),
                    "$act_selection % 2 == 1".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "act_selection".into(),
                        fact_shift("act_selection", -1),
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
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'options'".into(),
                    "$menu_context == 'act'".into(),
                    "$act_selection % 2 == 0".into(),
                    "$act_selection + 1 < $act_count".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "act_selection".into(),
                        fact_shift("act_selection", 1),
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
                    action: "Up".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'options'".into(),
                    "$menu_context == 'act'".into(),
                    "$act_selection >= 2".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "act_selection".into(),
                        fact_shift("act_selection", -2),
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
                    "$interactable == true".into(),
                    "$depth == 'options'".into(),
                    "$menu_context == 'act'".into(),
                    "$act_selection + 2 < $act_count".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "act_selection".into(),
                        fact_shift("act_selection", 2),
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
                    action: "Up".into(),
                    kind: ActionEventKind::JustPressed,
                },
                conditions: vec![
                    "$interactable == true".into(),
                    "$depth == 'options'".into(),
                    "$menu_context == 'mercy'".into(),
                    "$mercy_selection > 0".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "mercy_selection".into(),
                        fact_shift("mercy_selection", -1),
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
                    "$interactable == true".into(),
                    "$depth == 'options'".into(),
                    "$menu_context == 'mercy'".into(),
                    "$mercy_selection < $mercy_count - 1".into(),
                ],
                actions: vec![
                    RuleActionDef::SetLocalFact(
                        "mercy_selection".into(),
                        fact_shift("mercy_selection", 1),
                    ),
                    RuleActionDef::PlaySound("choice".into()),
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
