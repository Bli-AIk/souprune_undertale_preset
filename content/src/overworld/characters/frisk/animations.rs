//! Code representation of `overworld/characters/frisk/animations.animation_config.ron`.
//!
//! `overworld/characters/frisk/animations.animation_config.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::character::*;
use souprune_vessel::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> AnimationConfigAsset {
    AnimationConfigAsset {
        sprite_source: "overworld".into(),
        default_frame_duration: 0.15,
        default_looping: true,
        states: vec![
            (
                "Run".into(),
                StateAnimationMapping::Directional {
                    up: AnimationEntry::Path("characters/frisk/run/up".into()),
                    down: AnimationEntry::Path("characters/frisk/run/down".into()),
                    left: AnimationEntry::Path("characters/frisk/run/side".into()),
                    right: AnimationEntry::Full {
                        path: "characters/frisk/run/side".into(),
                        flip_x: true,
                        flip_y: false,
                        frame_duration: None,
                        looping: None,
                    },
                },
            ),
            (
                "Walk".into(),
                StateAnimationMapping::Directional {
                    up: AnimationEntry::Path("characters/frisk/walk/up".into()),
                    down: AnimationEntry::Path("characters/frisk/walk/down".into()),
                    left: AnimationEntry::Path("characters/frisk/walk/side".into()),
                    right: AnimationEntry::Full {
                        path: "characters/frisk/walk/side".into(),
                        flip_x: true,
                        flip_y: false,
                        frame_duration: None,
                        looping: None,
                    },
                },
            ),
            (
                "Idle".into(),
                StateAnimationMapping::Directional {
                    up: AnimationEntry::Path("characters/frisk/walk/up/0.png".into()),
                    down: AnimationEntry::Path("characters/frisk/walk/down/0.png".into()),
                    left: AnimationEntry::Path("characters/frisk/walk/side/0.png".into()),
                    right: AnimationEntry::Full {
                        path: "characters/frisk/walk/side/0.png".into(),
                        flip_x: true,
                        flip_y: false,
                        frame_duration: None,
                        looping: None,
                    },
                },
            ),
        ]
        .into_iter()
        .collect(),
    }
}
