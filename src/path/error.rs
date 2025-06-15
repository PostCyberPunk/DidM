use thiserror::Error;
#[derive(Debug, Error)]
pub enum PathError {
    #[error("Environment variable `{0}` is missing")]
    EnvVarMissing(String),

    #[error("Failed to create parent directory: {0}")]
    CreateDirFailed(String),

    #[error("File {0} already existed in {1}")]
    FileExists(String, String),

    #[error("Permission denied: {0}")]
    NoPermission(String),

    #[error("Path is not a directory: {0}")]
    NotDir(String),

    #[error("Path is not a file: {0}")]
    NotFile(String),

    #[error("Failed to resolve path")]
    ResolveFailed,
}
