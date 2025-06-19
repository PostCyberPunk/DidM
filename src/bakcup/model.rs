use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::Local;

use crate::{
    log::Logger,
    utils::{PathExtension, ResolvedPath},
};

use super::error;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BackupState {
    Ok,
    Skip,
    Symlink,
    Backuped,
}
pub struct BackupRoot {
    pub base_dir: PathBuf,
    pub is_dryrun: bool,
}
impl BackupRoot {
    pub fn new(base_path: &ResolvedPath, comp_name: String, is_dryrun: bool) -> Result<Self> {
        //Make sure we can write at the base path
        base_path
            .get()
            .check_dir()
            .and_then(|_| base_path.get().check_permission())
            .with_context(|| error::BackupError::InitializeFailed)?;
        //TODO: we can get data by meta data,we can have a better name
        let now = Local::now().format("%Y_%m_%d_%H_%M_%S").to_string();
        let base_dir = base_path
            .get()
            .join(".didm_backup")
            .join(format!("composition_{}-{}", comp_name, now));
        Ok(Self {
            base_dir,
            is_dryrun,
        })
    }
    pub fn has_bakcup(self, logger: &Logger) {
        if self.base_dir.exists() {
            logger.warn(&format!("Backup created at :{}", self.base_dir.display()));
        }
    }
}
