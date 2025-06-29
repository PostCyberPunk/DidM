use std::path::PathBuf;

use crate::bakcup::BackupState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    pub overwrite_existed: bool,
    pub bakcup_state: BackupState,
}

impl Entry {
    pub fn new(source_path: PathBuf, mut target_path: PathBuf, overwrite_existed: bool) -> Self {
        Entry {
            source_path,
            target_path,
            overwrite_existed,
            bakcup_state: BackupState::Ok,
        }
    }
}
