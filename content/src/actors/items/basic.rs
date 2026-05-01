//! Code representation of `actors/items/basic.items.ron`.
//!
//! `actors/items/basic.items.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::enemy::*;
use souprune_schema::item::*;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> ItemListAsset {
    ItemListAsset(vec![
        Item {
            id: "monster_candy".into(),
            locale: LocaleInfo {
                name: "MONSTER_CANDY".into(),
                file: "items".into(),
            },
            description: "Has a distinct, non-licorice flavor.".into(),
            mortar: Some("items/monster_candy.mortar".into()),
            item_type: ItemType::Food {
                consumable: true,
                effects: vec![
                    ItemEffect::Heal { amount: 10 },
                    ItemEffect::PlayAudio {
                        clip_path: "assets/audios/sfx/snd_heal_c.wav".into(),
                    },
                ],
            },
        },
        Item {
            id: "test_parent".into(),
            locale: LocaleInfo {
                name: "TEST_PARENT".into(),
                file: "items".into(),
            },
            description: "Comes with everything you need.".into(),
            mortar: None,
            item_type: ItemType::Food {
                consumable: true,
                effects: vec![
                    ItemEffect::Heal { amount: 4 },
                    ItemEffect::SpawnChildItem {
                        item_id: "dry_noodles".into(),
                    },
                ],
            },
        },
        Item {
            id: "dry_noodles".into(),
            locale: LocaleInfo {
                name: "DRY_NOODLES".into(),
                file: "items".into(),
            },
            description: "You don't have the water to boil them.".into(),
            mortar: None,
            item_type: ItemType::Food {
                consumable: true,
                effects: vec![ItemEffect::Heal { amount: 2 }],
            },
        },
        Item {
            id: "tough_glove".into(),
            locale: LocaleInfo {
                name: "TOUGH_GLOVE".into(),
                file: "items".into(),
            },
            description: "A worn pink leather glove.".into(),
            mortar: Some("items/tough_glove.mortar".into()),
            item_type: ItemType::Weapon {
                damage: 5,
                on_hit_effects: vec![ItemEffect::PlayAudio {
                    clip_path: "assets/audios/sfx/slice.wav".into(),
                }],
            },
        },
        Item {
            id: "faded_ribbon".into(),
            locale: LocaleInfo {
                name: "FADED_RIBBON".into(),
                file: "items".into(),
            },
            description: "If you're cuter, monsters won't hit you as hard.".into(),
            mortar: None,
            item_type: ItemType::Armor { defense: 3 },
        },
        Item {
            id: "stick".into(),
            locale: LocaleInfo {
                name: "STICK".into(),
                file: "items".into(),
            },
            description: "Its bark is worse than its bite.".into(),
            mortar: None,
            item_type: ItemType::Weapon {
                damage: 0,
                on_hit_effects: vec![],
            },
        },
        Item {
            id: "bandage".into(),
            locale: LocaleInfo {
                name: "BANDAGE".into(),
                file: "items".into(),
            },
            description: "It has already been used several times.".into(),
            mortar: None,
            item_type: ItemType::Armor { defense: 0 },
        },
        Item {
            id: "cell_phone".into(),
            locale: LocaleInfo {
                name: "CELL_PHONE".into(),
                file: "items".into(),
            },
            description: "An old cell phone.".into(),
            mortar: Some("items/cell_phone.mortar".into()),
            item_type: ItemType::KeyItem,
        },
    ])
}
