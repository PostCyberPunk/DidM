use crate::{
    bakcup::{BackupManager, BackupState},
    model::sketch::Mode,
};

use super::{Entry, SouceType};
use anyhow::Result;
use std::path::PathBuf;

pub struct EntryCtx<'a> {
    pub mode: Mode,
    pub overwrite: bool,
    pub backup_manager: Option<&'a BackupManager>,
}

pub struct EntryBuilder<'a> {
    source: PathBuf,
    target: PathBuf,
    source_type: SouceType,
    relative_path: Option<PathBuf>,
    ctx: EntryCtx<'a>,
}

impl<'a> EntryBuilder<'a> {
    pub fn new(
        source: impl Into<PathBuf>,
        target: impl Into<PathBuf>,
        config: EntryCtx<'a>,
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

    pub async fn build(mut self) -> Result<Entry> {
        if let Some(path) = self.relative_path {
            self.target = self.target.join(path);
        }

        let mut entry = Entry::new(self.source, self.target, self.ctx.overwrite);

        if let Some(bm) = self.ctx.backup_manager {
            entry.bakcup_state = match self.source_type {
                SouceType::Normal => bm.backup_normal(&entry.target_path, &entry.source_path),
                SouceType::Extra => bm.backup_other(&entry.target_path, self.source_type),
                _ => Ok(BackupState::Skip),
            }?;
        }

        Ok(entry)
    }
}
