//! Code representation of `view/structures/hp_bar.sdf.ron`.
//!
//! `view/structures/hp_bar.sdf.ron` 的代码表示。

use anyhow::Result;
use souprune_schema::view::*;
use souprune_cauld_ron::prelude::*;

pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

pub fn asset() -> SdfStructureAsset {
    SdfStructure {
        layer_count: 2,
        root: SdfLayerDef {
            name: "HPBarBackground".into(),
            sdf_type: SdfShapeKind::Inner,
            color_source: SdfColorSource::Custom(red()),
            z_offset: 0.0,
            is_filler: false,
            children: vec![SdfLayerDef {
                name: "HPBarForeground".into(),
                sdf_type: SdfShapeKind::Inner,
                color_source: SdfColorSource::Custom(color(1.0, 1.0, 0.0, 1.0)),
                z_offset: 0.1,
                is_filler: true,
                children: vec![],
            }],
        },
    }
}
