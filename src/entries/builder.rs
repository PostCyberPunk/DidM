use crate::bakcup::{BackupManager, BackupState};

use super::{Entry, SouceType};
use anyhow::Result;
use std::path::PathBuf;

pub struct EntryBuilderCtx<'a> {
    pub backup_manager: Option<&'a BackupManager>,
    pub overwrite: bool,
}

pub struct EntryBuilder<'a> {
    source: PathBuf,
    target: PathBuf,
    relative_path: Option<PathBuf>,
    ctx: EntryBuilderCtx<'a>,
    source_type: SouceType,
    overwrite: Option<bool>,
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
            relative_path: None,
            ctx: config,
            source_type: SouceType::Normal,
            overwrite: None,
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
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = Some(overwrite);
        self
    }

    pub async fn build(self) -> Result<Entry> {
        let mut target_path = self.target;
        if let Some(path) = &self.relative_path {
            target_path = target_path.join(path);
        }

        //Renamer
        //TODO: does not feel good about this
        if target_path.to_str().unwrap().contains("dot-") {
            target_path = PathBuf::from(target_path.to_str().unwrap().replace("dot-", "."));
        };

        let mut entry = Entry::new(
            self.source,
            target_path,
            self.overwrite.unwrap_or(self.ctx.overwrite),
        );

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
