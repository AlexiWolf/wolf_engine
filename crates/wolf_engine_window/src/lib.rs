pub type WindowSystem = ((), ());

pub struct WindowSettings {}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {}
    }
}

pub fn init() -> WindowSystem {
    ((), ())
}
