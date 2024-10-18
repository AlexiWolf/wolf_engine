/// A general-purpose error for the Window system.
#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum WindowError {
    #[error("Failed to initialize the window system")]
    InitError(String),
    #[error("Operation is unsupported by either the window system, or the current OS.")]
    UnsupportedError(String),
}
