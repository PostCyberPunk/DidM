use anyhow::Result;
use std::marker::PhantomData;
use std::path::PathBuf;

use crate::utils::PathResolver;

use super::types::BuildStrategy;
use super::{EntryBuilder, EntryBuilderCtx};

pub struct SameSourceBuilder;

impl BuildStrategy for SameSourceBuilder {}
impl SameSourceBuilder {
    fn create<'a>(
        ctx: &'a EntryBuilderCtx<'a>,
        source: PathBuf,
        target_path: &str,
        hint: &str,
    ) -> Result<EntryBuilder<'a, Self>> {
        let target =
            PathResolver::resolve_from_with_ctx(&ctx.target_root, target_path, hint, false)?
                .into_pathbuf();
        Ok(EntryBuilder {
            source,
            target,
            ctx,
            relative_path: None,
            _marker: PhantomData,
        })
    }
}
