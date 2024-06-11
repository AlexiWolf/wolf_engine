#[derive(Copy, Clone, PartialEq, Eq)]
pub struct WindowSettings {
    pub title: &'static str,
    pub size: (u32, u32),
}

impl WindowSettings {
    pub fn with_title<T: Into<&'static str>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_size<T: Into<(u32, u32)>>(mut self, size: T) -> Self {
        self.size = size.into();
        self
    }
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "untitled",
            size: (800, 600),
        }
    }
}
