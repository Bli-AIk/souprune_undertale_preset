//! Code representation of `battle/templates/undertale_battle.sequence.ron`.
//!
//! `battle/templates/undertale_battle.sequence.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_cauld_ron::prelude::*;

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
            Chapter::ModifyFact {
                modifications: vec![FactModificationDef::Set {
                    key: "battle_turn_count".into(),
                    value: FactValueMatch::Int(0),
                }],
            },
            Chapter::Loop {
                body: vec![
                    Chapter::EmitFactEvent {
                        event_id: "battle:turn_start".into(),
                        data: vec![].into_iter().collect(),
                    },
                    Chapter::RunSequence {
                        path: None,
                        path_fact: Some("_param_turn_narration".into()),
                        params: vec![].into_iter().collect(),
                    },
                    Chapter::RunSequence {
                        path: Some("battle/common/player_turn.sequence.ron".into()),
                        path_fact: None,
                        params: vec![].into_iter().collect(),
                    },
                    Chapter::Conditional {
                        condition: FactCondition::IsTrue("battle:should_end".into()),
                        then_branch: Box::new(Chapter::Break),
                        else_branch: None,
                    },
                    Chapter::RunSequence {
                        path: Some("battle/common/enemy_turn.sequence.ron".into()),
                        path_fact: None,
                        params: vec![
                            ("turn_group".into(), expr::fact("_param_turn_group").into()),
                            ("enemy_id".into(), expr::fact("_param_enemy_id").into()),
                        ]
                        .into_iter()
                        .collect(),
                    },
                    Chapter::ModifyFact {
                        modifications: vec![FactModificationDef::Increment {
                            key: "battle_turn_count".into(),
                            amount: 1,
                        }],
                    },
                ],
                max_iterations: None,
            },
        ],
    }
}
