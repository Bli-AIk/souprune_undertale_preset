//! Code representation of `battle/common/narration.sequence.ron`.
//!
//! `battle/common/narration.sequence.ron` 的代码表示。

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
            Chapter::EmitFactEvent {
                event_id: "dialogue:battle_narration:stop".into(),
                data: vec![].into_iter().collect(),
            },
            Chapter::SetViewFact {
                key: "dialogue:replay_on_resume".into(),
                value: FactValueMatch::Bool(false),
            },
            Chapter::SetViewFact {
                key: "depth".into(),
                value: FactValueMatch::Int(0),
            },
            Chapter::SetViewFact {
                key: "buttons_visible".into(),
                value: FactValueMatch::Bool(false),
            },
            Chapter::SetViewFact {
                key: "narration_visible".into(),
                value: FactValueMatch::Bool(true),
            },
            Chapter::ModifyFact {
                modifications: vec![
                    FactModificationDef::Set {
                        key: "dialogue:pending_channel".into(),
                        value: FactValueMatch::String("battle_narration".into()),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:pending_mortar_path".into(),
                        value: expr::fact("mortar_path").into(),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:pending_mortar_node".into(),
                        value: expr::fact("action_param").into(),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:pending_start".into(),
                        value: FactValueMatch::Bool(true),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:battle_narration:has_typewriter".into(),
                        value: FactValueMatch::Bool(true),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:battle_narration:has_focus".into(),
                        value: FactValueMatch::Bool(true),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:battle_narration:voice".into(),
                        value: FactValueMatch::String(
                            "assets/audios/voice/voice_typewriter_default.wav".into(),
                        ),
                    },
                    FactModificationDef::Set {
                        key: "dialogue:battle_narration:text_style".into(),
                        value: FactValueMatch::String("battle_narration".into()),
                    },
                ],
            },
            Chapter::AwaitFact {
                condition: "$dialogue:battle_narration:active == true".into(),
                local: false,
            },
            Chapter::AwaitFact {
                condition: "$dialogue:battle_narration:active == false".into(),
                local: false,
            },
            Chapter::SetViewFact {
                key: "narration_visible".into(),
                value: FactValueMatch::Bool(false),
            },
            Chapter::SetViewFact {
                key: "buttons_visible".into(),
                value: FactValueMatch::Bool(true),
            },
            Chapter::SetViewFact {
                key: "dialogue:replay_on_resume".into(),
                value: FactValueMatch::Bool(true),
            },
        ],
    }
}
