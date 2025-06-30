use crate::bakcup::BackupState;
use crate::entries::Entry;
use std::marker::PhantomData;
use std::path::PathBuf;

use super::EntryBuilderCtx;
use super::types::{BuildStrategy, CollectResult};

pub struct EntryBuilder<'a, S: BuildStrategy> {
    pub source: PathBuf,
    pub target: PathBuf,
    pub relative_path: Option<PathBuf>,
    pub ctx: &'a EntryBuilderCtx<'a>,
    pub _marker: PhantomData<S>,
}

impl<'a, S: BuildStrategy> EntryBuilder<'a, S> {
    pub fn build(mut self) -> (Entry, CollectResult) {
        self.do_join_relative().do_rename();
        match (self.ctx.overwrite, self.target.exists()) {
            (true, true) => S::deal_exist(self),
            (false, true) => (self.into_entry(), CollectResult::Skip),
            (_, _) => (self.into_entry(), CollectResult::Ok),
        }
    }
    pub fn into_entry(self) -> Entry {
        Entry::new(self.source, self.target)
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
    pub fn do_backup(&self) -> BackupState {
        if let Some(bm) = self.ctx.backup_manager {
            match bm.bakcup(&self.target, self.relative_path.as_deref()) {
                Ok(s) => s,
                Err(e) => BackupState::Skip(e.to_string()),
            }
        } else {
            BackupState::Ok
        }
    }
}
