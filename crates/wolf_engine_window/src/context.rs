use std::sync::Arc;

use crate::{settings::WindowSettings, Window, WindowBackendAdapter};

pub struct Context {
    backend_adapter: Arc<Box<dyn WindowBackendAdapter>>,
}

impl Context {
    fn new(backend_adapter: Box<dyn WindowBackendAdapter>) -> Self {
        Self {
            backend_adapter: Arc::new(backend_adapter),
        }
    }

    pub fn create_window(&self, settings: WindowSettings) -> Window {
        self.backend_adapter.create_window(settings)
    }

    fn backend_adapter(&self) -> Arc<Box<dyn WindowBackendAdapter>> {
        self.backend_adapter.clone()
    }
}
