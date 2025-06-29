use anyhow::Result;
use std::marker::PhantomData;
use std::path::PathBuf;

use log::warn;

use super::types::BuildStrategy;
use super::{EntryBuilder, EntryBuilderCtx};

pub struct NormalBuilder;
impl BuildStrategy for NormalBuilder {}
impl NormalBuilder {
    fn create<'a>(ctx: &'a EntryBuilderCtx<'a>, source: PathBuf) -> Result<EntryBuilder<'a, Self>> {
        let relative_path = match source.strip_prefix(ctx.source_root.as_path()) {
            Ok(p) => p.to_path_buf(),
            Err(e) => {
                warn!("Invalid entry path: {}", e);
                return Err(e.into());
            }
        };
        Ok(EntryBuilder {
            source,
            target: ctx.target_root.as_path().join(relative_path.clone()),
            ctx,
            relative_path: Some(relative_path),
            _marker: PhantomData,
        })
    }
}
