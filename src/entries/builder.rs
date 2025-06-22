use crate::{
    bakcup::{BackupManager, BackupState},
    model::sketch::Mode,
};

use super::{Entry, SouceType};
use anyhow::Result;
use std::path::PathBuf;

pub struct EntryBuilderCtx<'a> {
    pub mode: Mode,
    pub overwrite: bool,
    pub backup_manager: Option<&'a BackupManager>,
}

pub struct EntryBuilder<'a> {
    source: PathBuf,
    target: PathBuf,
    source_type: SouceType,
    relative_path: Option<PathBuf>,
    ctx: EntryBuilderCtx<'a>,
}

impl<'a> EntryBuilder<'a> {
    pub fn new(
        source: impl Into<PathBuf>,
        target: impl Into<PathBuf>,
        config: EntryBuilderCtx<'a>,
    ) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            source_type: SouceType::Normal,
            relative_path: None,
            ctx: config,
        }
    }

    pub fn source_type(mut self, s: SouceType) -> Self {
        self.source_type = s;
        self
    }

    pub fn relative_path(mut self, path: PathBuf) -> Self {
        self.relative_path = Some(path);
        self
    }

    pub async fn build(self) -> Result<Entry> {
        let mut target_path = self.target;
        if let Some(path) = &self.relative_path {
            target_path = target_path.join(path);
        }

        let mut entry = Entry::new(self.source, target_path, self.ctx.overwrite);

        //Bakcuper
        if let Some(bm) = self.ctx.backup_manager {
            entry.bakcup_state = match bm
                .bakcup_async(&entry.target_path, self.relative_path, self.source_type)
                .await
            {
                Ok(s) => s,
                Err(_) => BackupState::Skip,
            };
        }

        Ok(entry)
    }
}
