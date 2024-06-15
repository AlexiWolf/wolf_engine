#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowError {
    Closed,
    Unsupported,
}
