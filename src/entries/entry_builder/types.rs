use crate::entries::Entry;
use crate::utils::ResolvedPath;

use crate::bakcup::{BackupManager, BackupState};

use super::EntryBuilder;
use anyhow::Result;

pub struct EntryBuilderCtx<'a> {
    pub backup_manager: Option<&'a BackupManager>,
    pub source_root: ResolvedPath,
    pub target_root: ResolvedPath,
    pub overwrite: bool,
}
pub trait BuildStrategy: Sized {
    fn deal_exist(builder: EntryBuilder<'_, Self>) -> (Entry, CollectResult) {
        match builder.do_backup() {
            BackupState::Skip => (builder.into_entry(), CollectResult::Skip),
            _ => (builder.into_entry(), CollectResult::Ok),
        }
    }
}

pub enum CollectResult {
    Ok,
    Skip,
    Backup,
}
