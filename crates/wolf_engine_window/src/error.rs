/// A general-purpose error for the Window system.
#[derive(thiserror::Error, Debug)]
pub enum WindowError {
    #[error("Operation is unsupported by the OS")]
    OsError(#[from] OsError),
    #[error("Operation is unsupported by the window system.")]
    UnsupportedError(#[from] UnsupportedError),
}

/// Indicates an operation is unsupported by the OS.
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct OsError {
    #[from]
    error: anyhow::Error,
}

/// Indicates an operation is unsupported by specific back-end system.
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct UnsupportedError {
    #[from]
    error: anyhow::Error,
}
