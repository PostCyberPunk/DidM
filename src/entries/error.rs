use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EntryApplyError {
    #[error("Failed to Create symlinka at: {0}\nReason: {1}")]
    FailToCreateLink(PathBuf, String),
    #[error("Copy directory is not supported now,Consider create a `Sketch` instead")]
    CantCopyFolder,
    // #[error("Failed to copy folder at: {0}\nReason: {1}")]
    // FailToCopyFolder(PathBuf, String),
    // #[error("Failed to overwrite target at : {0}")]
    // FailToOverwriteTarget(PathBuf),
}
