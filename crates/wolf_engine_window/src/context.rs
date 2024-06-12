use std::sync::Arc;

use crate::{backend::WindowBackend, settings::WindowSettings, Window};

pub struct WindowContext {
    backend_adapter: Arc<Box<dyn WindowBackend>>,
}

impl WindowContext {
    pub(crate) fn new(backend_adapter: Box<dyn WindowBackend>) -> Self {
        Self {
            backend_adapter: Arc::new(backend_adapter),
        }
    }

    pub fn create_window(&self, settings: WindowSettings) -> Window {
        self.backend_adapter.create_window(settings)
    }

    pub(crate) fn backend_adapter(&self) -> Arc<Box<dyn WindowBackend>> {
        self.backend_adapter.clone()
    }
}
