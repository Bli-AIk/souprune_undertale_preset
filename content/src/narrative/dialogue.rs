//! Bootstrapped code asset for `narrative/dialogue.ron`.
//!
//! `narrative/dialogue.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::dialogue::*;
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
    }
}
