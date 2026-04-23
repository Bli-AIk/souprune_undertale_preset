//! View asset for `overworld/view/damage_flash.view.ron`.
//!
//! `overworld/view/damage_flash.view.ron` 的 view 资源。

use anyhow::Result;
use souprune_schema::view::*;
use souprune_vessel::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> ViewLayoutAsset {
    view_layout(vec![view_node("ChaseHUD")
        .texts(vec![
            view_text("PlayerName", "{$player:name}", "battlehud")
                .world_scale(vector2(24.0, 24.0))
                .translation(vector3(-145.0, -97.0, 501.0)),
            view_text("PlayerLevelLabel", "{{battle/ui:LV}}", "battlehud")
                .world_scale(vector2(24.0, 24.0))
                .translation(vector3(-93.0, -97.0, 501.0)),
            view_text("PlayerLevelValue", "{$player:lv}", "battlehud")
                .world_scale(vector2(24.0, 24.0))
                .translation(vector3(-74.0, -97.0, 501.0)),
            view_text("HPValueCurrent", "{$player:hp}", "battlehud")
                .world_scale(vector2(24.0, 24.0))
                .translation(vector3(
                    expression("-3.0 + ($player:hp_max - 20) * 47.0 / 79"),
                    -97.0,
                    501.0,
                )),
            view_text("HPSeparator", "/", "battlehud")
                .world_scale(vector2(24.0, 24.0))
                .translation(vector3(
                    expression("17.0 + ($player:hp_max - 20) * 47.0 / 79"),
                    -97.0,
                    501.0,
                )),
            view_text("HPValueMax", "{$player:hp_max}", "battlehud")
                .world_scale(vector2(24.0, 24.0))
                .translation(vector3(
                    expression("29.0 + ($player:hp_max - 20) * 47.0 / 79"),
                    -97.0,
                    501.0,
                )),
        ])
        .children(vec![
            view_node("HPSprite").sprite(
                view_sprite("assets/textures/battle/view/hpname.png")
                    .translation(vector3(-32.0, -105.0, 501.0)),
            ),
            view_node("HPBar").sprite(
                view_sprite("procedural://white_pixel")
                    .translation(vector3(-22.0, -105.0, 501.0))
                    .scale(vector3(
                        expression("12.5 + ($player:hp_max - 20) * 47.5 / 79"),
                        10.0,
                        1.0,
                    ))
                    .pivot(vector2(0.0, 0.5))
                    .material(
                        material("assets/shaders/hp_bar_sprite.wgsl")
                            .static_parameter("alpha", 1.0)
                            .expression_parameter(
                                "half_width",
                                "20.0 + ($player:hp_max - 20) * 47.5 / 79 / 2",
                            )
                            .expression_parameter("hp_ratio", "$player:hp / $player:hp_max")
                            .static_parameter("lag_ratio", 1.0)
                            .lag_animation(
                                lag_animation("hp_ratio", "lag_ratio").easing(EasingDef::OutCirc),
                            ),
                    ),
            ),
        ])])
}
