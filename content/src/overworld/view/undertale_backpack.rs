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
    view_layout(vec![
        view_node("MenuBox")
            .visible_when("true")
            .texts(vec![
                view_text("MenuTextItem", "{{overworld/ui:ITEM}}", "DTM-Sans")
                    .world_scale(vector2(13.25, 13.25))
                    .translation(vector3(-9.5, 27.075, 1.0))
                    .line_height(1.125)
                    .conditional_color("player.inventory.is_empty", color(0.5, 0.5, 0.5, 1.0)),
                view_text("MenuTextStat", "{{overworld/ui:STAT}}", "DTM-Sans")
                    .world_scale(vector2(13.25, 13.25))
                    .translation(vector3(-9.5, 9.25, 1.0))
                    .line_height(0.0),
            ])
            .view_box(
                view_box(65.0, 68.0)
                    .border_width(3.0)
                    .offset(vector3(-108.5, -1.0, 0.0))
                    .structure_file("view/structures/view_box.sdf.ron"),
            )
            .children(vec![view_node("MenuCursor")
                .visible_when("$depth == 0")
                .sprite(
                    view_sprite("common/view/heartsmall")
                        .color(red())
                        .translation(vector3(
                            expression("-19.0"),
                            expression("18.5 + (-18.0 * $selection)"),
                            expression("6.0"),
                        )),
                )]),
        view_node("InfoBox")
            .visible_when("true")
            .texts(vec![
                view_text("NameText", "{$player:name}", "DTM-Sans")
                    .world_scale(vector2(13.0, 13.0))
                    .translation(vector3(-28.5, 23.25, 1.0))
                    .line_height(1.0)
                    .character_spacing(2.25)
                    .word_spacing(0.0),
                view_text(
                    "HUDInfoText",
                    concat!(
                        "{{overworld/ui:LV}}  {$player:lv}",
                        "\\n",
                        "{{overworld/ui:HP}}  {$player:hp}/{$player:hp_max}",
                    ),
                    "hud",
                )
                .world_scale(vector2(8.0, 8.0))
                .translation(vector3(-28.5, 3.5, 1.0))
                .line_height(2.925)
                .character_spacing(0.015)
                .word_spacing(0.065),
                view_text("HUDGoldText", "g   {$player:gold}", "hud")
                    .world_scale(vector2(8.0, 8.0))
                    .translation(vector3(-28.75, -14.55, 1.0))
                    .line_height(0.0),
            ])
            .view_box(
                view_box(65.0, 49.0)
                    .border_width(3.0)
                    .offset(vector3(-108.5, -68.5, 0.0))
                    .structure_file("view/structures/view_box.sdf.ron"),
            ),
        view_node("ItemBox")
            .visible_when("$depth == 1 || $depth == 2")
            .texts(vec![
                view_text("ItemLayerList", "{{data:player.inventory}}", "DTM-Sans")
                    .world_scale(vector2(13.25, 13.25))
                    .translation(vector3(-64.25, 76.5, 1.0))
                    .line_height(1.0),
                view_text("ItemLayerOptionUse", "{{overworld/ui:USE}}", "DTM-Sans")
                    .world_scale(vector2(13.25, 13.25))
                    .translation(vector3(-64.25, -63.5, 1.0))
                    .line_height(1.2)
                    .visible_when("$depth == 1 || $depth == 2"),
                view_text("ItemLayerOptionInfo", "{{overworld/ui:INFO}}", "DTM-Sans")
                    .world_scale(vector2(13.25, 13.25))
                    .translation(vector3(-16.25, -63.5, 1.0))
                    .line_height(1.2)
                    .visible_when("$depth == 1 || $depth == 2"),
                view_text("ItemLayerOptionDrop", "{{overworld/ui:DROP}}", "DTM-Sans")
                    .world_scale(vector2(13.25, 13.25))
                    .translation(vector3(40.75, -63.5, 1.0))
                    .line_height(1.2)
                    .visible_when("$depth == 1 || $depth == 2"),
            ])
            .view_box(
                view_box(167.0, 175.0)
                    .border_width(3.0)
                    .offset(vector3(20.5, 3.5, 0.0))
                    .structure_file("view/structures/view_box.sdf.ron"),
            )
            .children(vec![
                view_node("ItemCursor").visible_when("$depth == 1").sprite(
                    view_sprite("common/view/heartsmall")
                        .color(red())
                        .translation(vector3(
                            expression("-72.0"),
                            expression("68 + (-16.0 * $selection)"),
                            expression("6.0"),
                        )),
                ),
                view_node("OptionsCursor")
                    .visible_when("$depth == 2")
                    .sprite(
                        view_sprite("common/view/heartsmall")
                            .color(red())
                            .translation(vector3(
                                expression(
                                    "if($selection == 0, -72.0, if($selection == 1, -24.25, 33.0))",
                                ),
                                expression("-72.0"),
                                expression("6.0"),
                            )),
                    ),
            ]),
        view_node("StatusBox")
            .visible_when("$depth == 3")
            .texts(vec![
                view_text("StatusLayerName", "\"{$player:name}\"", "DTM-Sans")
                    .world_scale(vector2(13.5, 13.5))
                    .translation(vector3(-72.5, 88.4, 1.0))
                    .line_height(1.15)
                    .character_spacing(-1.25)
                    .word_spacing(-21.585),
                view_text(
                    "StatusLayerLv",
                    "{{overworld/ui:LV}}   {$player:lv}",
                    "DTM-Sans",
                )
                .world_scale(vector2(13.5, 13.5))
                .translation(vector3(-72.5, 58.5, 1.0))
                .line_height(0.975)
                .character_spacing(-2.385)
                .word_spacing(-6.5),
                view_text(
                    "StatusLayerHp",
                    "{{overworld/ui:HP}}  {$player:hp} / {$player:hp_max}",
                    "DTM-Sans",
                )
                .world_scale(vector2(13.5, 13.5))
                .translation(vector3(-72.5, 42.5, 1.0))
                .line_height(0.975)
                .character_spacing(-2.385)
                .word_spacing(4.7),
                view_text(
                    "StatusLayerCombatLeft",
                    concat!(
                        "{{overworld/ui:ATK}}  {{data:player.total_attack}} ({{data:player.weapon",
                        "_atk}})",
                        "\\n",
                        "{{overworld/ui:DEF}}  {{data:player.total_defense}} ({{data:player.armor",
                        "_def}})",
                    ),
                    "DTM-Sans",
                )
                .world_scale(vector2(13.5, 13.5))
                .translation(vector3(-72.626, 10.42, 1.0))
                .line_height(0.98)
                .character_spacing(-2.825)
                .word_spacing(6.21),
                view_text(
                    "StatusLayerCombatRight",
                    concat!(
                        "{{overworld/ui:EXP}}: {$player:exp}",
                        "\\n",
                        "{{overworld/ui:NEXT}}: {$player:next_exp}",
                    ),
                    "DTM-Sans",
                )
                .world_scale(vector2(13.5, 13.5))
                .translation(vector3(11.485, 10.66, 1.0))
                .line_height(0.975)
                .character_spacing(-4.137)
                .word_spacing(15.79),
                view_text(
                    "StatusLayerEquipment",
                    concat!(
                        "{{overworld/ui:WEAPON}}: {{data:player.weapon}}",
                        "\\n",
                        "{{overworld/ui:ARMOR}}: {{data:player.armor}}",
                    ),
                    "DTM-Sans",
                )
                .world_scale(vector2(13.5, 13.5))
                .translation(vector3(-72.387, -35.895, 1.0))
                .line_height(0.945)
                .character_spacing(-1.062)
                .word_spacing(2.573),
                view_text(
                    "StatusLayerGold",
                    "{{overworld/ui:GOLD}}: {$player:gold}",
                    "DTM-Sans",
                )
                .world_scale(vector2(13.5, 13.5))
                .translation(vector3(-72.377, -71.352, 1.0))
                .line_height(1.15)
                .character_spacing(-0.632)
                .word_spacing(0.0),
            ])
            .view_box(
                view_box(167.0, 202.5)
                    .border_width(3.0)
                    .offset(vector3(20.5, -10.5, 0.0))
                    .structure_file("view/structures/view_box.sdf.ron"),
            ),
    ])
    .require_file("overworld/rules/backpack.fre.ron")
    .initial_facts(vec![
        ("depth", InitialFactValue::Int(0)),
        ("interactable", InitialFactValue::Bool(true)),
        ("selection", InitialFactValue::Int(0)),
    ])
}
