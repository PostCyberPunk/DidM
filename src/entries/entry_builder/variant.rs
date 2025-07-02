use anyhow::Result;
use std::marker::PhantomData;
use std::path::PathBuf;

use log::warn;

use super::types::BuildStrategy;
use super::{EntryBuilder, EntryBuilderCtx};

pub struct VariantBuilder;
impl BuildStrategy for VariantBuilder {}
impl VariantBuilder {
    pub fn create<'a>(
        ctx: &'a EntryBuilderCtx<'a>,
        source: PathBuf,
    ) -> Result<EntryBuilder<'a, Self>> {
        let filename = source
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_prefix("didm_va_")
            .unwrap();
        let target = source.parent().unwrap().parent().unwrap().join(filename);
        let relative_path = match target.strip_prefix(ctx.source_root.as_path()) {
            Ok(p) => p.to_path_buf(),
            Err(e) => {
                warn!("Invalid entry path: {}", e);
                return Err(e.into());
            }
        };
        Ok(EntryBuilder {
            source,
            target,
            ctx,
            relative_path: Some(relative_path),
            _marker: PhantomData,
        })
    }
}
