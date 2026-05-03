//! Code representation of `narrative/dialogue.ron`.
//!
//! `narrative/dialogue.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::dialogue::*;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> DialogueConfig {
    DialogueConfig {
        auto_pause: AutoPauseConfig {
            default_preset: "normal".into(),
            presets: vec![
                (
                    "fast".into(),
                    vec![
                        ("，".into(), 0.04),
                        ("!".into(), 0.167),
                        ("！".into(), 0.167),
                        ("\n".into(), 0.05),
                        (".".into(), 0.167),
                        (",".into(), 0.04),
                        ("。".into(), 0.167),
                        ("?".into(), 0.167),
                        ("…".into(), 0.167),
                        ("？".into(), 0.167),
                    ]
                    .into_iter()
                    .collect(),
                ),
                (
                    "dramatic".into(),
                    vec![
                        ("：".into(), 0.3),
                        ("\n".into(), 0.2),
                        ("…".into(), 0.667),
                        ("、".into(), 0.15),
                        ("?".into(), 0.5),
                        ("；".into(), 0.3),
                        (".".into(), 0.5),
                        (",".into(), 0.15),
                        ("!".into(), 0.5),
                        (":".into(), 0.3),
                        ("？".into(), 0.5),
                        (";".into(), 0.3),
                        ("！".into(), 0.5),
                        ("。".into(), 0.5),
                        ("，".into(), 0.15),
                    ]
                    .into_iter()
                    .collect(),
                ),
                (
                    "normal".into(),
                    vec![
                        ("：".into(), 0.2),
                        ("！".into(), 0.333),
                        ("\n".into(), 0.15),
                        (":".into(), 0.2),
                        ("，".into(), 0.08),
                        (".".into(), 0.333),
                        ("。".into(), 0.333),
                        ("、".into(), 0.1),
                        (",".into(), 0.08),
                        (";".into(), 0.2),
                        ("？".into(), 0.333),
                        ("!".into(), 0.333),
                        ("?".into(), 0.333),
                        ("…".into(), 0.333),
                        ("；".into(), 0.2),
                    ]
                    .into_iter()
                    .collect(),
                ),
            ]
            .into_iter()
            .collect(),
        },
        voice: VoiceConfig {
            default_preset: "normal".into(),
            presets: vec![
                (
                    "normal".into(),
                    vec![
                        ("：".into(), false),
                        (".".into(), false),
                        ("？".into(), false),
                        ("、".into(), false),
                        (",".into(), false),
                        (":".into(), false),
                        (" ".into(), false),
                        ("，".into(), false),
                        ("…".into(), false),
                        ("\u{3000}".into(), false),
                        ("！".into(), false),
                        ("?".into(), false),
                        ("\n".into(), false),
                        ("。".into(), false),
                        ("；".into(), false),
                        (";".into(), false),
                        ("!".into(), false),
                    ]
                    .into_iter()
                    .collect(),
                ),
                (
                    "dense".into(),
                    vec![("\n".into(), false)].into_iter().collect(),
                ),
                (
                    "whisper".into(),
                    vec![
                        (" ".into(), false),
                        ("\n".into(), false),
                        ("\u{3000}".into(), false),
                    ]
                    .into_iter()
                    .collect(),
                ),
            ]
            .into_iter()
            .collect(),
        },
        text_animation: TextAnimationConfigDef {
            default_preset: "default".into(),
            presets: vec![
                ("default".into(), TextAnimationPresetDef {
                    display: TextDisplayDef::Normal,
                    shake: None,
                    wave: None,
                }),
                ("battle_narration".into(), TextAnimationPresetDef {
                    display: TextDisplayDef::Normal,
                    shake: Some(TextShakeDef {
                        intensity: 1.0,
                        mode: TextShakeModeDef::RandomSingle {
                            interval_seconds: 0.08,
                            chance: 0.45,
                            duration_seconds: 0.035,
                        },
                    }),
                    wave: None,
                }),
                ("flowey_evil".into(), TextAnimationPresetDef {
                    display: TextDisplayDef::Normal,
                    shake: Some(TextShakeDef {
                        intensity: 3.0,
                        mode: TextShakeModeDef::Continuous,
                    }),
                    wave: None,
                }),
                ("napstablook".into(), TextAnimationPresetDef {
                    display: TextDisplayDef::Floating {
                        spawn_area: RectDef { x: 0.0, y: 0.0, width: 280.0, height: 200.0 },
                        linger_seconds: 1.5,
                    },
                    shake: None,
                    wave: None,
                }),
                ("mad_dummy".into(), TextAnimationPresetDef {
                    display: TextDisplayDef::Floating {
                        spawn_area: RectDef { x: 0.0, y: 0.0, width: 280.0, height: 200.0 },
                        linger_seconds: 0.8,
                    },
                    shake: None,
                    wave: Some(TextWaveDef {
                        amplitude: 1.4,
                        frequency: 10.0,
                        orbit_angle_per_char_deg: Some(30.0),
                    }),
                }),
                ("heat_waver".into(), TextAnimationPresetDef {
                    display: TextDisplayDef::Normal,
                    shake: None,
                    wave: Some(TextWaveDef {
                        amplitude: 2.0,
                        frequency: 6.0,
                        ..Default::default()
                    }),
                }),
            ]
            .into_iter()
            .collect(),
        },
    }
}
