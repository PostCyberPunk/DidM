use anyhow::Result;
use std::marker::PhantomData;

use crate::utils::PathResolver;

use super::types::BuildStrategy;
use super::{EntryBuilder, EntryBuilderCtx};

pub struct ExtraBuilder;
impl BuildStrategy for ExtraBuilder {}
impl ExtraBuilder {
    pub fn create<'a>(
        ctx: &'a EntryBuilderCtx<'a>,
        source_path: &str,
        target_path: &str,
    ) -> Result<EntryBuilder<'a, Self>> {
        let source = PathResolver::resolve_from_with_ctx(
            &ctx.source_root,
            source_path,
            "extra entry",
            true,
        )?
        .into_pathbuf();
        let target = PathResolver::resolve_from_with_ctx(
            &ctx.target_root,
            target_path,
            "extra entry target",
            false,
        )?
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
