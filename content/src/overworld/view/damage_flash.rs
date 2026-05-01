//! View asset for `overworld/view/damage_flash.view.ron`.
//!
//! `overworld/view/damage_flash.view.ron` 的 view 资源。

use anyhow::Result;
use souprune_schema::view::*;
use souprune_cauld_ron::prelude::*;

/// Emit this asset.
///
/// 生成当前资源。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

fn player_hp_max_delta(width: f64) -> expr::Expression {
    (expr::fact("player:hp_max") - 20) * width / 79
}

fn chase_hud_hp_x(base: f64) -> FloatOrExpr {
    (base + player_hp_max_delta(47.0)).into()
}

fn chase_hp_bar_width() -> FloatOrExpr {
    (12.5 + player_hp_max_delta(47.5)).into()
}

fn chase_hp_bar_half_width() -> MaterialParamValue {
    (20.0 + player_hp_max_delta(47.5) / 2).into()
}

fn player_hp_ratio_param() -> MaterialParamValue {
    (expr::fact("player:hp") / expr::fact("player:hp_max")).into()
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
                        translation: Some(vector3(
                            chase_hud_hp_x(-3.0),
                            -97.0,
                            501.0,
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
                        translation: Some(vector3(
                            chase_hud_hp_x(17.0),
                            -97.0,
                            501.0,
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
                        translation: Some(vector3(
                            chase_hud_hp_x(29.0),
                            -97.0,
                            501.0,
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
                            scale: Some(vector3(
                                chase_hp_bar_width(),
                                10.0,
                                1.0,
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
                                    chase_hp_bar_half_width(),
                                ),
                                ("hp_ratio".into(), player_hp_ratio_param()),
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
                                })
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
