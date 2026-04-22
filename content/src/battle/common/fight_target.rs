//! Bootstrapped code asset for `battle/common/fight_target.sequence.ron`.
//!
//! `battle/common/fight_target.sequence.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::sequence::*;
use souprune_schema::val::*;
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
            Chapter::EmitFactEvent {
                event_id: "dialogue:stop".into(),
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
                key: "dialogue_text".into(),
                value: FactValueMatch::String("".into()),
            },
            Chapter::ModifyFact {
                modifications: vec![
                    FactModificationDef::Set {
                        key: "fight:bar_done".into(),
                        value: FactValueMatch::Bool(false),
                    },
                    FactModificationDef::Set {
                        key: "fight:confirmed".into(),
                        value: FactValueMatch::Bool(false),
                    },
                    FactModificationDef::Set {
                        key: "fight:bar_complete".into(),
                        value: FactValueMatch::Bool(false),
                    },
                    FactModificationDef::Set {
                        key: "fight:bar_x".into(),
                        value: FactValueMatch::Float(-274.0),
                    },
                    FactModificationDef::Set {
                        key: "fight:bar_flash_on".into(),
                        value: FactValueMatch::Bool(false),
                    },
                    FactModificationDef::Set {
                        key: "fight:bar_active".into(),
                        value: FactValueMatch::Bool(true),
                    },
                ],
            },
            Chapter::ModifyViewElement {
                selector: ElementSelector::LocalName("AttackBar".into()),
                modification: ElementModification::Reset,
            },
            Chapter::ModifyViewElement {
                selector: ElementSelector::LocalName("DumbTarget".into()),
                modification: ElementModification::Reset,
            },
            Chapter::SetViewFact {
                key: "fight_target_visible".into(),
                value: FactValueMatch::Bool(true),
            },
            Chapter::SpawnBehavior {
                behavior_id: "fight_bar".into(),
                context: Some("battle".into()),
            },
            Chapter::AwaitFact {
                condition: "$fight:bar_complete == true".into(),
                local: false,
            },
            Chapter::ModifyFact {
                modifications: vec![FactModificationDef::Set {
                    key: "fight:bar_active".into(),
                    value: FactValueMatch::Bool(false),
                }],
            },
            Chapter::Parallel(vec![
                Chapter::SetViewElement {
                    selector: ElementSelector::LocalName("AttackBar".into()),
                    target: TweenTarget::Alpha {
                        from: None,
                        to: Val::Static(0.0),
                    },
                    duration: Some(0.01),
                    easing: EaseKindRepr::Linear,
                    wait_for_completion: true,
                },
                Chapter::SetViewElement {
                    selector: ElementSelector::LocalName("DumbTarget".into()),
                    target: TweenTarget::Scale {
                        from: None,
                        to: Vec3Tuple::Positional(
                            Val::Static(0.0),
                            Val::Static(1.0),
                            Val::Static(1.0),
                        ),
                    },
                    duration: Some(0.4),
                    easing: EaseKindRepr::Linear,
                    wait_for_completion: true,
                },
                Chapter::SetViewElement {
                    selector: ElementSelector::LocalName("DumbTarget".into()),
                    target: TweenTarget::Alpha {
                        from: None,
                        to: Val::Static(0.0),
                    },
                    duration: Some(0.4),
                    easing: EaseKindRepr::Linear,
                    wait_for_completion: true,
                },
            ]),
            Chapter::SetViewFact {
                key: "fight_target_visible".into(),
                value: FactValueMatch::Bool(false),
            },
            Chapter::ModifyFact {
                modifications: vec![
                    FactModificationDef::Set {
                        key: "fight:bar_active".into(),
                        value: FactValueMatch::Bool(false),
                    },
                    FactModificationDef::Set {
                        key: "fight:bar_done".into(),
                        value: FactValueMatch::Bool(false),
                    },
                    FactModificationDef::Set {
                        key: "fight:bar_complete".into(),
                        value: FactValueMatch::Bool(false),
                    },
                    FactModificationDef::Set {
                        key: "fight:confirmed".into(),
                        value: FactValueMatch::Bool(false),
                    },
                    FactModificationDef::Set {
                        key: "fight:bar_x".into(),
                        value: FactValueMatch::Float(-274.0),
                    },
                    FactModificationDef::Set {
                        key: "fight:bar_flash_on".into(),
                        value: FactValueMatch::Bool(false),
                    },
                ],
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
