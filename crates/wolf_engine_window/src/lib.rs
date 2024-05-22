pub type WindowSystem = ((), Context);

pub struct WindowSettings {}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {}
    }
}

pub struct Context {}

impl Context {
    pub fn create_window(&self, settings: WindowSettings) -> Window {
        Window {}
    }
}

pub fn init() -> WindowSystem {
    ((), Context {})
}
