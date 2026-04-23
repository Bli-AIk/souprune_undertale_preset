//! View asset for `overworld/view/undertale_backpack.view.ron`.
//!
//! `overworld/view/undertale_backpack.view.ron` 的 view 资源。

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
                name: "MenuBox".into(),
                visible_when: Some("true".into()),
                texts: Vec::from([
                    TextDef {
                        id: "MenuTextItem".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{{overworld/ui:ITEM}}".into()),
                        world_scale: vector2(13.25, 13.25),
                        transform: SerializableTransform {
                            translation: Some(vector3(-9.5, 27.075, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.125),
                        conditional_style: Some(ConditionalStyleDef {
                            condition: "player.inventory.is_empty".into(),
                            color: color(0.5, 0.5, 0.5, 1.0),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    TextDef {
                        id: "MenuTextStat".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{{overworld/ui:STAT}}".into()),
                        world_scale: vector2(13.25, 13.25),
                        transform: SerializableTransform {
                            translation: Some(vector3(-9.5, 9.25, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(0.0),
                        ..Default::default()
                    },
                ]),
                view_box: Some(ViewBoxLogicDef {
                    width: 65.0,
                    height: 68.0,
                    border_width: 3.0,
                    offset: vector3(-108.5, -1.0, 0.0),
                    structure_file: Some("view/structures/view_box.sdf.ron".into()),
                    ..Default::default()
                }),
                children: Vec::from([
                    ViewNodeDef {
                        name: "MenuCursor".into(),
                        visible_when: Some("$depth == 0".into()),
                        sprite: Some(SpriteDef {
                            visual: Visual("common/view/heartsmall".into()),
                            color: Some(red()),
                            transform: Some(SerializableTransform {
                                translation: Some(
                                    vector3_value(
                                        expression("-19.0"),
                                        expression("18.5 + (-18.0 * $selection)"),
                                        expression("6.0"),
                                    ),
                                ),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            },
            ViewNodeDef {
                name: "InfoBox".into(),
                visible_when: Some("true".into()),
                texts: Vec::from([
                    TextDef {
                        id: "NameText".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{$player:name}".into()),
                        world_scale: vector2(13.0, 13.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-28.5, 23.25, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.0),
                        char_spacing: Some(2.25),
                        word_spacing: Some(0.0),
                        ..Default::default()
                    },
                    TextDef {
                        id: "HUDInfoText".into(),
                        font: "hud".into(),
                        content: Some(
                            concat!(
                                "{{overworld/ui:LV}}  {$player:lv}\\n{{overworld/ui:HP}}  {$player:hp}/{$p",
                                "layer:hp_max}",
                            )
                                .into(),
                        ),
                        world_scale: vector2(8.0, 8.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-28.5, 3.5, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(2.925),
                        char_spacing: Some(0.015),
                        word_spacing: Some(0.065),
                        ..Default::default()
                    },
                    TextDef {
                        id: "HUDGoldText".into(),
                        font: "hud".into(),
                        content: Some("g   {$player:gold}".into()),
                        world_scale: vector2(8.0, 8.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-28.75, -14.55, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(0.0),
                        ..Default::default()
                    },
                ]),
                view_box: Some(ViewBoxLogicDef {
                    width: 65.0,
                    height: 49.0,
                    border_width: 3.0,
                    offset: vector3(-108.5, -68.5, 0.0),
                    structure_file: Some("view/structures/view_box.sdf.ron".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "ItemBox".into(),
                visible_when: Some("$depth == 1 || $depth == 2".into()),
                texts: Vec::from([
                    TextDef {
                        id: "ItemLayerList".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{{data:player.inventory}}".into()),
                        world_scale: vector2(13.25, 13.25),
                        transform: SerializableTransform {
                            translation: Some(vector3(-64.25, 76.5, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.0),
                        ..Default::default()
                    },
                    TextDef {
                        id: "ItemLayerOptionUse".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{{overworld/ui:USE}}".into()),
                        world_scale: vector2(13.25, 13.25),
                        transform: SerializableTransform {
                            translation: Some(vector3(-64.25, -63.5, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.2),
                        visible_when: Some("$depth == 1 || $depth == 2".into()),
                        ..Default::default()
                    },
                    TextDef {
                        id: "ItemLayerOptionInfo".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{{overworld/ui:INFO}}".into()),
                        world_scale: vector2(13.25, 13.25),
                        transform: SerializableTransform {
                            translation: Some(vector3(-16.25, -63.5, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.2),
                        visible_when: Some("$depth == 1 || $depth == 2".into()),
                        ..Default::default()
                    },
                    TextDef {
                        id: "ItemLayerOptionDrop".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{{overworld/ui:DROP}}".into()),
                        world_scale: vector2(13.25, 13.25),
                        transform: SerializableTransform {
                            translation: Some(vector3(40.75, -63.5, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.2),
                        visible_when: Some("$depth == 1 || $depth == 2".into()),
                        ..Default::default()
                    },
                ]),
                view_box: Some(ViewBoxLogicDef {
                    width: 167.0,
                    height: 175.0,
                    border_width: 3.0,
                    offset: vector3(20.5, 3.5, 0.0),
                    structure_file: Some("view/structures/view_box.sdf.ron".into()),
                    ..Default::default()
                }),
                children: Vec::from([
                    ViewNodeDef {
                        name: "ItemCursor".into(),
                        visible_when: Some("$depth == 1".into()),
                        sprite: Some(SpriteDef {
                            visual: Visual("common/view/heartsmall".into()),
                            color: Some(red()),
                            transform: Some(SerializableTransform {
                                translation: Some(
                                    vector3_value(
                                        expression("-72.0"),
                                        expression("68 + (-16.0 * $selection)"),
                                        expression("6.0"),
                                    ),
                                ),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    ViewNodeDef {
                        name: "OptionsCursor".into(),
                        visible_when: Some("$depth == 2".into()),
                        sprite: Some(SpriteDef {
                            visual: Visual("common/view/heartsmall".into()),
                            color: Some(red()),
                            transform: Some(SerializableTransform {
                                translation: Some(
                                    vector3_value(
                                        expression(
                                            "if($selection == 0, -72.0, if($selection == 1, -24.25, 33.0))",
                                        ),
                                        expression("-72.0"),
                                        expression("6.0"),
                                    ),
                                ),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            },
            ViewNodeDef {
                name: "StatusBox".into(),
                visible_when: Some("$depth == 3".into()),
                texts: Vec::from([
                    TextDef {
                        id: "StatusLayerName".into(),
                        font: "DTM-Sans".into(),
                        content: Some("\"{$player:name}\"".into()),
                        world_scale: vector2(13.5, 13.5),
                        transform: SerializableTransform {
                            translation: Some(vector3(-72.5, 88.4, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.15),
                        char_spacing: Some(-1.25),
                        word_spacing: Some(-21.585),
                        ..Default::default()
                    },
                    TextDef {
                        id: "StatusLayerLv".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{{overworld/ui:LV}}   {$player:lv}".into()),
                        world_scale: vector2(13.5, 13.5),
                        transform: SerializableTransform {
                            translation: Some(vector3(-72.5, 58.5, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(0.975),
                        char_spacing: Some(-2.385),
                        word_spacing: Some(-6.5),
                        ..Default::default()
                    },
                    TextDef {
                        id: "StatusLayerHp".into(),
                        font: "DTM-Sans".into(),
                        content: Some(
                            "{{overworld/ui:HP}}  {$player:hp} / {$player:hp_max}".into(),
                        ),
                        world_scale: vector2(13.5, 13.5),
                        transform: SerializableTransform {
                            translation: Some(vector3(-72.5, 42.5, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(0.975),
                        char_spacing: Some(-2.385),
                        word_spacing: Some(4.7),
                        ..Default::default()
                    },
                    TextDef {
                        id: "StatusLayerCombatLeft".into(),
                        font: "DTM-Sans".into(),
                        content: Some(
                            concat!(
                                "{{overworld/ui:ATK}}  {{data:player.total_attack}} ({{data:player.weapon",
                                "_atk}})\\n{{overworld/ui:DEF}}  {{data:player.total_defense}} ({{data:pla",
                                "yer.armor_def}})",
                            )
                                .into(),
                        ),
                        world_scale: vector2(13.5, 13.5),
                        transform: SerializableTransform {
                            translation: Some(vector3(-72.626, 10.42, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(0.98),
                        char_spacing: Some(-2.825),
                        word_spacing: Some(6.21),
                        ..Default::default()
                    },
                    TextDef {
                        id: "StatusLayerCombatRight".into(),
                        font: "DTM-Sans".into(),
                        content: Some(
                            concat!(
                                "{{overworld/ui:EXP}}: {$player:exp}\\n{{overworld/ui:NEXT}}: {$player:nex",
                                "t_exp}",
                            )
                                .into(),
                        ),
                        world_scale: vector2(13.5, 13.5),
                        transform: SerializableTransform {
                            translation: Some(vector3(11.485, 10.66, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(0.975),
                        char_spacing: Some(-4.137),
                        word_spacing: Some(15.79),
                        ..Default::default()
                    },
                    TextDef {
                        id: "StatusLayerEquipment".into(),
                        font: "DTM-Sans".into(),
                        content: Some(
                            concat!(
                                "{{overworld/ui:WEAPON}}: {{data:player.weapon}}\\n{{overworld/ui:ARMOR}}:",
                                " {{data:player.armor}}",
                            )
                                .into(),
                        ),
                        world_scale: vector2(13.5, 13.5),
                        transform: SerializableTransform {
                            translation: Some(vector3(-72.387, -35.895, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(0.945),
                        char_spacing: Some(-1.062),
                        word_spacing: Some(2.573),
                        ..Default::default()
                    },
                    TextDef {
                        id: "StatusLayerGold".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{{overworld/ui:GOLD}}: {$player:gold}".into()),
                        world_scale: vector2(13.5, 13.5),
                        transform: SerializableTransform {
                            translation: Some(vector3(-72.377, -71.352, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.15),
                        char_spacing: Some(-0.632),
                        word_spacing: Some(0.0),
                        ..Default::default()
                    },
                ]),
                view_box: Some(ViewBoxLogicDef {
                    width: 167.0,
                    height: 202.5,
                    border_width: 3.0,
                    offset: vector3(20.5, -10.5, 0.0),
                    structure_file: Some("view/structures/view_box.sdf.ron".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        ]),
        requires: Vec::from([
            DataRequirement::File("overworld/rules/backpack.fre.ron".into()),
        ]),
        facts: Some(
            Vec::from([
                    ("depth".into(), InitialFactValue::Int(0)),
                    ("interactable".into(), InitialFactValue::Bool(true)),
                    ("selection".into(), InitialFactValue::Int(0)),
                ])
                .into_iter()
                .collect(),
        ),
        ..Default::default()
    }
}
