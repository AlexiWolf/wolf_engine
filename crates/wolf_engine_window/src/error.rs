/// A general-purpose error for the Window system.
#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum WindowError {
    #[error("Operation is unsupported by the window system.")]
    UnsupportedError(String),
}
