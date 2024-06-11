#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum WindowEvent {
    CloseRequested { id: Uuid },
    Resized { id: Uuid, width: u32, height: u32 },
}

