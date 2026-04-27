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

fn menu_cursor_y() -> FloatOrExpr {
    (-22.5 + 18.0 * expr::fact("selection")).into()
}

fn item_cursor_y() -> FloatOrExpr {
    (-72.0 + 16.0 * expr::fact("selection")).into()
}

fn options_cursor_x() -> FloatOrExpr {
    expr::if_else(
        expr::fact("selection").equal_to(0),
        -76.0,
        expr::if_else(expr::fact("selection").equal_to(1), -28.0, 29.0),
    )
    .into()
}

fn gms_coordinate_space() -> CoordinateSpaceDef {
    CoordinateSpaceDef {
        axis_origin: vector2(0.0, 0.0),
        y_axis: YAxisDirectionDef::Down,
        rotation: RotationDirectionDef::CounterClockwise,
        extent: CoordinateExtentDef::Explicit((640.0, 480.0)),
    }
}

fn gms_transform(x: f32, y: f32, z: f32) -> SerializableTransform {
    SerializableTransform {
        translation: Some(vector3(x, y, z)),
        ..Default::default()
    }
}

/// Build the typed asset value.
///
/// 构建该资源的类型化值。
pub fn asset() -> ViewLayoutAsset {
    ViewLayout {
        coordinate_space: Some(gms_coordinate_space()),
        roots: Vec::from([
            ViewNodeDef {
                name: "MenuBox".into(),
                transform: Some(gms_transform(51.0, 120.5, 0.0)),
                visible_when: Some("true".into()),
                texts: Vec::from([
                    TextDef {
                        id: "MenuTextItem".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{{overworld/ui:ITEM}}".into()),
                        world_scale: vector2(13.25, 13.25),
                        transform: SerializableTransform {
                            translation: Some(vector3(-9.0, -26.5, 1.0)),
                            ..Default::default()
                        },
                        line_height: Some(1.125),
                        conditional_style: Some(ConditionalStyleDef {
                            condition: "player.inventory.is_empty".into(),
                            color: color(0.5, 0.5, 0.5, 1.0),
                        }),
                        ..Default::default()
                    },
                    TextDef {
                        id: "MenuTextStat".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{{overworld/ui:STAT}}".into()),
                        world_scale: vector2(13.25, 13.25),
                        transform: SerializableTransform {
                            translation: Some(vector3(-9.0, -8.5, 1.0)),
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
                    offset: vector3(0.0, 0.0, 0.0),
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
                                    vector3(
                                        expr::literal(-23.0),
                                        menu_cursor_y(),
                                        expr::literal(6.0),
                                    ),
                                ),
                                ..Default::default()
                            }),
                            pivot: Some(vector2(0.0, 0.0)),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            },
            ViewNodeDef {
                name: "InfoBox".into(),
                transform: Some(gms_transform(51.0, 53.0, 0.0)),
                visible_when: Some("true".into()),
                texts: Vec::from([
                    TextDef {
                        id: "NameText".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{$player:name}".into()),
                        world_scale: vector2(13.0, 13.0),
                        transform: SerializableTransform {
                            translation: Some(vector3(-28.0, -23.0, 1.0)),
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
                            translation: Some(vector3(-28.0, -3.0, 1.0)),
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
                            translation: Some(vector3(-28.0, 15.0, 1.0)),
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
                    offset: vector3(0.0, 0.0, 0.0),
                    structure_file: Some("view/structures/view_box.sdf.ron".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ViewNodeDef {
                name: "ItemBox".into(),
                transform: Some(gms_transform(180.0, 116.0, 0.0)),
                visible_when: Some("$depth == 1 || $depth == 2".into()),
                texts: Vec::from([
                    TextDef {
                        id: "ItemLayerList".into(),
                        font: "DTM-Sans".into(),
                        content: Some("{{data:player.inventory}}".into()),
                        world_scale: vector2(13.25, 13.25),
                        transform: SerializableTransform {
                            translation: Some(vector3(-64.0, -76.0, 1.0)),
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
                            translation: Some(vector3(-64.0, 64.0, 1.0)),
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
                            translation: Some(vector3(-16.0, 64.0, 1.0)),
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
                            translation: Some(vector3(41.0, 64.0, 1.0)),
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
                    offset: vector3(0.0, 0.0, 0.0),
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
                                    vector3(
                                        expr::literal(-76.0),
                                        item_cursor_y(),
                                        expr::literal(6.0),
                                    ),
                                ),
                                ..Default::default()
                            }),
                            pivot: Some(vector2(0.0, 0.0)),
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
                                    vector3(
                                        options_cursor_x(),
                                        expr::literal(68.0),
                                        expr::literal(6.0),
                                    ),
                                ),
                                ..Default::default()
                            }),
                            pivot: Some(vector2(0.0, 0.0)),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            },
            ViewNodeDef {
                name: "StatusBox".into(),
                transform: Some(gms_transform(180.0, 130.0, 0.0)),
                visible_when: Some("$depth == 3".into()),
                texts: Vec::from([
                    TextDef {
                        id: "StatusLayerName".into(),
                        font: "DTM-Sans".into(),
                        content: Some("\"{$player:name}\"".into()),
                        world_scale: vector2(13.5, 13.5),
                        transform: SerializableTransform {
                            translation: Some(vector3(-72.0, -88.0, 1.0)),
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
                            translation: Some(vector3(-72.0, -58.0, 1.0)),
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
                            translation: Some(vector3(-72.0, -42.0, 1.0)),
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
                            translation: Some(vector3(-72.0, -10.0, 1.0)),
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
                            translation: Some(vector3(12.0, -10.0, 1.0)),
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
                            translation: Some(vector3(-72.0, 36.0, 1.0)),
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
                            translation: Some(vector3(-72.0, 72.0, 1.0)),
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
                    offset: vector3(0.0, 0.0, 0.0),
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
