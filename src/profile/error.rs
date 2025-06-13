use std::path::PathBuf;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenerateEntriesError {
    #[error("Failed to add ignore rule: {0}")]
    OverrideError(#[from] ignore::Error),

    #[error("Failed to acquire lock: {0}")]
    LockError(#[source] std::sync::PoisonError<Mutex<Vec<PathBuf>>>),

    #[error("Failed to unwrap Arc for entries")]
    ArcUnwrapError,
}
#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("Failed to generate entries: {0}")]
    GenerateEntriesError(#[from] GenerateEntriesError),
}
