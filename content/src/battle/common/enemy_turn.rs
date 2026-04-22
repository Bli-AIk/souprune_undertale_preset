//! Bootstrapped code asset for `battle/common/enemy_turn.sequence.ron`.
//!
//! `battle/common/enemy_turn.sequence.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 发射当前 bootstrap 资产。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资产的类型化值。
pub fn asset() -> SequenceAsset {
    SequenceAsset {
        mode: None,
        rules_file: None,
        exits: vec![].into_iter().collect(),
        chapters: vec![
            Chapter::SetPlayer(PlayerAction::Spawn {
                config_path: "battle/players/player.battle_player.ron".into(),
                position: Some((0.0, -80.0)),
            }),
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
