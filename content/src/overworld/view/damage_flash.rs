//! Bootstrapped code asset for `overworld/view/damage_flash.view.ron`.
//!
//! `overworld/view/damage_flash.view.ron` 的 bootstrap 代码资源。

use anyhow::Result;
use souprune_schema::val::*;
use souprune_schema::view::*;
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
pub fn asset() -> ViewLayoutAsset {
    ViewLayout {
        roots: vec![ViewNodeDef {
            name: "ChaseHUD".into(),
            tags: vec![],
            style: StyleDef {
                width: None,
                height: None,
                left: None,
                right: None,
                top: None,
                bottom: None,
                position_type: None,
                flex_direction: None,
                justify_content: None,
                align_items: None,
            },
            visible_when: None,
            background_color: None,
            border_color: None,
            image: None,
            sprite: None,
            state_sprite: None,
            texts: vec![
                TextDef {
                    id: "PlayerName".into(),
                    content: Some("{$player:name}".into()),
                    font: "battlehud".into(),
                    align: None,
                    anchor: None,
                    world_scale: (Val::Static(24.0), Val::Static(24.0)),
                    color: (
                        Val::Static(1.0),
                        Val::Static(1.0),
                        Val::Static(1.0),
                        Val::Static(1.0),
                    ),
                    transform: SerializableTransform {
                        translation: Some((
                            Val::Static(-145.0),
                            Val::Static(-97.0),
                            Val::Static(501.0),
                        )),
                        rotation: None,
                        scale: None,
                    },
                    line_height: None,
                    char_spacing: None,
                    word_spacing: None,
                    conditional_style: None,
                    visible_when: None,
                },
                TextDef {
                    id: "PlayerLevelLabel".into(),
                    content: Some("{{battle/ui:LV}}".into()),
                    font: "battlehud".into(),
                    align: None,
                    anchor: None,
                    world_scale: (Val::Static(24.0), Val::Static(24.0)),
                    color: (
                        Val::Static(1.0),
                        Val::Static(1.0),
                        Val::Static(1.0),
                        Val::Static(1.0),
                    ),
                    transform: SerializableTransform {
                        translation: Some((
                            Val::Static(-93.0),
                            Val::Static(-97.0),
                            Val::Static(501.0),
                        )),
                        rotation: None,
                        scale: None,
                    },
                    line_height: None,
                    char_spacing: None,
                    word_spacing: None,
                    conditional_style: None,
                    visible_when: None,
                },
                TextDef {
                    id: "PlayerLevelValue".into(),
                    content: Some("{$player:lv}".into()),
                    font: "battlehud".into(),
                    align: None,
                    anchor: None,
                    world_scale: (Val::Static(24.0), Val::Static(24.0)),
                    color: (
                        Val::Static(1.0),
                        Val::Static(1.0),
                        Val::Static(1.0),
                        Val::Static(1.0),
                    ),
                    transform: SerializableTransform {
                        translation: Some((
                            Val::Static(-74.0),
                            Val::Static(-97.0),
                            Val::Static(501.0),
                        )),
                        rotation: None,
                        scale: None,
                    },
                    line_height: None,
                    char_spacing: None,
                    word_spacing: None,
                    conditional_style: None,
                    visible_when: None,
                },
                TextDef {
                    id: "HPValueCurrent".into(),
                    content: Some("{$player:hp}".into()),
                    font: "battlehud".into(),
                    align: None,
                    anchor: None,
                    world_scale: (Val::Static(24.0), Val::Static(24.0)),
                    color: (
                        Val::Static(1.0),
                        Val::Static(1.0),
                        Val::Static(1.0),
                        Val::Static(1.0),
                    ),
                    transform: SerializableTransform {
                        translation: Some((
                            Val::Expr("-3.0 + ($player:hp_max - 20) * 47.0 / 79".into()),
                            Val::Static(-97.0),
                            Val::Static(501.0),
                        )),
                        rotation: None,
                        scale: None,
                    },
                    line_height: None,
                    char_spacing: None,
                    word_spacing: None,
                    conditional_style: None,
                    visible_when: None,
                },
                TextDef {
                    id: "HPSeparator".into(),
                    content: Some("/".into()),
                    font: "battlehud".into(),
                    align: None,
                    anchor: None,
                    world_scale: (Val::Static(24.0), Val::Static(24.0)),
                    color: (
                        Val::Static(1.0),
                        Val::Static(1.0),
                        Val::Static(1.0),
                        Val::Static(1.0),
                    ),
                    transform: SerializableTransform {
                        translation: Some((
                            Val::Expr("17.0 + ($player:hp_max - 20) * 47.0 / 79".into()),
                            Val::Static(-97.0),
                            Val::Static(501.0),
                        )),
                        rotation: None,
                        scale: None,
                    },
                    line_height: None,
                    char_spacing: None,
                    word_spacing: None,
                    conditional_style: None,
                    visible_when: None,
                },
                TextDef {
                    id: "HPValueMax".into(),
                    content: Some("{$player:hp_max}".into()),
                    font: "battlehud".into(),
                    align: None,
                    anchor: None,
                    world_scale: (Val::Static(24.0), Val::Static(24.0)),
                    color: (
                        Val::Static(1.0),
                        Val::Static(1.0),
                        Val::Static(1.0),
                        Val::Static(1.0),
                    ),
                    transform: SerializableTransform {
                        translation: Some((
                            Val::Expr("29.0 + ($player:hp_max - 20) * 47.0 / 79".into()),
                            Val::Static(-97.0),
                            Val::Static(501.0),
                        )),
                        rotation: None,
                        scale: None,
                    },
                    line_height: None,
                    char_spacing: None,
                    word_spacing: None,
                    conditional_style: None,
                    visible_when: None,
                },
            ],
            view_box: None,
            children: vec![
                ViewNodeDef {
                    name: "HPSprite".into(),
                    tags: vec![],
                    style: StyleDef {
                        width: None,
                        height: None,
                        left: None,
                        right: None,
                        top: None,
                        bottom: None,
                        position_type: None,
                        flex_direction: None,
                        justify_content: None,
                        align_items: None,
                    },
                    visible_when: None,
                    background_color: None,
                    border_color: None,
                    image: None,
                    sprite: Some(SpriteDef {
                        visual: Visual("assets/textures/battle/view/hpname.png".into()),
                        initial_state: None,
                        color: None,
                        flip_x: false,
                        flip_y: false,
                        transform: Some(SerializableTransform {
                            translation: Some((
                                Val::Static(-32.0),
                                Val::Static(-105.0),
                                Val::Static(501.0),
                            )),
                            rotation: None,
                            scale: None,
                        }),
                        pivot: None,
                        frame_duration: None,
                        visible_when: None,
                        material: None,
                    }),
                    state_sprite: None,
                    texts: vec![],
                    view_box: None,
                    children: vec![],
                    repeat: None,
                },
                ViewNodeDef {
                    name: "HPBar".into(),
                    tags: vec![],
                    style: StyleDef {
                        width: None,
                        height: None,
                        left: None,
                        right: None,
                        top: None,
                        bottom: None,
                        position_type: None,
                        flex_direction: None,
                        justify_content: None,
                        align_items: None,
                    },
                    visible_when: None,
                    background_color: None,
                    border_color: None,
                    image: None,
                    sprite: Some(SpriteDef {
                        visual: Visual("procedural://white_pixel".into()),
                        initial_state: None,
                        color: None,
                        flip_x: false,
                        flip_y: false,
                        transform: Some(SerializableTransform {
                            translation: Some((
                                Val::Static(-22.0),
                                Val::Static(-105.0),
                                Val::Static(501.0),
                            )),
                            rotation: None,
                            scale: Some((
                                Val::Expr("12.5 + ($player:hp_max - 20) * 47.5 / 79".into()),
                                Val::Static(10.0),
                                Val::Static(1.0),
                            )),
                        }),
                        pivot: Some((Val::Static(0.0), Val::Static(0.5))),
                        frame_duration: None,
                        visible_when: None,
                        material: Some(MaterialDef {
                            shader: "assets/shaders/hp_bar_sprite.wgsl".into(),
                            params: vec![
                                ("lag_ratio".into(), MaterialParamValue::Static(1.0)),
                                ("alpha".into(), MaterialParamValue::Static(1.0)),
                                (
                                    "hp_ratio".into(),
                                    MaterialParamValue::Expr("$player:hp / $player:hp_max".into()),
                                ),
                                (
                                    "half_width".into(),
                                    MaterialParamValue::Expr(
                                        "20.0 + ($player:hp_max - 20) * 47.5 / 79 / 2".into(),
                                    ),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                            animations: Some(MaterialAnimationsDef {
                                lag: Some(LagAnimationDef {
                                    source: "hp_ratio".into(),
                                    target: "lag_ratio".into(),
                                    delay: 0.2,
                                    duration: 0.4,
                                    easing: EasingDef::OutCirc,
                                }),
                            }),
                            texture: None,
                        }),
                    }),
                    state_sprite: None,
                    texts: vec![],
                    view_box: None,
                    children: vec![],
                    repeat: None,
                },
            ],
            repeat: None,
        }],
        requires: vec![],
        facts: None,
        world_space: false,
        coordinate_system: CoordinateSystem::Standard,
    }
}
