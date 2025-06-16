use thiserror::Error;
#[derive(Debug, Error)]
pub enum PathError {
    #[error("Unresolved envrionment varible: `{0}`")]
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

    #[error("Failed to resolve path:{0}")]
    ResolveFailed(String),

    #[error("Path is root, no parent")]
    NoParent,

    #[error("Failed to resolve symlink: {0}")]
    UnresolvedSymlink(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
