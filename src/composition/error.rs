use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlanError {
    #[error("Plan not found.")]
    PlanNotFound,

    #[error("Sketch `{0}` not found.")]
    SketchNotFound(String),

    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    #[error("File operation failed: {0}")]
    FileOpFailed(String),

    #[error("Environment variable error: {0}")]
    EnvError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
