//! Vessel content guest for `undertale_preset`.
//!
//! `undertale_preset` 的 Vessel 内容 guest。

use anyhow::Result;
use souprune_vessel::prelude::*;

vessel_guest! {
    fn build(reg: &mut Registry) -> Result<()> {
        let _ = reg;
        Ok(())
    }
}
