/// A general-purpose error for the Window system.
#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum WindowError {
    /// Indicates the window system could not be initialized.
    #[error("Failed to initialize the window system")]
    InitError(String),
    /// Indicates the operation is not supposrted by either the window system, or the current OS.
    #[error("Operation is unsupported by either the window system, or the current OS.")]
    UnsupportedError(String),
}
