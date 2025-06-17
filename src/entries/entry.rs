use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BackupState {
    Ok,
    Skip,
    Backuped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    source_path: PathBuf,
    target_path: PathBuf,
    overwrite_existed: bool,
    bakcup_state: BackupState,
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
