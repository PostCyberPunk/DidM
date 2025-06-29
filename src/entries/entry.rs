use std::path::PathBuf;

use crate::bakcup::BackupState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    // pub overwrite_existed: bool,
    // pub backup_state: BackupState,
}

impl Entry {
    pub fn new(source_path: PathBuf, target_path: PathBuf) -> Self {
        Entry {
            source_path,
            target_path,
            // overwrite_existed,
            // backup_state,
        }
    }
}
