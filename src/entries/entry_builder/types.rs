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
    fn deal_exist(builder: EntryBuilder<'_, Self>) -> Result<(Entry, CollectResult)> {
        match builder.do_backup() {
            BackupState::Skip(e) => Ok((builder.into_entry(), CollectResult::SkipWithError(e))),
            _ => Ok((builder.into_entry(), CollectResult::Backuped)),
        }
    }
}

pub enum CollectResult {
    Ok,
    Skip,
    Backuped,
    SkipWithError(String),
}
