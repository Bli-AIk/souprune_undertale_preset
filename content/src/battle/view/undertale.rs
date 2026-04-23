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
    ViewLayout {
        roots: Vec::from([
            ViewNodeDef {
                name: "BattleBox".into(),
                tags: Vec::from(["BattleBox".into()]),
                texts: Vec::from([
                    TextDef {
                        id: "BattleDialogue".into(),
                        font: "DTM-Mono".into(),
                        content: Some("{{dialogue_text}}".into()),
                        world_scale: vector2(26.0, 26.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-267.5, 50.0, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.025),
                        char_spacing: Some(3.0),
                        word_spacing: Some(-9.0),
                        visible_when: Some(
                            "$depth == 0 && $dialogue_visible == true && $narration_visible == false"
                                .into(),
                        ),
                        ..Default::default()
                    },
                    TextDef {
                        id: "EnemyNames".into(),
                        font: "DTM-Mono".into(),
                        content: Some(
                            concat!(
                                "{|name, i| in $enemy_names[$enemy_view_offset..$enemy_view_offset+$enemy",
                                "_display_limit] => \"* {name}\"}",
                            )
                                .into(),
                        ),
                        world_scale: vector2(26.0, 26.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-220.0, 50.0, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.025),
                        char_spacing: Some(3.0),
                        word_spacing: Some(-9.0),
                        visible_when: Some(
                            concat!(
                                "$depth == 1 && ($menu_context == 0 || $menu_context == 1 || $menu_contex",
                                "t == 3) && $enemy_names >= 1",
                            )
                                .into(),
                        ),
                        ..Default::default()
                    },
                    TextDef {
                        id: "ActOptionsLeft".into(),
                        font: "DTM-Mono".into(),
                        content: Some(
                            "{|name, i| in $action_labels[0..$act_count step 2] => \"* {name}\"}"
                                .into(),
                        ),
                        world_scale: vector2(26.0, 26.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-220.0, 50.0, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.025),
                        char_spacing: Some(3.0),
                        word_spacing: Some(-9.0),
                        visible_when: Some("$depth == 2 && $menu_context == 1".into()),
                        ..Default::default()
                    },
                    TextDef {
                        id: "ActOptionsRight".into(),
                        font: "DTM-Mono".into(),
                        content: Some(
                            "{|name, i| in $action_labels[1..$act_count step 2] => \"* {name}\"}"
                                .into(),
                        ),
                        world_scale: vector2(26.0, 26.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(35.75, 50.0, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.025),
                        char_spacing: Some(3.0),
                        word_spacing: Some(-9.0),
                        visible_when: Some("$depth == 2 && $menu_context == 1".into()),
                        ..Default::default()
                    },
                    TextDef {
                        id: "MercyOptions".into(),
                        font: "DTM-Mono".into(),
                        content: Some(
                            "{|name, i| in $mercy_labels[0..$mercy_count] => \"* {name}\"}"
                                .into(),
                        ),
                        world_scale: vector2(26.0, 26.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-220.0, 50.0, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.025),
                        char_spacing: Some(3.0),
                        word_spacing: Some(-9.0),
                        visible_when: Some("$depth == 2 && $menu_context == 3".into()),
                        ..Default::default()
                    },
                    TextDef {
                        id: "ItemGridLeft".into(),
                        font: "DTM-Mono".into(),
                        content: Some(
                            concat!(
                                "{|name, i| in $item_display_names[floor($item_selection/4)*4..floor($ite",
                                "m_selection/4)*4+4 step 2] => \"* {name}\"}",
                            )
                                .into(),
                        ),
                        world_scale: vector2(26.0, 26.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-220.0, 50.0, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.025),
                        char_spacing: Some(3.0),
                        word_spacing: Some(-9.0),
                        visible_when: Some("$depth == 1 && $menu_context == 2".into()),
                        ..Default::default()
                    },
                    TextDef {
                        id: "ItemGridRight".into(),
                        font: "DTM-Mono".into(),
                        content: Some(
                            concat!(
                                "{|name, i| in $item_display_names[floor($item_selection/4)*4+1..floor($i",
                                "tem_selection/4)*4+4 step 2] => \"* {name}\"}",
                            )
                                .into(),
                        ),
                        world_scale: vector2(26.0, 26.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(35.75, 50.0, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.025),
                        char_spacing: Some(3.0),
                        word_spacing: Some(-9.0),
                        visible_when: Some("$depth == 1 && $menu_context == 2".into()),
                        ..Default::default()
                    },
                    TextDef {
                        id: "ItemPageText".into(),
                        font: "DTM-Mono".into(),
                        content: Some("PAGE {{item_page}}".into()),
                        world_scale: vector2(26.0, 26.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(66.0, -14.5, 1.0)),
                            ..Default::default()
                        },
                        char_spacing: Some(3.0),
                        word_spacing: Some(-9.0),
                        visible_when: Some(
                            "$depth == 1 && $menu_context == 2 && $item_page_count > 1"
                                .into(),
                        ),
                        ..Default::default()
                    },
                    TextDef {
                        id: "NarrationText".into(),
                        font: "DTM-Mono".into(),
                        content: Some("{{dialogue_text}}".into()),
                        world_scale: vector2(26.0, 26.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-267.5, 50.0, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.025),
                        char_spacing: Some(3.0),
                        word_spacing: Some(-9.0),
                        visible_when: Some(
                            "$narration_visible == true && $depth == 0".into(),
                        ),
                        ..Default::default()
                    },
                ]),
                view_box: Some(ViewBoxLogicDef {
                    width: 566.0,
                    height: 130.0,
                    border_width: 5.0,
                    offset: vector3(0.0, -80.0, 0.0),
                    structure_file: Some("view/structures/view_box.sdf.ron".into()),
                    ..Default::default()
                }),
                children: Vec::from([
                    ViewNodeDef {
                        name: "EnemyHpBar".into(),
                        visible_when: Some(
                            concat!(
                                "$depth == 1 && $menu_context == 0 && @i >= $enemy_view_offset && @i < $e",
                                "nemy_view_offset + $enemy_display_limit",
                            )
                                .into(),
                        ),
                        sprite: Some(SpriteDef {
                            visual: Visual("procedural://white_pixel".into()),
                            transform: Some(SerializableTransform {
                                translation: Some(
                                    vector3_value(
                                        expression("15 * max_strlen($enemy_names) - 125"),
                                        expression("31.25 - (@i - $enemy_view_offset) * 32.0"),
                                        static_float(10.0),
                                    ),
                                ),
                                scale: Some(vector3(100.75, 17.0, 1.0)),
                                ..Default::default()
                            }),
                            pivot: Some(vector2(0.0, 0.5)),
                            material: Some(MaterialDef {
                                shader: "assets/shaders/enemy_hp_bar_sprite.wgsl".into(),
                                params: Vec::from([
                                        ("alpha".into(), MaterialParamValue::Static(1.0)),
                                        ("half_width".into(), MaterialParamValue::Static(50.0)),
                                        (
                                            "hp_ratio".into(),
                                            MaterialParamValue::Expr(
                                                "$enemy_hps[@i] / $enemy_hp_maxs[@i]".into(),
                                            ),
                                        ),
                                        ("lag_ratio".into(), MaterialParamValue::Static(1.0)),
                                    ])
                                    .into_iter()
                                    .collect(),
                                animations: Some(MaterialAnimationsDef {
                                    lag: Some(LagAnimationDef {
                                        source: "hp_ratio".into(),
                                        target: "lag_ratio".into(),
                                        easing: EasingDef::OutCirc,
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        repeat: Some(RepeatDef {
                            source: "enemy_names".into(),
                            index_var: Some("i".into()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            },
            ViewNodeDef {
                name: "MenuCursor".into(),
                visible_when: Some("$depth == 0 && $interactable".into()),
                sprite: Some(SpriteDef {
                    visual: Visual("common/view/heart".into()),
                    color: Some(red()),
                    transform: Some(SerializableTransform {
                        translation: Some(
                            vector3_value(
                                expression(
                                    concat!(
                                        "if($button_selection == 0, -272.0, if($button_selection == 1, -119.0, if",
                                        "($button_selection == 2, 41.0, 196.0)))",
                                    ),
                                ),
                                expression("-214.0"),
                                expression("2.0"),
                            ),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "EnemySelectionCursor".into(),
                visible_when: Some(
                    concat!(
                        "$depth == 1 && $interactable && ($menu_context == 0 || $menu_context == ",
                        "1 || $menu_context == 3)",
                    )
                        .into(),
                ),
                sprite: Some(SpriteDef {
                    visual: Visual("common/view/heart".into()),
                    color: Some(red()),
                    transform: Some(SerializableTransform {
                        translation: Some(
                            vector3_value(
                                expression("-248.0"),
                                expression(
                                    "-45.5 - ($enemy_selection - $enemy_view_offset) * 32.0",
                                ),
                                expression("10.0"),
                            ),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "ActSelectionCursor".into(),
                visible_when: Some(
                    "$depth == 2 && $interactable && $menu_context == 1".into(),
                ),
                sprite: Some(SpriteDef {
                    visual: Visual("common/view/heart".into()),
                    color: Some(red()),
                    transform: Some(SerializableTransform {
                        translation: Some(
                            vector3_value(
                                expression("if($act_selection % 2 == 0, -248.0, 11.5)"),
                                expression("-45.5 - floor($act_selection / 2) * 32.0"),
                                expression("10.0"),
                            ),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "MercySelectionCursor".into(),
                visible_when: Some(
                    "$depth == 2 && $interactable && $menu_context == 3".into(),
                ),
                sprite: Some(SpriteDef {
                    visual: Visual("common/view/heart".into()),
                    color: Some(red()),
                    transform: Some(SerializableTransform {
                        translation: Some(
                            vector3_value(
                                expression("-248.0"),
                                expression("-45.5 - $mercy_selection * 32.0"),
                                expression("10.0"),
                            ),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "ItemSelectionCursor".into(),
                visible_when: Some(
                    "$depth == 1 && $interactable && $menu_context == 2".into(),
                ),
                sprite: Some(SpriteDef {
                    visual: Visual("common/view/heart".into()),
                    color: Some(red()),
                    transform: Some(SerializableTransform {
                        translation: Some(
                            vector3_value(
                                expression("if($item_selection % 2 == 0, -248.0, 0.0)"),
                                expression("-45.5 - floor($item_selection % 4 / 2) * 32.0"),
                                expression("10.0"),
                            ),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnFight".into(),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/fight/false.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(-233.0, -213.0, 1.0)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnFightSelected".into(),
                visible_when: Some(
                    concat!(
                        "$button_selection == 0 && $buttons_visible == true && $interactable == t",
                        "rue",
                    )
                        .into(),
                ),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/fight/true.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(-233.0, -213.0, 1.5)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnAct".into(),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/act/false.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(-80.0, -213.0, 1.0)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnActSelected".into(),
                visible_when: Some(
                    concat!(
                        "$button_selection == 1 && $buttons_visible == true && $interactable == t",
                        "rue",
                    )
                        .into(),
                ),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/act/true.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(-80.0, -213.0, 1.5)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnItem".into(),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/item/false.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(80.0, -213.0, 1.0)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnItemSelected".into(),
                visible_when: Some(
                    concat!(
                        "$button_selection == 2 && $buttons_visible == true && $interactable == t",
                        "rue",
                    )
                        .into(),
                ),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/item/true.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(80.0, -213.0, 1.5)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnMercy".into(),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/mercy/false.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(235.0, -213.0, 1.0)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BtnMercySelected".into(),
                visible_when: Some(
                    concat!(
                        "$button_selection == 3 && $buttons_visible == true && $interactable == t",
                        "rue",
                    )
                        .into(),
                ),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/mercy/true.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(235.0, -213.0, 1.5)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "DumbTarget".into(),
                visible_when: Some("$fight_target_visible == true".into()),
                sprite: Some(SpriteDef {
                    visual: Visual("assets/textures/battle/view/dumb_target.png".into()),
                    transform: Some(SerializableTransform {
                        translation: Some(vector3(-1.0, -80.5, 10.5)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "AttackBar".into(),
                visible_when: Some("$fight_target_visible == true".into()),
                view_box: Some(ViewBoxLogicDef {
                    width: 6.0,
                    height: 120.0,
                    border_width: 4.25,
                    offset: vector3_value(
                        expression("$fight:bar_x"),
                        static_float(-80.5),
                        static_float(11.0),
                    ),
                    structure_file: Some("view/structures/attack_bar.sdf.ron".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "BattleHUD".into(),
                texts: Vec::from([
                    TextDef {
                        id: "PlayerName".into(),
                        font: "battlehud".into(),
                        content: Some("{$player:name}".into()),
                        world_scale: vector2(24.0, 24.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-290.0, -156.5, 1.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TextDef {
                        id: "PlayerLevelLabel".into(),
                        font: "battlehud".into(),
                        content: Some("{{battle/ui:LV}}".into()),
                        world_scale: vector2(24.0, 24.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-187.5, -156.5, 1.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TextDef {
                        id: "PlayerLevelValue".into(),
                        font: "battlehud".into(),
                        content: Some("{$player:lv}".into()),
                        world_scale: vector2(24.0, 24.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-148.5, -156.5, 1.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TextDef {
                        id: "HPValueCurrent".into(),
                        font: "battlehud".into(),
                        content: Some("{$player:hp}".into()),
                        world_scale: vector2(24.0, 24.0),
                        transform: SerializableTransform {
                            translation: Some(
                                vector3_value(
                                    expression("-5.5 + ($player:hp_max - 20) * 94.5 / 79"),
                                    static_float(-156.5),
                                    static_float(1.0),
                                ),
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TextDef {
                        id: "HPSeparator".into(),
                        font: "battlehud".into(),
                        content: Some("/".into()),
                        world_scale: vector2(24.0, 24.0),
                        transform: SerializableTransform {
                            translation: Some(
                                vector3_value(
                                    expression("33.5 + ($player:hp_max - 20) * 94.5 / 79"),
                                    static_float(-156.5),
                                    static_float(1.0),
                                ),
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    TextDef {
                        id: "HPValueMax".into(),
                        font: "battlehud".into(),
                        content: Some("{$player:hp_max}".into()),
                        world_scale: vector2(24.0, 24.0),
                        transform: SerializableTransform {
                            translation: Some(
                                vector3_value(
                                    expression("57.5 + ($player:hp_max - 20) * 94.5 / 79"),
                                    static_float(-156.5),
                                    static_float(1.0),
                                ),
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ]),
                children: Vec::from([
                    ViewNodeDef {
                        name: "HPSprite".into(),
                        sprite: Some(SpriteDef {
                            visual: Visual(
                                "assets/textures/battle/view/hpname.png".into(),
                            ),
                            transform: Some(SerializableTransform {
                                translation: Some(vector3(-64.5, -170.0, 1.0)),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    ViewNodeDef {
                        name: "HPBar".into(),
                        sprite: Some(SpriteDef {
                            visual: Visual("procedural://white_pixel".into()),
                            transform: Some(SerializableTransform {
                                translation: Some(vector3(-45.0, -170.5, 1.0)),
                                scale: Some(
                                    vector3_value(
                                        expression("25.0 + ($player:hp_max - 20) * 95.0 / 79"),
                                        static_float(20.5),
                                        static_float(1.0),
                                    ),
                                ),
                                ..Default::default()
                            }),
                            pivot: Some(vector2(0.0, 0.5)),
                            material: Some(MaterialDef {
                                shader: "assets/shaders/hp_bar_sprite.wgsl".into(),
                                params: Vec::from([
                                        ("alpha".into(), MaterialParamValue::Static(1.0)),
                                        (
                                            "half_width".into(),
                                            MaterialParamValue::Expr(
                                                "40.0 + ($player:hp_max - 20) * 95.0 / 79 / 2".into(),
                                            ),
                                        ),
                                        (
                                            "hp_ratio".into(),
                                            MaterialParamValue::Expr(
                                                "$player:hp / $player:hp_max".into(),
                                            ),
                                        ),
                                        ("lag_ratio".into(), MaterialParamValue::Static(1.0)),
                                    ])
                                    .into_iter()
                                    .collect(),
                                animations: Some(MaterialAnimationsDef {
                                    lag: Some(LagAnimationDef {
                                        source: "hp_ratio".into(),
                                        target: "lag_ratio".into(),
                                        easing: EasingDef::OutCirc,
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            },
        ]),
        requires: Vec::from([
            DataRequirement::File("battle/rules/menu_navigation.fre.ron".into()),
            DataRequirement::File("battle/rules/menu_confirm.fre.ron".into()),
            DataRequirement::File("battle/rules/menu_cancel.fre.ron".into()),
            DataRequirement::File("battle/rules/dialogue_stop.fre.ron".into()),
            DataRequirement::File("battle/rules/fight_hit.fre.ron".into()),
            DataRequirement::File("battle/rules/dialogue_test.fre.ron".into()),
            DataRequirement::File("narrative/dialogue.fre.ron".into()),
            DataRequirement::Interface {
                interface: "enemies".into(),
                expects: Vec::from([
                    "enemy_ids: StringList".into(),
                    "enemy_names: StringList".into(),
                    "enemy_hps: IntList".into(),
                    "enemy_hp_maxs: IntList".into(),
                ]),
            },
            DataRequirement::Interface {
                interface: "current_enemy".into(),
                expects: Vec::from([
                    "action_labels: StringList".into(),
                    "action_sequences: StringList".into(),
                    "action_params: StringList".into(),
                ]),
            },
        ]),
        facts: Some(
            Vec::from([
                    ("act_count".into(), InitialFactValue::Int(0)),
                    ("act_selection".into(), InitialFactValue::Int(0)),
                    ("action_param".into(), InitialFactValue::String("".into())),
                    ("button_selection".into(), InitialFactValue::Int(0)),
                    ("buttons_visible".into(), InitialFactValue::Bool(true)),
                    ("confirm_pressed".into(), InitialFactValue::Bool(false)),
                    ("depth".into(), InitialFactValue::Int(0)),
                    ("dialogue:replay_on_resume".into(), InitialFactValue::Bool(true)),
                    ("dialogue_text".into(), InitialFactValue::String("".into())),
                    ("dialogue_visible".into(), InitialFactValue::Bool(false)),
                    ("enemy_display_limit".into(), InitialFactValue::Int(3)),
                    ("enemy_selection".into(), InitialFactValue::Int(0)),
                    ("enemy_view_offset".into(), InitialFactValue::Int(0)),
                    ("fight_target_visible".into(), InitialFactValue::Bool(false)),
                    ("interactable".into(), InitialFactValue::Bool(false)),
                    ("item_selection".into(), InitialFactValue::Int(0)),
                    ("menu_context".into(), InitialFactValue::Int(0)),
                    ("mercy_count".into(), InitialFactValue::Int(0)),
                    ("mercy_selection".into(), InitialFactValue::Int(0)),
                    ("narration_visible".into(), InitialFactValue::Bool(false)),
                    ("selection_confirmed".into(), InitialFactValue::Bool(false)),
                    ("view_rules_loaded".into(), InitialFactValue::Bool(false)),
                ])
                .into_iter()
                .collect(),
        ),
        world_space: true,
        ..Default::default()
    }
}
