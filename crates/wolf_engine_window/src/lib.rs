pub fn init() -> WindowContextBuilder {
    todo!()
}

pub struct WindowContextBuilder {}

impl WindowContextBuilder {
    pub fn with_title(mut self, title: &str) -> Self {
        self
    }

    pub fn with_size(mut self, size: (u32, u32)) -> Self {
        self
    }

    pub fn resizable(mut self, is_resizable: bool) -> Self {
        self
    }

    pub fn build(self) -> WindowContext {
        todo!()
    }
}

pub struct WindowContext {}

impl WindowContext {
    pub fn run<F: FnMut(WindowEvent, Window)>(mut self, event_handler: F) {}
}

