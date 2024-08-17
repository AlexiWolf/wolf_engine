use std::sync::Arc;

/// A general-purpose error for the Window system.
#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum WindowError {
    #[error("Operation is unsupported by the OS")]
    OsError(#[from] OsError),
    #[error("Operation is unsupported by the window system.")]
    UnsupportedError(#[from] UnsupportedError),
}

/// Indicates an operation is unsupported by the OS.
#[derive(thiserror::Error, Debug, Clone)]
#[error(transparent)]
pub struct OsError {
    error: Arc<anyhow::Error>,
}

impl From<anyhow::Error> for OsError {
    fn from(error: anyhow::Error) -> Self {
        Self {
            error: Arc::new(error),
        }
    }
}

impl PartialEq for OsError {
    fn eq(&self, other: &Self) -> bool {
        self.error.to_string() == other.error.to_string()
    }
}

/// Indicates an operation is unsupported by specific back-end system.
#[derive(thiserror::Error, Debug, Clone)]
#[error(transparent)]
pub struct UnsupportedError {
    error: Arc<anyhow::Error>,
}

impl From<anyhow::Error> for UnsupportedError {
    fn from(error: anyhow::Error) -> Self {
        Self {
            error: Arc::new(error),
        }
    }
}

impl PartialEq for UnsupportedError {
    fn eq(&self, other: &Self) -> bool {
        self.error.to_string() == other.error.to_string()
    }
}
