//! Bootstrapped code asset for `view/structures/hp_bar.sdf.ron`.
//!
//! `view/structures/hp_bar.sdf.ron` 的 bootstrap 代码资产。

use anyhow::Result;
use souprune_schema::val::*;
use souprune_schema::view::*;
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
pub fn asset() -> SdfStructureAsset {
    SdfStructure {
        layer_count: 2,
        root: SdfLayerDef {
            name: "HPBarBackground".into(),
            sdf_type: SdfShapeKind::Inner,
            color_source: SdfColorSource::Custom((
                Val::Static(1.0),
                Val::Static(0.0),
                Val::Static(0.0),
                Val::Static(1.0),
            )),
            z_offset: 0.0,
            is_filler: false,
            children: vec![SdfLayerDef {
                name: "HPBarForeground".into(),
                sdf_type: SdfShapeKind::Inner,
                color_source: SdfColorSource::Custom((
                    Val::Static(1.0),
                    Val::Static(1.0),
                    Val::Static(0.0),
                    Val::Static(1.0),
                )),
                z_offset: 0.1,
                is_filler: true,
                children: vec![],
            }],
        },
    }
}
