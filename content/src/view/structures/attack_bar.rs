//! Code representation of `view/structures/attack_bar.sdf.ron`.
//!
//! `view/structures/attack_bar.sdf.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::view::*;
use souprune_vessel::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> SdfStructureAsset {
    SdfStructure {
        layer_count: 2,
        root: SdfLayerDef {
            name: "AttackBarOuter".into(),
            sdf_type: SdfShapeKind::Outer,
            color_source: SdfColorSource::FactToggle {
                key: "fight:bar_flash_on".into(),
                on: white(),
                off: color(0.0, 0.0, 0.0, 1.0),
            },
            z_offset: 0.0,
            is_filler: false,
            children: vec![SdfLayerDef {
                name: "AttackBarInner".into(),
                sdf_type: SdfShapeKind::Inner,
                color_source: SdfColorSource::FactToggle {
                    key: "fight:bar_flash_on".into(),
                    on: color(0.0, 0.0, 0.0, 1.0),
                    off: white(),
                },
                z_offset: 1.0,
                is_filler: true,
                children: vec![],
            }],
        },
    }
}
