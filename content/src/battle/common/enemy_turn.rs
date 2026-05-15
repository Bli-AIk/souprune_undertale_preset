//! Code representation of `battle/common/enemy_turn.sequence.ron`.
//!
//! `battle/common/enemy_turn.sequence.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_cauld_ron::prelude::*;
use std::collections::HashMap;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> SequenceAsset {
    SequenceAsset {
        mode: None,
        rules_file: None,
        exits: vec![].into_iter().collect(),
        chapters: vec![
            Chapter::SpawnBehavior {
                behavior_id: "undertale_battle_area".into(),
                context: Some("battle".into()),
            },
            set_battle_box_bounds(-0.5, -80.0, 565.0, 130.0, 0.0),
            Chapter::Custom {
                action_type: "BindPlayerToBox".into(),
                params: HashMap::from([("id".into(), "main".into())]),
            },
            spawn_battle_player(),
            Chapter::PickEnemyTurn {
                enemy_id: None,
                enemy_id_fact: Some("_param_enemy_id".into()),
                group: None,
                group_fact: Some("_param_turn_group".into()),
            },
            Chapter::Sequence(vec![
                Chapter::Wait(0.5),
                Chapter::SetPlayer(PlayerAction::Despawn),
            ]),
        ],
    }
}

fn spawn_battle_player() -> Chapter {
    Chapter::Custom {
        action_type: "SpawnBattlePlayer".into(),
        params: HashMap::from([
            (
                "texture".into(),
                "assets/textures/common/view/heart.png".into(),
            ),
            ("x".into(), "0.0".into()),
            ("y".into(), "-80.0".into()),
            ("z".into(), "10.0".into()),
            ("behavior_id".into(), "soul_red".into()),
            ("behavior_context".into(), "battle".into()),
            ("mode_scope".into(), "battle".into()),
            ("box_id".into(), "main".into()),
            ("physics_radius".into(), "8.0".into()),
            ("trigger_half_width".into(), "2.0".into()),
            ("trigger_half_height".into(), "2.0".into()),
            ("color_red".into(), "1.0".into()),
            ("color_green".into(), "0.0".into()),
            ("color_blue".into(), "0.0".into()),
            ("color_alpha".into(), "1.0".into()),
            ("invincibility_duration".into(), "1.0".into()),
            ("flash_interval".into(), "0.1".into()),
            ("normal_color_red".into(), "1.0".into()),
            ("normal_color_green".into(), "0.0".into()),
            ("normal_color_blue".into(), "0.0".into()),
            ("normal_color_alpha".into(), "1.0".into()),
            ("flash_color_red".into(), "0.5".into()),
            ("flash_color_green".into(), "0.0".into()),
            ("flash_color_blue".into(), "0.0".into()),
            ("flash_color_alpha".into(), "1.0".into()),
            (
                "damage_sound".into(),
                "assets/audios/sfx/hurtsound.wav".into(),
            ),
        ]),
    }
}

fn set_battle_box_bounds(
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,
    duration: f32,
) -> Chapter {
    Chapter::Custom {
        action_type: "SetBattleBoxBounds".into(),
        params: HashMap::from([
            ("id".into(), "main".into()),
            ("center_x".into(), center_x.to_string()),
            ("center_y".into(), center_y.to_string()),
            ("width".into(), width.to_string()),
            ("height".into(), height.to_string()),
            ("duration".into(), duration.to_string()),
        ]),
    }
}
