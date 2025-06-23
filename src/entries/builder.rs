use crate::{
    bakcup::{BackupManager, BackupState},
    utils::ResolvedPath,
};

use super::{Entry, SouceType};
use anyhow::Result;
use std::path::PathBuf;

pub struct EntryBuilderCtx<'a> {
    pub backup_manager: Option<&'a BackupManager>,
    pub source_root: ResolvedPath,
    pub target_root: ResolvedPath,
    pub overwrite: bool,
}

pub struct EntryBuilder<'a> {
    source: PathBuf,
    target: PathBuf,
    relative_path: Option<PathBuf>,
    ctx: &'a EntryBuilderCtx<'a>,
    source_type: SouceType,
    overwrite: Option<bool>,
}

impl<'a> EntryBuilder<'a> {
    pub fn new(
        source: impl Into<PathBuf>,
        target: impl Into<PathBuf>,
        config: &'a EntryBuilderCtx<'a>,
    ) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            relative_path: None,
            ctx: config,
            source_type: SouceType::Normal,
            overwrite: None,
        }
    }

    pub async fn build(mut self) -> Result<Entry> {
        self.do_join_relative().do_rename();

        let bakcup_state = self.do_backup().await;
        let overwrite = self.get_overwrite();

        let entry = Entry::new(self.source, self.target, overwrite, bakcup_state);
        Ok(entry)
    }

    pub fn source_type(mut self, s: SouceType) -> Self {
        self.source_type = s;
        self
    }
    pub fn relative_path(mut self, path: PathBuf) -> Self {
        self.relative_path = Some(path);
        self
    }
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = Some(overwrite);
        self
    }

    fn get_overwrite(&self) -> bool {
        self.overwrite.unwrap_or(self.ctx.overwrite)
    }
    fn do_join_relative(&mut self) -> &mut Self {
        if let Some(path) = &self.relative_path {
            self.target = self.ctx.target_root.as_path().join(path);
        }
        self
    }
    fn do_rename(&mut self) -> &mut Self {
        let target = self.target.to_str().unwrap();
        if target.contains("dot-") {
            self.target = PathBuf::from(target.replace("dot-", "."));
        };
        self
    }
    async fn do_backup(&self) -> BackupState {
        if let Some(bm) = self.ctx.backup_manager {
            match bm
                .bakcup_async(
                    &self.target,
                    self.relative_path.as_deref(),
                    self.source_type,
                )
                .await
            {
                Ok(s) => s,
                Err(_) => BackupState::Skip,
            }
        } else {
            BackupState::Ok
        }
    }
}
