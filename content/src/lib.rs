//! Cauld-ron content guest for `undertale_preset`.
//!
//! `undertale_preset` 的 Cauld-ron 内容 guest。

use anyhow::Result;
use souprune_cauld_ron::prelude::*;

mod support;
include!(concat!(env!("OUT_DIR"), "/cauld_ron_content_registry.rs"));

cauld_ron_guest! {
    fn build(reg: &mut Registry) -> Result<()> {
        emit_all(&mut reg)
    }
}
