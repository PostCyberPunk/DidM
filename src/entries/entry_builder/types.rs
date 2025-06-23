use anyhow::Result;
use std::marker::PhantomData;
use std::path::PathBuf;

use async_trait::async_trait;
use log::warn;

use crate::utils::ResolvedPath;

use crate::bakcup::{BackupManager, BackupState};

use super::EntryBuilder;

pub struct EntryBuilderCtx<'a> {
    pub backup_manager: Option<&'a BackupManager>,
    pub source_root: ResolvedPath,
    pub target_root: ResolvedPath,
    pub overwrite: bool,
}
#[async_trait]
pub trait BuildStrategy: Sized {
    // fn builder<'a>(
    //     ctx: &'a EntryBuilderCtx<'a>,
    //     source: PathBuf,
    //     target: PathBuf,
    // ) -> Result<EntryBuilder<'a, Self>>;

    // async fn do_backup(builder: &EntryBuilder<'_, Self>) -> BackupState;
}
pub struct NormalBuilder;
impl BuildStrategy for NormalBuilder {}
impl NormalBuilder {
    fn builder<'a>(
        ctx: &'a EntryBuilderCtx<'a>,
        source: PathBuf,
    ) -> Result<EntryBuilder<'a, Self>> {
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
            source_type: crate::entries::SouceType::Normal,
            relative_path: Some(relative_path),
            _marker: PhantomData,
            overwrite: Some(false),
        })
    }
}
