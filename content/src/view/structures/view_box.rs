//! Code representation of `view/structures/view_box.sdf.ron`.
//!
//! `view/structures/view_box.sdf.ron` 的代码表示。

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
            name: "UIBoxBorder".into(),
            sdf_type: SdfShapeKind::Outer,
            color_source: SdfColorSource::White,
            z_offset: 5.0,
            is_filler: false,
            children: vec![SdfLayerDef {
                name: "UIBoxFiller".into(),
                sdf_type: SdfShapeKind::Inner,
                color_source: SdfColorSource::FillColor,
                z_offset: 0.1,
                is_filler: true,
                children: vec![],
            }],
        },
    }
}
