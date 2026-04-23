//! Bootstrapped code asset for `view/structures/view_box.sdf.ron`.
//!
//! `view/structures/view_box.sdf.ron` 的 bootstrap 代码资源。

use anyhow::Result;
use souprune_schema::view::*;
use souprune_vessel::prelude::*;

/// Emit this bootstrapped asset.
///
/// 生成当前 bootstrap 资源。
pub fn emit(reg: &mut Registry) -> Result<()> {
    reg.emit_auto(file!(), &asset())?;
    Ok(())
}

/// Build the typed asset value.
///
/// 构建该资源的类型化值。
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
