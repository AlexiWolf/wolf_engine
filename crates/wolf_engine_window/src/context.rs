use std::rc::Rc;

use crate::{backend::WindowBackend, settings::WindowSettings, Window};

pub struct WindowContext {
    backend_adapter: Rc<Box<dyn WindowBackend>>,
}

impl WindowContext {
    pub(crate) fn new(backend_adapter: Box<dyn WindowBackend>) -> Self {
        Self {
            backend_adapter: Rc::new(backend_adapter),
        }
    }

    pub fn create_window(&self, settings: WindowSettings) -> Window {
        self.backend_adapter.create_window(settings)
    }

    pub(crate) fn backend_adapter(&self) -> Rc<Box<dyn WindowBackend>> {
        self.backend_adapter.clone()
    }
}
