//! View asset for `battle/view/undertale.view.ron`.
//!
//! `battle/view/undertale.view.ron` 的 view 资源。

use anyhow::Result;
use souprune_schema::view::*;
use souprune_vessel::prelude::*;

/// Emit this asset.
///
/// 生成当前资源。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资源的类型化值。
pub fn asset() -> ViewLayoutAsset {
    view_layout(vec![
        view_node("BattleBox")
            .tags(vec!["BattleBox"])
            .texts(vec![
                view_text("BattleDialogue", "{{dialogue_text}}", "DTM-Mono")
                    .world_scale(vector2(26.0, 26.0))
                    .translation(vector3(-267.5, 50.0, 1.0))
                    .line_height(1.025)
                    .character_spacing(3.0)
                    .word_spacing(-9.0)
                    .visible_when(
                        "$depth == 0 && $dialogue_visible == true && $narration_visible == false",
                    ),
                view_text(
                    "EnemyNames",
                    concat!(
                        "{|name, i| in $enemy_names[$enemy_view_offset..$enemy_view_offset+$enemy",
                        "_display_limit] => \"* {name}\"}",
                    ),
                    "DTM-Mono",
                )
                .world_scale(vector2(26.0, 26.0))
                .translation(vector3(-220.0, 50.0, 1.0))
                .line_height(1.025)
                .character_spacing(3.0)
                .word_spacing(-9.0)
                .visible_when(concat!(
                    "$depth == 1 && ($menu_context == 0 || $menu_context == 1 || $menu_contex",
                    "t == 3) && $enemy_names >= 1",
                )),
                view_text(
                    "ActOptionsLeft",
                    "{|name, i| in $action_labels[0..$act_count step 2] => \"* {name}\"}",
                    "DTM-Mono",
                )
                .world_scale(vector2(26.0, 26.0))
                .translation(vector3(-220.0, 50.0, 1.0))
                .line_height(1.025)
                .character_spacing(3.0)
                .word_spacing(-9.0)
                .visible_when("$depth == 2 && $menu_context == 1"),
                view_text(
                    "ActOptionsRight",
                    "{|name, i| in $action_labels[1..$act_count step 2] => \"* {name}\"}",
                    "DTM-Mono",
                )
                .world_scale(vector2(26.0, 26.0))
                .translation(vector3(35.75, 50.0, 1.0))
                .line_height(1.025)
                .character_spacing(3.0)
                .word_spacing(-9.0)
                .visible_when("$depth == 2 && $menu_context == 1"),
                view_text(
                    "MercyOptions",
                    "{|name, i| in $mercy_labels[0..$mercy_count] => \"* {name}\"}",
                    "DTM-Mono",
                )
                .world_scale(vector2(26.0, 26.0))
                .translation(vector3(-220.0, 50.0, 1.0))
                .line_height(1.025)
                .character_spacing(3.0)
                .word_spacing(-9.0)
                .visible_when("$depth == 2 && $menu_context == 3"),
                view_text(
                    "ItemGridLeft",
                    concat!(
                        "{|name, i| in $item_display_names[floor($item_selection/4)*4..floor($ite",
                        "m_selection/4)*4+4 step 2] => \"* {name}\"}",
                    ),
                    "DTM-Mono",
                )
                .world_scale(vector2(26.0, 26.0))
                .translation(vector3(-220.0, 50.0, 1.0))
                .line_height(1.025)
                .character_spacing(3.0)
                .word_spacing(-9.0)
                .visible_when("$depth == 1 && $menu_context == 2"),
                view_text(
                    "ItemGridRight",
                    concat!(
                        "{|name, i| in $item_display_names[floor($item_selection/4)*4+1..floor($i",
                        "tem_selection/4)*4+4 step 2] => \"* {name}\"}",
                    ),
                    "DTM-Mono",
                )
                .world_scale(vector2(26.0, 26.0))
                .translation(vector3(35.75, 50.0, 1.0))
                .line_height(1.025)
                .character_spacing(3.0)
                .word_spacing(-9.0)
                .visible_when("$depth == 1 && $menu_context == 2"),
                view_text("ItemPageText", "PAGE {{item_page}}", "DTM-Mono")
                    .world_scale(vector2(26.0, 26.0))
                    .translation(vector3(66.0, -14.5, 1.0))
                    .character_spacing(3.0)
                    .word_spacing(-9.0)
                    .visible_when("$depth == 1 && $menu_context == 2 && $item_page_count > 1"),
                view_text("NarrationText", "{{dialogue_text}}", "DTM-Mono")
                    .world_scale(vector2(26.0, 26.0))
                    .translation(vector3(-267.5, 50.0, 1.0))
                    .line_height(1.025)
                    .character_spacing(3.0)
                    .word_spacing(-9.0)
                    .visible_when("$narration_visible == true && $depth == 0"),
            ])
            .view_box(
                view_box(566.0, 130.0)
                    .border_width(5.0)
                    .offset(vector3(0.0, -80.0, 0.0))
                    .structure_file("view/structures/view_box.sdf.ron"),
            )
            .children(vec![view_node("EnemyHpBar")
                .visible_when(concat!(
                    "$depth == 1 && $menu_context == 0 && @i >= $enemy_view_offset && @i < $e",
                    "nemy_view_offset + $enemy_display_limit",
                ))
                .sprite(
                    view_sprite("procedural://white_pixel")
                        .translation(vector3(
                            expression("15 * max_strlen($enemy_names) - 125"),
                            expression("31.25 - (@i - $enemy_view_offset) * 32.0"),
                            10.0,
                        ))
                        .scale(vector3(100.75, 17.0, 1.0))
                        .pivot(vector2(0.0, 0.5))
                        .material(
                            material("assets/shaders/enemy_hp_bar_sprite.wgsl")
                                .static_parameter("alpha", 1.0)
                                .static_parameter("half_width", 50.0)
                                .expression_parameter(
                                    "hp_ratio",
                                    "$enemy_hps[@i] / $enemy_hp_maxs[@i]",
                                )
                                .static_parameter("lag_ratio", 1.0)
                                .lag_animation(
                                    lag_animation("hp_ratio", "lag_ratio")
                                        .easing(EasingDef::OutCirc),
                                ),
                        ),
                )
                .repeat(repeat("enemy_names").index_variable("i"))]),
        view_node("MenuCursor")
            .visible_when("$depth == 0 && $interactable")
            .sprite(
                view_sprite("common/view/heart")
                    .color(red())
                    .translation(vector3(
                        expression(concat!(
                "if($button_selection == 0, -272.0, if($button_selection == 1, -119.0, if",
                "($button_selection == 2, 41.0, 196.0)))",
            )),
                        expression("-214.0"),
                        expression("2.0"),
                    )),
            ),
        view_node("EnemySelectionCursor")
            .visible_when(concat!(
                "$depth == 1 && $interactable && ($menu_context == 0 || $menu_context == ",
                "1 || $menu_context == 3)",
            ))
            .sprite(
                view_sprite("common/view/heart")
                    .color(red())
                    .translation(vector3(
                        expression("-248.0"),
                        expression("-45.5 - ($enemy_selection - $enemy_view_offset) * 32.0"),
                        expression("10.0"),
                    )),
            ),
        view_node("ActSelectionCursor")
            .visible_when("$depth == 2 && $interactable && $menu_context == 1")
            .sprite(
                view_sprite("common/view/heart")
                    .color(red())
                    .translation(vector3(
                        expression("if($act_selection % 2 == 0, -248.0, 11.5)"),
                        expression("-45.5 - floor($act_selection / 2) * 32.0"),
                        expression("10.0"),
                    )),
            ),
        view_node("MercySelectionCursor")
            .visible_when("$depth == 2 && $interactable && $menu_context == 3")
            .sprite(
                view_sprite("common/view/heart")
                    .color(red())
                    .translation(vector3(
                        expression("-248.0"),
                        expression("-45.5 - $mercy_selection * 32.0"),
                        expression("10.0"),
                    )),
            ),
        view_node("ItemSelectionCursor")
            .visible_when("$depth == 1 && $interactable && $menu_context == 2")
            .sprite(
                view_sprite("common/view/heart")
                    .color(red())
                    .translation(vector3(
                        expression("if($item_selection % 2 == 0, -248.0, 0.0)"),
                        expression("-45.5 - floor($item_selection % 4 / 2) * 32.0"),
                        expression("10.0"),
                    )),
            ),
        view_node("BtnFight").sprite(
            view_sprite("assets/textures/battle/view/fight/false.png")
                .translation(vector3(-233.0, -213.0, 1.0)),
        ),
        view_node("BtnFightSelected")
            .visible_when(concat!(
                "$button_selection == 0 && $buttons_visible == true && $interactable == t",
                "rue",
            ))
            .sprite(
                view_sprite("assets/textures/battle/view/fight/true.png")
                    .translation(vector3(-233.0, -213.0, 1.5)),
            ),
        view_node("BtnAct").sprite(
            view_sprite("assets/textures/battle/view/act/false.png")
                .translation(vector3(-80.0, -213.0, 1.0)),
        ),
        view_node("BtnActSelected")
            .visible_when(concat!(
                "$button_selection == 1 && $buttons_visible == true && $interactable == t",
                "rue",
            ))
            .sprite(
                view_sprite("assets/textures/battle/view/act/true.png")
                    .translation(vector3(-80.0, -213.0, 1.5)),
            ),
        view_node("BtnItem").sprite(
            view_sprite("assets/textures/battle/view/item/false.png")
                .translation(vector3(80.0, -213.0, 1.0)),
        ),
        view_node("BtnItemSelected")
            .visible_when(concat!(
                "$button_selection == 2 && $buttons_visible == true && $interactable == t",
                "rue",
            ))
            .sprite(
                view_sprite("assets/textures/battle/view/item/true.png")
                    .translation(vector3(80.0, -213.0, 1.5)),
            ),
        view_node("BtnMercy").sprite(
            view_sprite("assets/textures/battle/view/mercy/false.png")
                .translation(vector3(235.0, -213.0, 1.0)),
        ),
        view_node("BtnMercySelected")
            .visible_when(concat!(
                "$button_selection == 3 && $buttons_visible == true && $interactable == t",
                "rue",
            ))
            .sprite(
                view_sprite("assets/textures/battle/view/mercy/true.png")
                    .translation(vector3(235.0, -213.0, 1.5)),
            ),
        view_node("DumbTarget")
            .visible_when("$fight_target_visible == true")
            .sprite(
                view_sprite("assets/textures/battle/view/dumb_target.png")
                    .translation(vector3(-1.0, -80.5, 10.5)),
            ),
        view_node("AttackBar")
            .visible_when("$fight_target_visible == true")
            .view_box(
                view_box(6.0, 120.0)
                    .border_width(4.25)
                    .offset(vector3(expression("$fight:bar_x"), -80.5, 11.0))
                    .structure_file("view/structures/attack_bar.sdf.ron"),
            ),
        view_node("BattleHUD")
            .texts(vec![
                view_text("PlayerName", "{$player:name}", "battlehud")
                    .world_scale(vector2(24.0, 24.0))
                    .translation(vector3(-290.0, -156.5, 1.0)),
                view_text("PlayerLevelLabel", "{{battle/ui:LV}}", "battlehud")
                    .world_scale(vector2(24.0, 24.0))
                    .translation(vector3(-187.5, -156.5, 1.0)),
                view_text("PlayerLevelValue", "{$player:lv}", "battlehud")
                    .world_scale(vector2(24.0, 24.0))
                    .translation(vector3(-148.5, -156.5, 1.0)),
                view_text("HPValueCurrent", "{$player:hp}", "battlehud")
                    .world_scale(vector2(24.0, 24.0))
                    .translation(vector3(
                        expression("-5.5 + ($player:hp_max - 20) * 94.5 / 79"),
                        -156.5,
                        1.0,
                    )),
                view_text("HPSeparator", "/", "battlehud")
                    .world_scale(vector2(24.0, 24.0))
                    .translation(vector3(
                        expression("33.5 + ($player:hp_max - 20) * 94.5 / 79"),
                        -156.5,
                        1.0,
                    )),
                view_text("HPValueMax", "{$player:hp_max}", "battlehud")
                    .world_scale(vector2(24.0, 24.0))
                    .translation(vector3(
                        expression("57.5 + ($player:hp_max - 20) * 94.5 / 79"),
                        -156.5,
                        1.0,
                    )),
            ])
            .children(vec![
                view_node("HPSprite").sprite(
                    view_sprite("assets/textures/battle/view/hpname.png")
                        .translation(vector3(-64.5, -170.0, 1.0)),
                ),
                view_node("HPBar").sprite(
                    view_sprite("procedural://white_pixel")
                        .translation(vector3(-45.0, -170.5, 1.0))
                        .scale(vector3(
                            expression("25.0 + ($player:hp_max - 20) * 95.0 / 79"),
                            20.5,
                            1.0,
                        ))
                        .pivot(vector2(0.0, 0.5))
                        .material(
                            material("assets/shaders/hp_bar_sprite.wgsl")
                                .static_parameter("alpha", 1.0)
                                .expression_parameter(
                                    "half_width",
                                    "40.0 + ($player:hp_max - 20) * 95.0 / 79 / 2",
                                )
                                .expression_parameter("hp_ratio", "$player:hp / $player:hp_max")
                                .static_parameter("lag_ratio", 1.0)
                                .lag_animation(
                                    lag_animation("hp_ratio", "lag_ratio")
                                        .easing(EasingDef::OutCirc),
                                ),
                        ),
                ),
            ]),
    ])
    .require_file("battle/rules/menu_navigation.fre.ron")
    .require_file("battle/rules/menu_confirm.fre.ron")
    .require_file("battle/rules/menu_cancel.fre.ron")
    .require_file("battle/rules/dialogue_stop.fre.ron")
    .require_file("battle/rules/fight_hit.fre.ron")
    .require_file("battle/rules/dialogue_test.fre.ron")
    .require_file("narrative/dialogue.fre.ron")
    .require_interface(
        "enemies",
        vec![
            "enemy_ids: StringList",
            "enemy_names: StringList",
            "enemy_hps: IntList",
            "enemy_hp_maxs: IntList",
        ],
    )
    .require_interface(
        "current_enemy",
        vec![
            "action_labels: StringList",
            "action_sequences: StringList",
            "action_params: StringList",
        ],
    )
    .initial_facts(vec![
        ("act_count", InitialFactValue::Int(0)),
        ("act_selection", InitialFactValue::Int(0)),
        ("action_param", InitialFactValue::String("".into())),
        ("button_selection", InitialFactValue::Int(0)),
        ("buttons_visible", InitialFactValue::Bool(true)),
        ("confirm_pressed", InitialFactValue::Bool(false)),
        ("depth", InitialFactValue::Int(0)),
        ("dialogue:replay_on_resume", InitialFactValue::Bool(true)),
        ("dialogue_text", InitialFactValue::String("".into())),
        ("dialogue_visible", InitialFactValue::Bool(false)),
        ("enemy_display_limit", InitialFactValue::Int(3)),
        ("enemy_selection", InitialFactValue::Int(0)),
        ("enemy_view_offset", InitialFactValue::Int(0)),
        ("fight_target_visible", InitialFactValue::Bool(false)),
        ("interactable", InitialFactValue::Bool(false)),
        ("item_selection", InitialFactValue::Int(0)),
        ("menu_context", InitialFactValue::Int(0)),
        ("mercy_count", InitialFactValue::Int(0)),
        ("mercy_selection", InitialFactValue::Int(0)),
        ("narration_visible", InitialFactValue::Bool(false)),
        ("selection_confirmed", InitialFactValue::Bool(false)),
        ("view_rules_loaded", InitialFactValue::Bool(false)),
    ])
    .world_space(true)
}
