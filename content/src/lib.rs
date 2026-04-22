//! Vessel content guest for `undertale_preset`.
//!
//! `undertale_preset` 的 Vessel 内容 guest。

use anyhow::Result;
use souprune_vessel::prelude::*;

mod static_assets;

vessel_guest! {
    fn build(reg: &mut Registry) -> Result<()> {
        static_assets::emit_all(&mut reg)?;
        Ok(())
    }
}
