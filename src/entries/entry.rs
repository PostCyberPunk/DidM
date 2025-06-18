use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
// TODO: this is not apply to entry
pub enum BackupState {
    Ok,
    Skip,
    Backuped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub source_path: PathBuf,
    pub target_path: PathBuf,
    pub overwrite_existed: bool,
    pub bakcup_state: BackupState,
}

impl Entry {
    pub fn new(source_path: PathBuf, target_path: PathBuf, overwrite_existed: bool) -> Self {
        Entry {
            source_path,
            target_path,
            overwrite_existed,
            bakcup_state: BackupState::Ok,
        }
    }
}
