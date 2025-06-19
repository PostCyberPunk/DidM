use thiserror::Error;
#[derive(Error, Debug)]
pub enum BackupError {
    #[error("Failed to initialize backuper")]
    InitializeFailed,
    #[error("An backup already exists: {0}")]
    BackupExsisted(String),
    #[error("Failed to backup :{0}")]
    Failed(String),
    //TODO:! this could be avoid with right abstraction model
    #[error("BUG:Calling normal entry on backup_other")]
    BugWrongType,
}
