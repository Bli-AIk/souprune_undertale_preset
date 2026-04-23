//! Bootstrapped code asset for `battle/rules/menu_navigation.fre.ron`.
//!
//! `battle/rules/menu_navigation.fre.ron` 的 bootstrap 代码资源。

use anyhow::Result;
use souprune_schema::fre::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 生成当前 bootstrap 资源。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资源的类型化值。
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
                        LocalFactValue::Expr("$button_selection - 1".into()),
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
                        LocalFactValue::Expr("$button_selection + 1".into()),
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
                        LocalFactValue::Expr("$enemy_selection - 1".into()),
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
                        LocalFactValue::Expr("$enemy_selection - 1".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "enemy_view_offset".into(),
                        LocalFactValue::Expr("$enemy_view_offset - 1".into()),
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
                        LocalFactValue::Expr("$enemy_selection + 1".into()),
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
                        LocalFactValue::Expr("$enemy_selection + 1".into()),
                    ),
                    RuleActionDef::SetLocalFact(
                        "enemy_view_offset".into(),
                        LocalFactValue::Expr("$enemy_view_offset + 1".into()),
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
                        LocalFactValue::Expr("$item_selection - 1".into()),
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
                        LocalFactValue::Expr("$item_selection - 3".into()),
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
                        LocalFactValue::Expr("$item_selection + 5".into()),
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
                        LocalFactValue::Expr("$item_selection + 1".into()),
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
                        LocalFactValue::Expr("$item_selection + 1".into()),
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
                        LocalFactValue::Expr("$item_selection + 3".into()),
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
                        LocalFactValue::Expr("$item_selection - 5".into()),
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
                        LocalFactValue::Expr("$item_selection - 1".into()),
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
                        LocalFactValue::Expr("$item_selection - 2".into()),
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
                        LocalFactValue::Expr("$item_selection + 2".into()),
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
                        LocalFactValue::Expr("$act_selection - 1".into()),
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
                        LocalFactValue::Expr("$act_selection + 1".into()),
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
                        LocalFactValue::Expr("$act_selection - 2".into()),
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
                        LocalFactValue::Expr("$act_selection + 2".into()),
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
                        LocalFactValue::Expr("$mercy_selection - 1".into()),
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
                        LocalFactValue::Expr("$mercy_selection + 1".into()),
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
