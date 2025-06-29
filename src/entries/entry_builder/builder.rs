use crate::bakcup::BackupState;
use crate::entries::Entry;
use anyhow::Result;
use std::marker::PhantomData;
use std::path::PathBuf;

use super::EntryBuilderCtx;
use super::types::BuildStrategy;

pub struct EntryBuilder<'a, S: BuildStrategy> {
    pub source: PathBuf,
    pub target: PathBuf,
    pub relative_path: Option<PathBuf>,
    pub ctx: &'a EntryBuilderCtx<'a>,
    pub _marker: PhantomData<S>,
}

impl<'a, S: BuildStrategy> EntryBuilder<'a, S> {
    pub async fn build(mut self) -> Result<Entry> {
        self.do_join_relative().do_rename();
        let entry = Entry::new(self.source, self.target);
        Ok(entry)
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
                .bakcup_async(&self.target, self.relative_path.as_deref())
                .await
            {
                Ok(s) => s,
                Err(e) => BackupState::Skip,
            }
        } else {
            BackupState::Ok
        }
    }
}
