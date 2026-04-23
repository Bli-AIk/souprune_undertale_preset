//! View asset for `overworld/view/damage_flash.view.ron`.
//!
//! `overworld/view/damage_flash.view.ron` 的 view 资源。

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
        roots: Vec::from([ViewNodeDef {
            name: "ChaseHUD".into(),
            texts: Vec::from([
                TextDef {
                    id: "PlayerName".into(),
                    font: "battlehud".into(),
                    content: Some("{$player:name}".into()),
                    world_scale: vector2(24.0, 24.0),
                    transform: SerializableTransform {
                        translation: Some(vector3(-145.0, -97.0, 501.0)),
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
                        translation: Some(vector3(-93.0, -97.0, 501.0)),
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
                        translation: Some(vector3(-74.0, -97.0, 501.0)),
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
                        translation: Some(vector3_value(
                            expression("-3.0 + ($player:hp_max - 20) * 47.0 / 79"),
                            static_float(-97.0),
                            static_float(501.0),
                        )),
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
                        translation: Some(vector3_value(
                            expression("17.0 + ($player:hp_max - 20) * 47.0 / 79"),
                            static_float(-97.0),
                            static_float(501.0),
                        )),
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
                        translation: Some(vector3_value(
                            expression("29.0 + ($player:hp_max - 20) * 47.0 / 79"),
                            static_float(-97.0),
                            static_float(501.0),
                        )),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ]),
            children: Vec::from([
                ViewNodeDef {
                    name: "HPSprite".into(),
                    sprite: Some(SpriteDef {
                        visual: Visual("assets/textures/battle/view/hpname.png".into()),
                        transform: Some(SerializableTransform {
                            translation: Some(vector3(-32.0, -105.0, 501.0)),
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
                            translation: Some(vector3(-22.0, -105.0, 501.0)),
                            scale: Some(vector3_value(
                                expression("12.5 + ($player:hp_max - 20) * 47.5 / 79"),
                                static_float(10.0),
                                static_float(1.0),
                            )),
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
                                        "20.0 + ($player:hp_max - 20) * 47.5 / 79 / 2".into(),
                                    ),
                                ),
                                (
                                    "hp_ratio".into(),
                                    MaterialParamValue::Expr("$player:hp / $player:hp_max".into()),
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
        }]),
        ..Default::default()
    }
}
